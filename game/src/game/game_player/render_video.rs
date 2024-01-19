// From https://github.com/RustWorks/spe-FFMPEG-SDL2-Video-Player

use ffmpeg_next::device::input::video;
use ffmpeg_next::encoder::video::Video;
use ffmpeg_next::ffi::{av_seek_frame, AVSEEK_FLAG_ANY, AV_TIME_BASE, AVSEEK_FLAG_FRAME, AVSEEK_FLAG_BACKWARD};
use ffmpeg_next::{codec, decoder, Rational, Stream};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use std::os::unix::process;
use std::path::Path;
use std::{time, thread};

use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context as Scaler, flag::Flags};
//use ffmpeg::util::rational::Rational;
use num_rational::{Rational64, Ratio};
use std::{
    sync::mpsc::{channel},
};

use crate::game::game_common_context::{GameCommonContext, self};

pub(crate) struct VideoFileRenderer {
    pub(crate) wanted_time_in_second: Rational64,
    last_decoded_timestamp: Option<i64>,
    input: ffmpeg_next::format::context::Input,
    stream_index: usize,
    video_decoder: ffmpeg_next::codec::decoder::Video
}

fn get_size_from_stream(stream: Stream) -> (u32, u32) {
    let context_decoder = ffmpeg_next::codec::context::Context::from_parameters(stream.parameters()).expect("Failed to get decoder");
    let mut video_decoder = context_decoder.decoder().video().expect("Failed to get video decoder");
    
    return (video_decoder.width(), video_decoder.height());

}

impl VideoFileRenderer {
    pub(crate) fn new(path: &Path) -> VideoFileRenderer {
        ffmpeg_next::init().unwrap();

        let mut input = input(&path.to_str().expect("Non-unicode character in path")).expect("Failed to open file");
        let mut video_stream_index = input.streams().best(Type::Video).expect("Video stream not found").index();
        let stream = input.stream(video_stream_index).expect("illegal stream index");
        let context_decoder = ffmpeg_next::codec::context::Context::from_parameters(stream.parameters().clone()).expect("Failed to get decoder");
        let mut video_decoder = context_decoder.decoder().video().expect("Failed to get video decoder");

        let result = VideoFileRenderer {
            wanted_time_in_second: Rational64::new(0, 1),
            input,
            stream_index: video_stream_index,
            video_decoder,
            last_decoded_timestamp: None
        };

        return result;
    }

    fn process_received_frames(decoder: &mut ffmpeg_next::codec::decoder::Video) -> Option<ffmpeg_next::frame::Video> {
        let mut decoded_frame = ffmpeg_next::frame::Video::empty();
        while decoder.receive_frame(&mut decoded_frame).is_ok() {
            return Some(decoded_frame);
        }
        None
    }

    pub(crate) fn get_size(&self) -> (u32, u32) {
        return (self.video_decoder.width(), self.video_decoder.height());
    }

    /// texture shoulde be IYUV format
    pub(crate) fn render_frame(&mut self, texture: &mut Texture) {
        let stream = self.input.stream(self.stream_index).expect("illegal stream index");
        
        let mut scaler = Scaler::get(
            self.video_decoder.format(),
            self.video_decoder.width(),
            self.video_decoder.height(),
            Pixel::YUV420P,
            self.video_decoder.width(),
            self.video_decoder.height(),
            Flags::BILINEAR,
        ).unwrap();

        let timebase=Rational64::new(
            stream.time_base().numerator() as i64,
            stream.time_base().denominator() as i64,
        );
        let target_ts = (self.wanted_time_in_second / timebase).to_integer() as i64;
        if let Some(last_decoded_timestamp) = self.last_decoded_timestamp {
            if last_decoded_timestamp > target_ts {
                return;
            }
        }

        let mut unscaled_frame = ffmpeg_next::frame::Video::empty();
        let mut has_frame_data = false;
        let mut reached_target_ts = false;
        //println!("kast_ts={} {}s", last_ts, self.time_in_second.to_integer());
        for (stream, packet) in self.input.packets() {
            if stream.index() == self.stream_index {
                self.video_decoder.send_packet(&packet);
                let processed_frame = Self::process_received_frames(&mut self.video_decoder);
                if let Some(decoded_frame) = processed_frame {
                    let decoded_frame_timestamp =decoded_frame.timestamp().unwrap();
                    self.last_decoded_timestamp = Some(decoded_frame_timestamp);
                    println!("timestamp={}, last_ts={}", decoded_frame_timestamp, target_ts);
                    has_frame_data = true;
                    if decoded_frame_timestamp > target_ts {
                        reached_target_ts = true;
                    }
                    unscaled_frame = decoded_frame;
                    if reached_target_ts {
                        break;
                    }
                }
            }
        }

        if has_frame_data {
            println!("rendered");
            let mut scaled_frame = ffmpeg_next::frame::Video::empty();
            scaler.run(&unscaled_frame, &mut scaled_frame);
            texture.update_yuv(
                Rect::new(0, 0, scaled_frame.width(), scaled_frame.height()),
                scaled_frame.data(0),
                scaled_frame.stride(0),
                scaled_frame.data(1),
                scaled_frame.stride(1),
                scaled_frame.data(2),
                scaled_frame.stride(2),
            ).unwrap();
        }
    }
}

/*
pub(crate) fn render_video_frame(common_context: &mut GameCommonContext, path: &Path, timestamp_in_second: Rational64) -> Result<(), ffmpeg_next::Error> {
    ffmpeg_next::init().unwrap();

    if let Ok(mut ictx) = input(&path.to_str().expect("Failed to open file")) {
        let in_stream = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg_next::Error::StreamNotFound)?;

        let stream_index = in_stream.index();

        let context_decoder = ffmpeg_next::codec::context::Context::from_parameters(in_stream.parameters())?;
        let mut video_decoder = context_decoder.decoder().video()?;

        let mut scaler = Scaler::get(
            video_decoder.format(),
            video_decoder.width(),
            video_decoder.height(),
            Pixel::YUV420P,
            video_decoder.width(),
            video_decoder.height(),
            Flags::BILINEAR,
        )?;

        let texture_creator = common_context.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::IYUV,
                video_decoder.width(),
                video_decoder.height(),
            )
            .unwrap();


        let timebase=Rational64::new(
            in_stream.time_base().numerator() as i64,
            in_stream.time_base().denominator() as i64,
        );

        let ts = (timestamp_in_second / timebase).to_integer();
        video_decoder.flush();
        ictx.seek(ts, ts..ts).unwrap();
        video_decoder.flush();

        for (stream, packet) in ictx.packets() {
            if stream.index() == stream_index {
                video_decoder.send_packet(&packet)?;
                if receive_and_process_decoded_frames(&mut video_decoder, timebase) {
                    return Ok(());
                }
            }
        }
        video_decoder.send_eof()?;
        
        if receive_and_process_decoded_frames(&mut video_decoder, timebase) {
            return Ok(());
        }
    }
    Ok(())
}
 */