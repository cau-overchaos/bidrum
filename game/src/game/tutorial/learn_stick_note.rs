use std::{
    ops::Sub,
    path::{self},
    time::{Duration, Instant},
};

use bidrum_data_struct_lib::{
    janggu::{JangguFace, JangguStick},
    song::GameChart,
};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use sdl2::{image::LoadTexture, rect::Rect, render::Texture};

use crate::constants::DEFAULT_AUDIO_PATH as AUDIO_PATH;
use crate::constants::DEFAULT_IMG_PATH as IMG_PATH;

use crate::game::{
    common::event_loop_common,
    game_common_context::GameCommonContext,
    game_player::{
        chart_player::ChartPlayer,
        chart_player_ui::{displayed_song_note::DisplayedSongNote, ChartPlayerUI},
        effect_sound_player::EffectSoundPlayer,
        janggu_state_with_tick::JangguStateWithTick,
    },
};

use super::{display_tutorial_messages, get_message_image_asset_dst_rect};

fn display_animated_example_note(
    common_context: &mut GameCommonContext,
    janggu_state_and_tutorial_start_time: &mut (&mut JangguStateWithTick, Instant),
    message: &(Texture, StaticSoundData),
    stick: JangguStick,
    pane: JangguFace,
) {
    let texture_creator = common_context.canvas.texture_creator();
    let animation_frames = [1, 2, 3, 4, 5, 6].map(|idx| -> Texture {
        return texture_creator
            .load_texture(format!(
                "{}/tutorial/{}_stick/{}_pane_hit_animation/{}.png",
                IMG_PATH,
                match stick {
                    JangguStick::궁채 => "left",
                    JangguStick::열채 => "right",
                },
                match pane {
                    JangguFace::궁편 => "left",
                    JangguFace::열편 => "right",
                },
                idx
            ))
            .expect("Animation frame load failure");
    });

    let animation_frame_width = 150;
    let animation_frame_height = (animation_frames[0].query().height as f32
        * (animation_frame_width as f32 / animation_frames[0].query().width as f32))
        as u32;
    let janggu_animation_rect = Rect::new(
        (common_context.canvas.viewport().width() as i32) / 2
            + match pane {
                JangguFace::궁편 => -(60 + animation_frame_width as i32),
                JangguFace::열편 => 60,
            },
        (common_context.canvas.viewport().height() as i32) / 2 + 120,
        animation_frame_width,
        animation_frame_height,
    );

    let note_count = 3;
    let note_duration = std::cmp::max(Duration::from_secs(2), message.1.duration() / 3);
    let total_note_duration = note_duration * note_count;
    common_context
        .audio_manager
        .play(message.1.clone())
        .expect("Failed to play tutorial message");
    let voice_started_at = Instant::now();

    let mut chart_player_ui = ChartPlayerUI::new(&texture_creator);

    loop {
        for event in common_context.event_pump.poll_iter() {
            event_loop_common(&event, &mut common_context.coins);
        }

        if voice_started_at.elapsed() >= std::cmp::max(total_note_duration, message.1.duration()) {
            return;
        }

        // Update janggu input state
        let tick = janggu_state_and_tutorial_start_time.1.elapsed().as_millis() as i128;
        janggu_state_and_tutorial_start_time
            .0
            .update(common_context.read_janggu_state(), tick);

        // Clear canvas
        common_context.canvas.clear();

        // Display message
        common_context
            .canvas
            .copy(
                &message.0,
                None,
                get_message_image_asset_dst_rect(
                    common_context.canvas.viewport(),
                    message.0.query().width,
                    message.0.query().height,
                ),
            )
            .expect("Tutorial message image asset copy failure");

        // Display UI
        let note_positions = (0..note_count)
            .map(|idx| -> f64 {
                return 6.0 * idx as f64
                    + 6.0
                        * (1.0
                            - (voice_started_at.elapsed().as_millis() as f64
                                / note_duration.as_millis() as f64));
            })
            .filter(|i| *i >= 0.0);

        chart_player_ui.notes = note_positions
            .clone()
            .map(|position| -> DisplayedSongNote {
                return DisplayedSongNote {
                    distance: position,
                    face: pane,
                    stick: stick,
                };
            })
            .collect();
        chart_player_ui
            .input_effect
            .update(janggu_state_and_tutorial_start_time.0, tick);
        chart_player_ui.overall_effect_tick =
            common_context.game_initialized_at.elapsed().as_millis();
        chart_player_ui.draw(&mut common_context.canvas);

        // Display janggu animation
        let frame_index = if let Some(min_note_position) =
            note_positions.min_by(|a, b| a.partial_cmp(b).unwrap())
        {
            if min_note_position > 1.5 {
                (voice_started_at.elapsed().as_secs() * 2) as usize % 2
            } else {
                (((1.5 - min_note_position) / 1.5) * (animation_frames.len() as f64 - 1.0)) as usize
            }
        } else {
            (voice_started_at.elapsed().as_secs() * 2) as usize % 2
        };
        common_context
            .canvas
            .copy(&animation_frames[frame_index], None, janggu_animation_rect)
            .expect("Animation failure");

        common_context.canvas.present();
    }
}

