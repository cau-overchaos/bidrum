pub mod songs;
mod render_video;
mod draw_ui;
mod process_notes;

use std::{path::{Path, Display}, thread};

use ffmpeg_next::{codec::{Video, video}, device::input::video};
use kira::{clock::ClockSpeed, sound::static_sound::{StaticSoundSettings, StaticSoundData}, tween::Tween};
use num_rational::Rational64;
use sdl2::{image::LoadTexture, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, sys::KeyPress};

use crate::{game::{game_common_context, common::{self, event_loop_common, render_common}}, janggu::{DrumPane, JangguState}, serial::keys_to_state_struct};

use self::{draw_ui::{DisplayedSongNote, UIContent}, process_notes::{NoteAccuracy, NoteProcessor}, render_video::VideoFileRenderer, songs::GameSong};

pub(crate) fn play_song(common_context: &mut game_common_context::GameCommonContext, song: &GameSong, level: u64) {
    // Load cover image texture
    let cover_img_path = Path::new(&song.cover_image_filename);
    let texture_creator =  common_context.canvas.texture_creator();
    let cover_img_texture = texture_creator
        .load_texture(cover_img_path)
        .expect("Logo file not found");
    
    let mut clock = common_context.audio_manager.add_clock(ClockSpeed::TicksPerSecond(1000.0)).expect("clock initialization failure");
    let start_tick = clock.time() + 500;
    let song_path_string = song.audio_filename.clone();
    
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

        common_context.canvas.clear();
        common_context.canvas.copy(&cover_img_texture, None, None);
        render_common(common_context);
        common_context.canvas.present();
    };
        
    let mut handle = common_context.audio_manager.play(sound_data).expect("Audio play failure");
    
    let level = song.get_level(1).unwrap();
    let mut note_processor = NoteProcessor::new(&level.tracks);
    
    // Start the clock.
    clock.start().expect("Failed to start clock");
    let mut video_file_renderer = VideoFileRenderer::new(Path::new(&song.video_filename));
    let video_file_size = video_file_renderer.get_size();
    let texture_creator = common_context.canvas.texture_creator();
    let mut texture = texture_creator
    .create_texture_streaming(
        PixelFormatEnum::IYUV, video_file_size.0, video_file_size.1)
        .expect("Failed to create texture");

    // Need for displaying accuracy
    let mut accuracy: Option<NoteAccuracy> = None;
    let mut accuracy_tick: Option<i128> = None;

    'running: loop {
        let mut key1 = false;
        let mut key2 = false;
        let mut key3 = false;
        let mut key4 = false;
        let tick_now = clock.time().ticks as i128 - start_tick.ticks as i128;
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                handle.stop(Tween::default()).expect("Failed to stop song");
                break 'running;
            }
            match event {
                sdl2::event::Event::KeyDown { 
                    keycode: Some(Keycode::U),
                    ..
                 } => {
                    key1 = true;
                 },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    key2 = true;
                },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    key3 = true;
                },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    key4 = true;
                },
                  _ => {}
            }
        }

        common_context.canvas.clear();
        if tick_now >= 0 {
            video_file_renderer.wanted_time_in_second = Rational64::new(tick_now as i64, 1000);
            video_file_renderer.render_frame(&mut texture);
            common_context.canvas.copy(&texture, None, None);
        } else {
            common_context.canvas.copy(&cover_img_texture, None, None);
        }
        render_common(common_context);
        let mut display_notes = Vec::<DisplayedSongNote>::new();
        if tick_now >= 0 {
            for i in &level.tracks {
                for j in &i.notes {
                    display_notes.push(
                        DisplayedSongNote {
                            궁채: j.궁채,
                            열채: j.열채,
                            distance: j.get_position(
                                i.bpm as u64, 
                                i.delay,
                                i.bpm as u64 * 2, 
                                (tick_now) as u64,
                            )
                        }
                    );
                }
            }

            let input_now = keys_to_state_struct(key1, key2, key3, key4);
            let new_accuracy = note_processor.process(input_now,  tick_now as u64);
            if let Some(new_accuracy_unwrapped) = new_accuracy {
                accuracy_tick = Some(tick_now);
                accuracy = Some(new_accuracy_unwrapped);
            }
        }

        if let Some(accuracy_tick_unwrapped) = accuracy_tick {
            if tick_now - accuracy_tick_unwrapped > 1200 {
                accuracy_tick = None;
            }
        }

        draw_ui::draw_ui(&mut common_context.canvas, display_notes, UIContent {
            accuracy: if let Some(_) = accuracy_tick {
                accuracy
            } else {
                None
            }
        });
        common_context.canvas.present();
        if tick_now > 0 {
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