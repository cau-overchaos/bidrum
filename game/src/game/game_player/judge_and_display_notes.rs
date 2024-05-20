use std::time::Duration;

use bidrum_data_struct_lib::{
    janggu::JangguFace,
    song::{GameChart, GameSong},
};
use kira::{
    sound::{
        static_sound::{StaticSoundData, StaticSoundHandle},
        PlaybackState,
    },
    tween::Tween,
    StartTime,
};

use crate::game::{
    game_common_context,
    game_player::{
        draw_gameplay_ui::{DisapreaingNoteEffectItem, DisplayedSongNote, UIContent},
        is_input_effect_needed,
        timing_judge::NoteAccuracy,
    },
};

use super::{
    draw_gameplay_ui::{self, DisapreaingNoteEffect, GamePlayUIResources, InputEffect},
    janggu_state_with_tick::JangguStateWithTick,
    timing_judge::TimingJudge,
};

pub struct EffectSoundHandles {
    left_stick: Option<StaticSoundHandle>,
    right_stick: Option<StaticSoundHandle>,
}

impl EffectSoundHandles {
    pub fn new() -> EffectSoundHandles {
        EffectSoundHandles {
            left_stick: None,
            right_stick: None,
        }
    }
}

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
    effect_sound_handles: &mut EffectSoundHandles,
    input_effect: &InputEffect,
    disappearing_notes: &mut DisapreaingNoteEffect,
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
            let play_sound = if let Some(handle) = &mut effect_sound_handles.left_stick {
                !matches!(handle.state(), PlaybackState::Playing) || handle.position() > 0.01
            } else {
                true
            };
            if play_sound {
                let new_handle = common_context
                    .audio_manager
                    .play(kung_sound_data.clone())
                    .expect("Failed to play kung sound");
                effect_sound_handles.left_stick = Some(new_handle);
            }
        }
        if janggu_state_with_tick.열채.is_keydown_now {
            let play_sound = if let Some(handle) = &mut effect_sound_handles.right_stick {
                !matches!(handle.state(), PlaybackState::Playing) || handle.position() > 0.01
            } else {
                true
            };

            if play_sound {
                let new_handle = common_context
                    .audio_manager
                    .play(deok_sound_data.clone())
                    .expect("Failed to play deok sound");
                effect_sound_handles.right_stick = Some(new_handle);
            }
        }

        // if any judgement is made, display it
        if !new_accuracies.is_empty() {
            *accuracy_tick = Some(tick_now);
            *accuracy = new_accuracies.iter().map(|x| x.accuracy).max();
            for i in new_accuracies {
                processed_note_ids.push(i.note_id);
                if !matches!(i.accuracy, NoteAccuracy::Miss) {
                    disappearing_notes.notes.push(
                        [chart.left_face.clone(), chart.right_face.clone()]
                            .concat()
                            .iter()
                            .filter(|j| j.id == i.note_id)
                            .map(|j| DisplayedSongNote {
                                face: j.face,
                                stick: j.stick,
                                distance: j.get_position(
                                    chart.bpm,
                                    chart.delay,
                                    chart.bpm * 2,
                                    (tick_now) as u64,
                                ),
                            })
                            .map(|j| DisapreaingNoteEffectItem {
                                note: j.clone(),
                                tick: tick_now,
                            })
                            .last()
                            .unwrap(),
                    );
                }
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
    disappearing_notes.base_tick = tick_now;
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
            input_effect: input_effect.clone(),
            overall_effect_tick: common_context.game_initialized_at.elapsed().as_millis(),
            disappearing_note_effects: disappearing_notes.clone(),
        },
        gameplay_ui_resources,
    );
}
