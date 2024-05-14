pub mod draw_gameplay_ui;
pub mod game_result;
pub mod janggu_state_with_tick;
pub mod judge_and_display_notes;
pub mod load_hit_sounds;
mod render_video;
pub mod timing_judge;

use std::{path::Path, thread};

use kira::{
    clock::ClockSpeed,
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};
use num_rational::Rational64;
use sdl2::{image::LoadTexture, pixels::PixelFormatEnum};

use crate::game::{
    common::{event_loop_common, render_common},
    game_common_context,
    game_player::judge_and_display_notes::display_notes_and_judge,
};

use self::{
    draw_gameplay_ui::{DisplayedSongNote, UIContent},
    game_result::GameResult,
    janggu_state_with_tick::JangguStateWithTick,
    judge_and_display_notes::EffectSoundHandles,
    load_hit_sounds::load_hit_sounds,
    render_video::VideoFileRenderer,
    timing_judge::{NoteAccuracy, TimingJudge},
};

use bidrum_data_struct_lib::{janggu::JangguFace, song::GameSong};

pub fn is_input_effect_needed(state: &JangguStateWithTick, tick: i128) -> [Option<JangguFace>; 2] {
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

    // Load hit sound data
    let hit_sounds = load_hit_sounds();
    let mut effect_sound_handles = EffectSoundHandles::new();

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

        // Update janggu state
        let input_now = common_context.read_janggu_state();
        janggu_state_with_tick.update(input_now, tick_now);

        // display notes and accuracy
        if tick_now >= 0 {
            display_notes_and_judge(
                common_context,
                &chart,
                &mut timing_judge,
                &janggu_state_with_tick,
                &mut gameplay_ui_resources,
                &mut processed_note_ids,
                &mut accuracy,
                &mut accuracy_tick,
                &hit_sounds,
                &mut effect_sound_handles,
                tick_now,
            );
        }

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

    video_file_renderer.stop_decoding();

    return Some(timing_judge.get_game_result());
}
