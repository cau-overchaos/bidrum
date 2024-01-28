mod draw_gameplay_ui;
mod render_video;
pub mod songs;
mod timing_judge;

use std::{path::Path, thread};

use kira::{
    clock::ClockSpeed,
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};
use num_rational::Rational64;
use sdl2::{image::LoadTexture, keyboard::Keycode, pixels::PixelFormatEnum};

use crate::{
    game::{
        common::{event_loop_common, render_common},
        game_common_context,
    },
    serial::keys_to_state_struct,
};

use self::{
    draw_gameplay_ui::{DisplayedSongNote, UIContent},
    render_video::VideoFileRenderer,
    songs::GameSong,
    timing_judge::{NoteAccuracy, TimingJudge},
};

pub(crate) fn play_song(
    common_context: &mut game_common_context::GameCommonContext,
    song: &GameSong,
    _level: u64,
) {
    // Load cover image texture
    let cover_img_path = Path::new(&song.cover_image_filename);
    let texture_creator = common_context.canvas.texture_creator();
    let cover_img_texture = texture_creator
        .load_texture(cover_img_path)
        .expect("Logo file not found");

    // create clock for audio
    // the clock is used to play the audio file at the precise tick timing
    let clock = common_context
        .audio_manager
        .add_clock(ClockSpeed::TicksPerSecond(1000.0)) // tick per 1 millisecond
        .expect("clock initialization failure");
    let start_tick = clock.time() + 500; // the song will start at 500ms after clock starting
    let song_path_string = song.audio_filename.clone();

    // to receive coin input while loading the audio file,
    // loading should be done in separated thread.
    let sound_load_thread = thread::spawn(move || {
        return StaticSoundData::from_file(
            song_path_string.as_str(),
            StaticSoundSettings::new().start_time(start_tick),
        )
        .expect("Data initialization failure");
    });

    // get audio file data
    // while waiting for audio file data, processes input loop
    // and display necessary data such as coin count
    let sound_data = loop {
        // process input events
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                return;
            }
        }

        // if loading is finished, break the loop with the loaded audio data
        if sound_load_thread.is_finished() {
            break sound_load_thread
                .join()
                .expect("Data initialization failure");
        }

        // display song cover image while loading
        common_context.canvas.clear();
        common_context
            .canvas
            .copy(&cover_img_texture, None, None)
            .unwrap();

        // display necessary data such as coin count
        render_common(common_context);

        common_context.canvas.present();
    };

    // create handle for audio output
    let mut handle = common_context
        .audio_manager
        .play(sound_data)
        .expect("Audio play failure");

    // get judge and create timing judge
    let chart = song.get_chart(1).unwrap();
    let mut timing_judge = TimingJudge::new(&chart.tracks);

    // start the clock.
    clock.start().expect("Failed to start clock");

    // load video file and create video renderer and texture
    let mut video_file_renderer = VideoFileRenderer::new(Path::new(&song.video_filename));
    let video_file_size = video_file_renderer.get_size();
    let texture_creator = common_context.canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::IYUV, video_file_size.0, video_file_size.1) // the texture should be streaming IYUV format
        .expect("Failed to create texture");

    // variables for displaying accuracy
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

            // saves current key input
            // only handles keydown
            match event {
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::U),
                    repeat: false,
                    ..
                } => {
                    key1 = true;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::I),
                    repeat: false,
                    ..
                } => {
                    key2 = true;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::O),
                    repeat: false,
                    ..
                } => {
                    key3 = true;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::P),
                    repeat: false,
                    ..
                } => {
                    key4 = true;
                }
                _ => {}
            }
        }

        common_context.canvas.clear();

        // display bga
        if tick_now >= 0 {
            video_file_renderer.wanted_time_in_second = Rational64::new(tick_now as i64, 1000);
            video_file_renderer.render_frame(&mut texture);
            common_context.canvas.copy(&texture, None, None).unwrap();
        } else {
            // song is not started yet
            // therefore display game cover image
            common_context
                .canvas
                .copy(&cover_img_texture, None, None)
                .unwrap();
        }

        // display notes and accuracy
        let mut display_notes = Vec::<DisplayedSongNote>::new();
        if tick_now >= 0 {
            // get positions of the notes
            for i in &chart.tracks {
                for j in &i.notes {
                    display_notes.push(DisplayedSongNote {
                        궁채: j.궁채,
                        북채: j.북채,
                        distance: j.get_position(i.bpm, i.delay, i.bpm * 2, (tick_now) as u64),
                    });
                }
            }

            // make judgement
            let input_now = keys_to_state_struct(key1, key2, key3, key4);
            let new_accuracy = timing_judge.judge(input_now, tick_now as u64);

            // if any judgement is made, display it
            if let Some(new_accuracy_unwrapped) = new_accuracy {
                accuracy_tick = Some(tick_now);
                accuracy = Some(new_accuracy_unwrapped);
            }
        }

        // judgement is visible for only 1200 ms
        if let Some(accuracy_tick_unwrapped) = accuracy_tick {
            if tick_now - accuracy_tick_unwrapped > 1200 {
                accuracy_tick = None;
            }
        }

        // draw game play ui
        draw_gameplay_ui::draw_gameplay_ui(
            &mut common_context.canvas,
            display_notes,
            UIContent {
                accuracy: if let Some(_) = accuracy_tick {
                    accuracy
                } else {
                    None
                },
            },
        );

        // display necessary data such as coin count
        render_common(common_context);

        common_context.canvas.present();
        if tick_now > 0 {
            match handle.state() {
                kira::sound::PlaybackState::Playing => {}
                // break the loop when the song ends
                _ => break 'running,
            }
        }
    }
}
