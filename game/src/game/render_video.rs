use ffmpeg_next::ffi::{av_seek_frame, AVSEEK_FLAG_ANY, AVSEEK_FLAG_BACKWARD, AV_TIME_BASE};
use sdl2::rect::Rect;
use sdl2::render::Texture;

use std::path::Path;
use std::sync::atomic::AtomicU32;
use std::sync::mpsc::channel;
use std::{sync, thread};

use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context as Scaler, flag::Flags};
//use ffmpeg::util::rational::Rational;
use num_rational::{Rational32, Rational64};

/// Video file renderer with guarantee of rendering frame around `wanted_time_in_second`
/// without using delay
pub struct VideoFileRenderer {
    pub wanted_time_in_second: Rational64,
    last_decoded_timestamp: Option<i64>,
    last_target_ts: Option<i64>,
    stop_thread: sync::Arc<sync::atomic::AtomicBool>,
    timebase: Rational64,
    size: (u32, u32),
    decoded_frame_count: sync::Arc<sync::atomic::AtomicU32>,
    rx: std::sync::mpsc::Receiver<YUVData>,
    infinite: bool,
    duration: Rational64,
}

/// Video frame data
#[derive(Clone)]
struct YUVData {
    timestamp: i64,
    width: u32,
    height: u32,
    y_plane: Vec<u8>,
    y_pitch: usize,
    u_plane: Vec<u8>,
    u_pitch: usize,
    v_plane: Vec<u8>,
    v_pitch: usize,
}

#[macro_export]
macro_rules! create_streaming_iyuv_texture {
    ($texture_creator: expr, $width: expr, $height: expr) => {
        $texture_creator.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::IYUV,
            $width,
            $height,
        )
    };
}

impl VideoFileRenderer {
    /// Creates new VideoFileRenderer from video file path
    pub(crate) fn new(path: &Path, infinite: bool) -> VideoFileRenderer {
        // init FFmpeg
        ffmpeg_next::init().unwrap();

        // init variable used to stop the thread
        let stop_thread = sync::Arc::new(sync::atomic::AtomicBool::new(false));

        // init wanted_time
        let wanted_time_in_second = Rational64::new(0, 1);

        // init stream
        let mut input = input(&path.to_str().expect("Non-unicode character in path"))
            .expect("Failed to open file");
        let video_stream_index = input
            .streams()
            .best(Type::Video)
            .expect("Video stream not found")
            .index();
        let stream = input
            .stream(video_stream_index)
            .expect("illegal stream index");

        // init decoder
        let context_decoder =
            ffmpeg_next::codec::context::Context::from_parameters(stream.parameters().clone())
                .expect("Failed to get decoder");
        let mut video_decoder = context_decoder
            .decoder()
            .video()
            .expect("Failed to get video decoder");
        let size = (video_decoder.width(), video_decoder.height());

        // create channel
        let (tx, rx) = channel();

        // calculate decoded frame buffer limit
        let decoded_frame_buffer_limit = std::cmp::max(
            // frames of 2ms
            Rational32::new(
                stream.avg_frame_rate().numerator(),
                stream.avg_frame_rate().denominator() * 500,
            )
            .to_integer() as u32,
            10,
        );

        let decoded_frame_count = sync::Arc::new(AtomicU32::new(0));

        // get timebase
        // e.g. if timebase is 1/75, pts 1 means 1/75s, pts 2 means 2/75s, ...and more.
        let timebase = Rational64::new(
            stream.time_base().numerator() as i64,
            stream.time_base().denominator() as i64,
        );

        // get video duration
        let duration = Rational64::new(input.duration(), AV_TIME_BASE as i64);

        // spawn thread
        let stop_thread_for_thread = stop_thread.clone();
        let decoded_frame_count_for_thread = decoded_frame_count.clone();
        thread::spawn(move || {
            // create scaler
            let mut scaler = Scaler::get(
                video_decoder.format(),
                video_decoder.width(),
                video_decoder.height(),
                Pixel::YUV420P,
                video_decoder.width(),
                video_decoder.height(),
                Flags::BILINEAR,
            )
            .unwrap();

            // loop for the packets of the video file
            loop {
                for (stream, packet) in input.packets() {
                    if stop_thread_for_thread.load(sync::atomic::Ordering::Relaxed) {
                        return;
                    }
                    if stream.index() == video_stream_index {
                        // send packet
                        let _ = video_decoder.send_packet(&packet);

                        // try to decode packets
                        let processed_frame = Self::process_received_frames(&mut video_decoder);

                        // if decoding success
                        if let Some(decoded_frame) = processed_frame {
                            let mut scaled_frame = ffmpeg_next::frame::Video::empty();
                            scaler.run(&decoded_frame, &mut scaled_frame).unwrap();
                            let data = YUVData {
                                height: scaled_frame.height(),
                                width: scaled_frame.width(),
                                timestamp: decoded_frame.timestamp().unwrap(),
                                y_plane: scaled_frame.data(0).to_vec(),
                                y_pitch: scaled_frame.stride(0),
                                u_plane: scaled_frame.data(1).to_vec(),
                                u_pitch: scaled_frame.stride(1),
                                v_plane: scaled_frame.data(2).to_vec(),
                                v_pitch: scaled_frame.stride(2),
                            };
                            if !stop_thread_for_thread.load(sync::atomic::Ordering::Relaxed) {
                                while decoded_frame_count_for_thread
                                    .load(sync::atomic::Ordering::Relaxed)
                                    >= decoded_frame_buffer_limit
                                {}
                                // Ignore errors
                                let _ = tx.send(data);
                                decoded_frame_count_for_thread
                                    .fetch_add(1, sync::atomic::Ordering::Relaxed);
                            }
                        }
                    }
                }

                if infinite {
                    // Seek the video to the beginning
                    unsafe {
                        av_seek_frame(
                            input.as_mut_ptr(),
                            -1,
                            0,
                            AVSEEK_FLAG_BACKWARD | AVSEEK_FLAG_ANY,
                        );
                    }

                    // Flush the video decoder (if you don not, it will not render any frames anymore)
                    video_decoder.flush();
                } else {
                    break;
                }
            }
        });

        // return struct
        return VideoFileRenderer {
            wanted_time_in_second: wanted_time_in_second,
            last_decoded_timestamp: None,
            last_target_ts: None,
            stop_thread: stop_thread.clone(),
            size: size,
            timebase: timebase,
            decoded_frame_count: decoded_frame_count,
            rx: rx,
            infinite: infinite,
            duration: duration,
        };
    }

