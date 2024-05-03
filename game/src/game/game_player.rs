mod draw_gameplay_ui;
pub mod game_result;
mod janggu_state_with_tick;
mod render_video;
mod timing_judge;

use std::{path::Path, thread};

use kira::{
    clock::ClockSpeed, manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings}, sound::static_sound::{StaticSoundData, StaticSoundSettings}, tween::Tween
};
use num_rational::Rational64;
use sdl2::{image::LoadTexture, pixels::PixelFormatEnum};

use crate::game::{
    common::{event_loop_common, render_common},
    game_common_context,
};

use self::{
    draw_gameplay_ui::{DisplayedSongNote, UIContent},
    game_result::GameResult,
    janggu_state_with_tick::JangguStateWithTick,
    render_video::VideoFileRenderer,
    timing_judge::{NoteAccuracy, TimingJudge},
};

use bidrum_data_struct_lib::{janggu::JangguFace, song::GameSong};

fn is_input_effect_needed(state: &JangguStateWithTick, tick: i128) -> [Option<JangguFace>; 2] {
    const TIME_DELTA: i128 = 150;
    let mut faces = [None, None];
    if let Some(_) = state.궁채.face {
        if state.궁채.keydown_timing - tick < TIME_DELTA {
            faces[0] = state.궁채.face;
        }
    }
    if let Some(_) = state.열채.face {
        if state.열채.keydown_timing - tick < TIME_DELTA {
            faces[1] = state.열채.face;
        }
    }

    faces
}

pub(crate) fn play_song(
    common_context: &mut game_common_context::GameCommonContext,
    song: &GameSong,
    level: u32,
) -> Option<GameResult> {
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

    // hit sould path
    let kung_hit_sound_path = "assets/sound/janggu_hit/kung.wav";
    let deok_hit_sound_path = "assets/sound/janggu_hit/deok.wav";

    // hit sound load
    let mut audio_manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("Failed to create audio manager");
    let kung_sound_data = StaticSoundData::from_file(kung_hit_sound_path, StaticSoundSettings::default()).expect("Failed to load kung sound");
    let deok_sound_data = StaticSoundData::from_file(deok_hit_sound_path, StaticSoundSettings::default()).expect("Failed to load deok sound");

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
                return None;
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
    let chart = song.get_chart(level).unwrap();
    let mut timing_judge = TimingJudge::new(&chart);

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

    let mut janggu_state_with_tick = JangguStateWithTick::new();
    let mut processed_note_ids = Vec::<u64>::new();

    let mut gameplay_ui_resources = draw_gameplay_ui::GamePlayUIResources::new(&texture_creator);

    'running: loop {
        let tick_now = clock.time().ticks as i128 - start_tick.ticks as i128;
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                handle.stop(Tween::default()).expect("Failed to stop song");
                break 'running;
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
            for i in &chart.left_face {
                if !processed_note_ids.contains(&i.id) {
                    display_notes.push(DisplayedSongNote {
                        face: JangguFace::궁편,
                        stick: i.stick,
                        distance: i.get_position(
                            chart.bpm,
                            chart.delay,
                            chart.bpm * 2,
                            (tick_now) as u64,
                        ),
                    });
                }
            }

            for i in &chart.right_face {
                if !processed_note_ids.contains(&i.id) {
                    display_notes.push(DisplayedSongNote {
                        face: JangguFace::열편,
                        stick: i.stick,
                        distance: i.get_position(
                            chart.bpm,
                            chart.delay,
                            chart.bpm * 2,
                            (tick_now) as u64,
                        ),
                    });
                }
            }

            // make judgement
            let input_now = common_context.read_janggu_state();
            janggu_state_with_tick.update(input_now, tick_now);
            let new_accuracies = timing_judge.judge(&janggu_state_with_tick, tick_now as u64);

            // play hit sound when use git janggu
            if janggu_state_with_tick.궁채.is_keydown_now {
                audio_manager.play(kung_sound_data.clone()).expect("Failed to paly kung sound");
            }
            if janggu_state_with_tick.열채.is_keydown_now {
                audio_manager.play(deok_sound_data.clone()).expect("Failed to paly deok sound");
            }

            // if any judgement is made, display it
            if !new_accuracies.is_empty() {
                accuracy_tick = Some(tick_now);
                accuracy = new_accuracies.iter().map(|x| x.accuracy).max();
                for i in new_accuracies {
                    processed_note_ids.push(i.note_id);
                }
            }
        }

        // judgement is visible for only 1200 ms
        const ACCURACY_DISPLAY_DURATION: u32 = 800;
        if let Some(accuracy_tick_unwrapped) = accuracy_tick {
            if tick_now - accuracy_tick_unwrapped > ACCURACY_DISPLAY_DURATION as i128 {
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
                accuracy_time_progress: if let Some(accuracy_time_unwrapped) = accuracy_tick {
                    Some(
                        (tick_now - accuracy_time_unwrapped) as f32
                            / ACCURACY_DISPLAY_DURATION as f32,
                    )
                } else {
                    None
                },
                input_effect: is_input_effect_needed(&janggu_state_with_tick, tick_now),
            },
            &mut gameplay_ui_resources,
        );

        // display necessary data such as coin count
        render_common(common_context);

        common_context.canvas.present();
        if tick_now > 0 {
            match handle.state() {
                kira::sound::PlaybackState::Playing => {}
                // break the loop when the song ends
                _ => {
                    break 'running;
                }
            }
        }
    }
    return Some(timing_judge.get_game_result());
}
