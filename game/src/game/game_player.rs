pub mod chart_player;
pub mod chart_player_ui;
pub mod effect_sound_player;
pub mod game_result;
pub mod janggu_state_with_tick;
pub mod timing_judge;

use std::{path::Path, thread};

use kira::{
    clock::ClockSpeed,
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};
use num_rational::Rational64;
use sdl2::{image::LoadTexture, pixels::PixelFormatEnum};

use crate::constants::DEFAULT_IMG_PATH as IMG_PATH;

use crate::game::{
    common::{event_loop_common, render_common},
    game_common_context,
};

use self::{
    chart_player::ChartPlayer, effect_sound_player::EffectSoundPlayer, game_result::GameResult,
    janggu_state_with_tick::JangguStateWithTick,
};

use bidrum_data_struct_lib::song::GameSong;

use super::render_video::VideoFileRenderer;

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
    let mut effect_sounds: EffectSoundPlayer = EffectSoundPlayer::new();

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

    // start the clock.
    clock.start().expect("Failed to start clock");

    // load video file and create video renderer and texture
    let mut video_file_renderer = None;
    let mut video_file_size = None;

    if let Some(video_file_name) = &song.video_filename {
        // If video_filename is specified, get video_file_render and video_file_size
        video_file_renderer = Some(VideoFileRenderer::new(Path::new(video_file_name), false));
        video_file_size = Some(
            video_file_renderer
                .as_ref()
                .expect("Failed to get video file renderer")
                .get_size(),
        );
    }

    let texture_creator = common_context.canvas.texture_creator();
    let mut texture = None;
    if let Some(video_file_size) = video_file_size {
        // If video_file_size is not None, get texture for video rendering
        texture = Some(
            texture_creator
                .create_texture_streaming(
                    PixelFormatEnum::IYUV,
                    video_file_size.0,
                    video_file_size.1,
                )
                .expect("Failed to create texture streaming"),
        ); // the texture should be streaming IYUV format
    }

    let play_background_texture = texture_creator
        .load_texture(IMG_PATH.to_owned() + "/play_ui/play_background.jpeg")
        .expect("Failed to load play background image.");

    // variables for displaying accuracy

    let mut janggu_state_with_tick = JangguStateWithTick::new();

    let mut chart_player = ChartPlayer::new(chart, &texture_creator);

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
            if let Some(ref mut video_file_renderer) = video_file_renderer {
                // if video_file_renderer is not None, render video
                video_file_renderer.wanted_time_in_second = Rational64::new(tick_now as i64, 1000);
                video_file_renderer
                    .render_frame(&mut texture.as_mut().expect("Failed to use texture."));
                common_context
                    .canvas
                    .copy(
                        texture.as_ref().expect("Failed to use texture."),
                        None,
                        None,
                    )
                    .unwrap();
            } else {
                common_context
                    .canvas
                    .copy(&play_background_texture, None, None)
                    .unwrap();
            }
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

        effect_sounds.play_janggu_sound(&janggu_state_with_tick, &mut common_context.audio_manager);
        effect_sounds.play_combo_sound(
            &chart_player.game_result(),
            &mut common_context.audio_manager,
        );
        // display notes and accuracy
        if tick_now >= 0 {
            chart_player.judge(
                &janggu_state_with_tick,
                common_context.hat.spinning(),
                tick_now,
            );
            chart_player.draw(
                tick_now,
                &mut common_context.canvas,
                common_context.game_initialized_at.elapsed().as_millis(),
                &janggu_state_with_tick,
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

    if let Some(mut video_file_renderer) = video_file_renderer {
        // If video_file_renderer is not None, stop playing video
        video_file_renderer.stop_decoding();
    }
    return Some(chart_player.game_result());
}