fn display_tryitout_notes(
    common_context: &mut GameCommonContext,
    stick: JangguStick,
    pane: JangguFace,
) {
    let texture_creator = common_context.canvas.texture_creator();

    // Load effect sounds
    let mut effect_sound_player = EffectSoundPlayer::new();

    // Load janggu-hitting instruction animation frames
    let animation_frames = [1, 2, 3, 4, 5, 6].map(|idx| -> Texture {
        return texture_creator
            .load_texture(format!(
                "{}/tutorial/{}_stick/{}_pane_hit_animation/{}.png",
                IMG_PATH,
                match stick {
                    JangguStick::궁채 => "left",
                    JangguStick::열채 => "right",
                },
                match pane {
                    JangguFace::궁편 => "left",
                    JangguFace::열편 => "right",
                },
                idx
            ))
            .expect("Animation frame load failure");
    });

    // Calculate jnaggu-hitting animation coords and size
    let animation_frame_width = 150;
    let animation_frame_height = (animation_frames[0].query().height as f32
        * (animation_frame_width as f32 / animation_frames[0].query().width as f32))
        as u32;
    let janggu_animation_rect = Rect::new(
        (common_context.canvas.viewport().width() as i32) / 2
            + match pane {
                JangguFace::궁편 => -(60 + animation_frame_width as i32),
                JangguFace::열편 => 60,
            },
        (common_context.canvas.viewport().height() as i32) / 2 + 120,
        animation_frame_width,
        animation_frame_height,
    );

    // Generate tutorial chart
    let note_count = 5;
    let note_gap = 10;
    let chart_bpm = 120;
    let chart =
        GameChart::create_example_chart_for_tutorial(stick, pane, note_count, note_gap, chart_bpm);

    // Prepare tutorial play
    let mut judged_all_at = None;
    let tryitout_tutorial_started_at = Instant::now();

    let mut chart_player = ChartPlayer::new(chart.clone(), &texture_creator);

    let mut janggu_state = JangguStateWithTick::new();
    janggu_state.update(common_context.read_janggu_state(), 0);

    loop {
        for event in common_context.event_pump.poll_iter() {
            event_loop_common(&event, &mut common_context.coins);
        }

        // If tutorial ends, return
        if chart_player.game_result().total_judged_note_count() == note_count
            && judged_all_at.is_none()
        {
            judged_all_at = Some(Instant::now())
        } else if judged_all_at.is_some_and(|x| x.elapsed().as_millis() > 400) {
            return;
        }

        // Update janggu input state
        let tick = tryitout_tutorial_started_at.elapsed().as_millis() as u64;
        janggu_state.update(common_context.read_janggu_state(), tick as i128);

        // Clear canvas
        common_context.canvas.clear();

        // Play effect sound
        effect_sound_player.play_janggu_sound(&janggu_state, &mut common_context.audio_manager);

        // Judge and display UI
        chart_player.judge(&janggu_state, common_context.hat.spinning(), tick.into());
        chart_player.draw(
            tick.into(),
            &mut common_context.canvas,
            common_context.game_initialized_at.elapsed().as_millis(),
            &janggu_state,
        );

        // Display janggu animation
        let frame_index = if let Some(min_note_position) =
            [chart.left_face.clone(), chart.right_face.clone()]
                .concat()
                .iter()
                .filter(|x| (x.timing_in_ms(chart.bpm, chart.delay) as i64).sub(tick as i64) > -800)
                .map(|x| x.get_position(chart.bpm, chart.delay, 120, tick))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
        {
            if min_note_position > 1.5 {
                (tryitout_tutorial_started_at.elapsed().as_secs() * 2) as usize % 2
            } else {
                ((((1.5 - min_note_position) / 1.5) * (animation_frames.len() as f64 - 1.0))
                    as usize)
                    .clamp(0, animation_frames.len() - 1)
            }
        } else {
            (tryitout_tutorial_started_at.elapsed().as_secs() * 2) as usize % 2
        };
        common_context
            .canvas
            .copy(&animation_frames[frame_index], None, janggu_animation_rect)
            .expect("Animation failure");

        common_context.canvas.present();
    }
}

