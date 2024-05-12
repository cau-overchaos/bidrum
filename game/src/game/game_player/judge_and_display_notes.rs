use bidrum_data_struct_lib::{
    janggu::JangguFace,
    song::{GameChart, GameSong},
};
use kira::sound::static_sound::StaticSoundData;

use crate::game::{
    game_common_context,
    game_player::{
        draw_gameplay_ui::{DisplayedSongNote, UIContent},
        is_input_effect_needed,
        timing_judge::NoteAccuracy,
    },
};

use super::{
    draw_gameplay_ui::{self, GamePlayUIResources},
    janggu_state_with_tick::JangguStateWithTick,
    timing_judge::TimingJudge,
};

pub(crate) fn display_notes_and_judge(
    common_context: &mut game_common_context::GameCommonContext,
    chart: &GameChart,
    timing_judge: &mut TimingJudge,
    janggu_state_with_tick: &JangguStateWithTick,
    gameplay_ui_resources: &mut GamePlayUIResources,
    processed_note_ids: &mut Vec<u64>,
    accuracy: &mut Option<NoteAccuracy>,
    accuracy_tick: &mut Option<i128>,
    hit_sounds: &[StaticSoundData; 2],
    tick_now: i128,
) {
    let kung_sound_data = hit_sounds[0].clone();
    let deok_sound_data = hit_sounds[1].clone();

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
        let new_accuracies = timing_judge.judge(&janggu_state_with_tick, tick_now as u64);

        // play hit sound when use git janggu
        if janggu_state_with_tick.궁채.is_keydown_now {
            common_context
                .audio_manager
                .play(kung_sound_data.clone())
                .expect("Failed to play kung sound");
        }
        if janggu_state_with_tick.열채.is_keydown_now {
            common_context
                .audio_manager
                .play(deok_sound_data.clone())
                .expect("Failed to play deok sound");
        }

        // if any judgement is made, display it
        if !new_accuracies.is_empty() {
            *accuracy_tick = Some(tick_now);
            *accuracy = new_accuracies.iter().map(|x| x.accuracy).max();
            for i in new_accuracies {
                processed_note_ids.push(i.note_id);
            }
        }
    }

    // judgement is visible for only 800 ms
    const ACCURACY_DISPLAY_DURATION: u32 = 800;
    if let Some(accuracy_tick_unwrapped) = *accuracy_tick {
        if tick_now - accuracy_tick_unwrapped > ACCURACY_DISPLAY_DURATION as i128 {
            *accuracy_tick = None;
        }
    }

    // draw game play ui
    draw_gameplay_ui::draw_gameplay_ui(
        &mut common_context.canvas,
        display_notes,
        UIContent {
            accuracy: if let Some(_) = accuracy_tick {
                *accuracy
            } else {
                None
            },
            accuracy_time_progress: if let Some(accuracy_time_unwrapped) = *accuracy_tick {
                Some((tick_now - accuracy_time_unwrapped) as f32 / ACCURACY_DISPLAY_DURATION as f32)
            } else {
                None
            },
            input_effect: is_input_effect_needed(&janggu_state_with_tick, tick_now),
        },
        gameplay_ui_resources,
    );
}