    /// Stops decoding
    /// This can't be undone
    pub fn stop_decoding(&mut self) {
        self.stop_thread
            .store(true, sync::atomic::Ordering::Relaxed);
    }

    /// Decode received frame
    /// if decoding fails, return None
    fn process_received_frames(
        decoder: &mut ffmpeg_next::codec::decoder::Video,
    ) -> Option<ffmpeg_next::frame::Video> {
        let mut decoded_frame = ffmpeg_next::frame::Video::empty();
        while decoder.receive_frame(&mut decoded_frame).is_ok() {
            return Some(decoded_frame);
        }
        None
    }

    /// Get video width and height
    pub(crate) fn get_size(&self) -> (u32, u32) {
        return self.size;
    }

    /// Renders frame around `wanted_time_in_second` property
    ///
    /// # CAUTION
    ///  - `texture` parameter shoulde be IYUV format streaming texture
    pub(crate) fn render_frame(&mut self, texture: &mut Texture) {
        // calculate desired timestamp with the timebase and `wanted_time_in_second` property
        // ideally, frame at the desired timestamp is the best.
        let target_ts = (if self.infinite {
            self.wanted_time_in_second % self.duration
        } else {
            self.wanted_time_in_second
        } / self.timebase)
            .to_integer() as i64;

        // is the wanted time backward?
        let target_ts_is_backwards =
            self.infinite && self.last_target_ts.is_some_and(|x| x > target_ts);

        if let Some(last_decoded_timestamp) = self.last_decoded_timestamp {
            if last_decoded_timestamp > target_ts && !target_ts_is_backwards {
                // too fast
                return;
            }
        }

        let mut reached_target_ts = false;

        // loop for decoded frame datas
        let mut frame_data = None;
        while let Ok(data) = self.rx.try_recv() {
            if target_ts_is_backwards
                && self
                    .last_decoded_timestamp
                    .is_some_and(|x| x <= data.timestamp)
            {
                // Consume all frames
                // until it start from beginning
                continue;
            } else if target_ts_is_backwards {
                // Set last_decoded_timestamp to the initial value if the time is backwards
                // because it should start from beginning again!
                self.last_decoded_timestamp = None;
            }

            self.decoded_frame_count
                .fetch_sub(1, sync::atomic::Ordering::Relaxed);
            self.last_decoded_timestamp = Some(data.timestamp);
            frame_data = Some(data.clone());
            if data.timestamp >= target_ts {
                // got desired frame
                reached_target_ts = true;
            }
            // break when got desired frame
            if reached_target_ts {
                break;
            }
        }

        // render only when we have something to render
        if let Some(frame_data_unwrapped) = frame_data {
            texture
                .update_yuv(
                    Rect::new(
                        0,
                        0,
                        frame_data_unwrapped.width,
                        frame_data_unwrapped.height,
                    ),
                    frame_data_unwrapped.y_plane.as_slice(),
                    frame_data_unwrapped.y_pitch,
                    frame_data_unwrapped.u_plane.as_slice(),
                    frame_data_unwrapped.u_pitch,
                    frame_data_unwrapped.v_plane.as_slice(),
                    frame_data_unwrapped.v_pitch,
                )
                .unwrap();
        }

        self.last_target_ts = Some(target_ts);
    }
}