pub(crate) fn do_learn_stick_note(
    common_context: &mut GameCommonContext,
    janggu_state_and_tutorial_start_time: &mut (&mut JangguStateWithTick, Instant),
    stick: JangguStick,
) {
    let sub_directory_name = match stick {
        JangguStick::궁채 => "left_stick",
        JangguStick::열채 => "right_stick",
    };

    // Load tutorial message images and sounds
    let texture_creator = common_context.canvas.texture_creator();
    let messages = [1, 2, 3, 4, 5, 6].map(|idx| -> (Texture, StaticSoundData) {
        return (
            texture_creator
                .load_texture(format!(
                    "{}/tutorial/{}/{}.png",
                    IMG_PATH, sub_directory_name, idx
                ))
                .expect("Stick tutorial image asset load failure"),
            kira::sound::static_sound::StaticSoundData::from_file(
                path::Path::new(
                    format!("{}/tutorial/{}/{}.mp3", AUDIO_PATH, sub_directory_name, idx).as_str(),
                ),
                StaticSoundSettings::default(),
            )
            .expect("Stick tutorial audio load failure"),
        );
    });

    // Display two messages, Telling how the note looks like, at first
    display_tutorial_messages(
        common_context,
        &messages[..2],
        janggu_state_and_tutorial_start_time,
    );

    display_animated_example_note(
        common_context,
        janggu_state_and_tutorial_start_time,
        &messages[2],
        stick,
        match stick {
            JangguStick::궁채 => JangguFace::궁편,
            JangguStick::열채 => JangguFace::열편,
        },
    );

    display_tutorial_messages(
        common_context,
        &messages[3..4],
        janggu_state_and_tutorial_start_time,
    );

    display_tryitout_notes(
        common_context,
        stick,
        match stick {
            JangguStick::궁채 => JangguFace::궁편,
            JangguStick::열채 => JangguFace::열편,
        },
    );

    display_animated_example_note(
        common_context,
        janggu_state_and_tutorial_start_time,
        &messages[4],
        stick,
        match stick {
            JangguStick::궁채 => JangguFace::열편,
            JangguStick::열채 => JangguFace::궁편,
        },
    );

    display_tutorial_messages(
        common_context,
        &messages[5..6],
        janggu_state_and_tutorial_start_time,
    );

    display_tryitout_notes(
        common_context,
        stick,
        match stick {
            JangguStick::궁채 => JangguFace::열편,
            JangguStick::열채 => JangguFace::궁편,
        },
    );
}
