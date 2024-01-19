mod songs;
mod render_video;

use std::{path::Path, thread};

use ffmpeg_next::{codec::{Video, video}, device::input::video};
use kira::{clock::ClockSpeed, sound::static_sound::{StaticSoundSettings, StaticSoundData}, tween::Tween};
use num_rational::Rational64;
use sdl2::pixels::{Color, PixelFormatEnum};

use crate::game::{game_common_context, common::{self, event_loop_common, render_common}};

use self::render_video::VideoFileRenderer;

pub(crate) struct GameSong<'a> {
    pub audio_filename: &'a Path,
    pub video_filename: &'a Path
}

pub(crate) fn play_song(common_context: &mut game_common_context::GameCommonContext, song: GameSong) {
    let mut clock = common_context.audio_manager.add_clock(ClockSpeed::TicksPerSecond(1000.0)).expect("clock initialization failure");
    let start_tick = clock.time() + 500;
    let song_path_string = song.audio_filename.to_str().expect("Non-unicode in path").to_string();
    
    let sound_load_thread = thread::spawn(move || {
        return StaticSoundData::from_file(
            song_path_string.as_str(),
            StaticSoundSettings::new().start_time(start_tick))
            .expect("Data initialization failure");
        });
        
    let sound_data = loop {
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                return;
            }
        }

        if sound_load_thread.is_finished() {
            break sound_load_thread.join().expect("Data initialization failure");
        }

        common_context.canvas.set_draw_color(Color::WHITE);
        common_context.canvas.clear();
        render_common(common_context);
        common_context.canvas.present();
    };
        
    let mut handle = common_context.audio_manager.play(sound_data).expect("Audio play failure");
    // Start the clock.
    clock.start().expect("Failed to start clock");
    let mut video_file_renderer = VideoFileRenderer::new(&song.video_filename);
    let video_file_size = video_file_renderer.get_size();
    let texture_creator = common_context.canvas.texture_creator();
    let mut texture = texture_creator
    .create_texture_streaming(
        PixelFormatEnum::IYUV, video_file_size.0, video_file_size.1)
        .expect("Failed to create texture");

    'running: loop {
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                handle.stop(Tween::default()).expect("Failed to stop song");
                break 'running;
            }
        }

        common_context.canvas.set_draw_color(Color::BLUE);
        common_context.canvas.clear();
        if clock.time().ticks >= start_tick.ticks {
            video_file_renderer.wanted_time_in_second = Rational64::new((clock.time().ticks - start_tick.ticks)as i64, 1000);
            video_file_renderer.render_frame(&mut texture);
            common_context.canvas.copy(&texture, None, None);
        }
        render_common(common_context);
        common_context.canvas.present();
        if clock.time().ticks > start_tick.ticks {
            match handle.state() {
                kira::sound::PlaybackState::Playing => {
                    // Do nothing
                    println!("tick {}", clock.time().ticks);
                },
                _ => break 'running
            }
        }
    }
}