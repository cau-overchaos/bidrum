use sdl2::rect::Rect;
use sdl2::render::Texture;

use std::path::Path;

use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context as Scaler, flag::Flags};
//use ffmpeg::util::rational::Rational;
use num_rational::Rational64;

/// Video file renderer with guarantee of rendering frame around `wanted_time_in_second`
/// without using delay
pub(crate) struct VideoFileRenderer {
    pub(crate) wanted_time_in_second: Rational64,
    last_decoded_timestamp: Option<i64>,
    input: ffmpeg_next::format::context::Input,
    stream_index: usize,
    video_decoder: ffmpeg_next::codec::decoder::Video,
}

impl VideoFileRenderer {
    /// Creates new VideoFileRenderer from video file path
    pub(crate) fn new(path: &Path) -> VideoFileRenderer {
        // init FFmpeg
        ffmpeg_next::init().unwrap();

        // init stream
        let input = input(&path.to_str().expect("Non-unicode character in path"))
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
        let video_decoder = context_decoder
            .decoder()
            .video()
            .expect("Failed to get video decoder");

        // return struct
        return VideoFileRenderer {
            wanted_time_in_second: Rational64::new(0, 1),
            input,
            stream_index: video_stream_index,
            video_decoder,
            last_decoded_timestamp: None,
        };
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
        return (self.video_decoder.width(), self.video_decoder.height());
    }

    /// Renders frame around `wanted_time_in_second` property
    ///
    /// # CAUTION
    ///  - `texture` parameter shoulde be IYUV format streaming texture
    pub(crate) fn render_frame(&mut self, texture: &mut Texture) {
        // get stream
        let stream = self
            .input
            .stream(self.stream_index)
            .expect("illegal stream index");

        // create scaler
        let mut scaler = Scaler::get(
            self.video_decoder.format(),
            self.video_decoder.width(),
            self.video_decoder.height(),
            Pixel::YUV420P,
            self.video_decoder.width(),
            self.video_decoder.height(),
            Flags::BILINEAR,
        )
        .unwrap();

        // get timebase
        // e.g. if timebase is 1/75, pts 1 means 1/75s, pts 2 means 2/75s, ...and more.
        let timebase = Rational64::new(
            stream.time_base().numerator() as i64,
            stream.time_base().denominator() as i64,
        );

        // calculate desired timestamp with the timebase and `wanted_time_in_second` property
        // ideally, frame at the desired timestamp is the best.
        let target_ts = (self.wanted_time_in_second / timebase).to_integer() as i64;
        if let Some(last_decoded_timestamp) = self.last_decoded_timestamp {
            if last_decoded_timestamp > target_ts {
                // too fast
                return;
            }
        }

        let mut unscaled_frame = ffmpeg_next::frame::Video::empty();
        let mut has_frame_data = false;
        let mut reached_target_ts = false;
        // loop for the packets of the video file
        for (stream, packet) in self.input.packets() {
            if stream.index() == self.stream_index {
                // send packet
                let _ = self.video_decoder.send_packet(&packet);

                // try to decode packets
                let processed_frame = Self::process_received_frames(&mut self.video_decoder);

                // if decoding success
                if let Some(decoded_frame) = processed_frame {
                    let decoded_frame_timestamp = decoded_frame.timestamp().unwrap();
                    self.last_decoded_timestamp = Some(decoded_frame_timestamp);
                    has_frame_data = true;
                    if decoded_frame_timestamp >= target_ts {
                        // got desired frame
                        reached_target_ts = true;
                    }
                    unscaled_frame = decoded_frame;

                    // break when got desired frame
                    if reached_target_ts {
                        break;
                    }
                }
            }
        }

        // render only when we have something to render
        if has_frame_data {
            let mut scaled_frame = ffmpeg_next::frame::Video::empty();
            scaler.run(&unscaled_frame, &mut scaled_frame).unwrap();
            texture
                .update_yuv(
                    Rect::new(0, 0, scaled_frame.width(), scaled_frame.height()),
                    scaled_frame.data(0),
                    scaled_frame.stride(0),
                    scaled_frame.data(1),
                    scaled_frame.stride(1),
                    scaled_frame.data(2),
                    scaled_frame.stride(2),
                )
                .unwrap();
        }
    }
}
