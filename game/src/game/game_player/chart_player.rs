use bidrum_data_struct_lib::{
    janggu::JangguFace,
    song::{GameChart, GameNote},
};
use sdl2::{render::Canvas, video::Window};

use crate::constants::{ACCURACY_DISPLAY_DURATION, DEFAULT_BPM};

use crate::game::game_player::{
    chart_player_ui::displayed_song_note::DisplayedSongNote, timing_judge::NoteAccuracy,
};

use super::{
    chart_player_ui::{disappearing_note_effect::DisapearingNoteEffect, ChartPlayerUI},
    game_result::GameResult,
    janggu_state_with_tick::JangguStateWithTick,
    timing_judge::TimingJudge,
};

struct ProcessedNote {
    processed_note_id: u64,
    processed_at_tick: i128,
    accuracy: NoteAccuracy,
}

pub struct ChartPlayer<'a> {
    chart: GameChart,
    timing_judge: TimingJudge,
    ui: ChartPlayerUI<'a>,
    processed_notes: Vec<ProcessedNote>,
    accuracy: Option<(NoteAccuracy, i128)>,
}

impl ChartPlayer<'_> {
    pub fn new(
        chart: GameChart,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> ChartPlayer {
        ChartPlayer {
            chart: chart.clone(),
            timing_judge: TimingJudge::new(&chart),
            ui: ChartPlayerUI::new(texture_creator),
            processed_notes: vec![],
            accuracy: None,
        }
    }

    fn append_disappearing_notes(&mut self, tick: i128, note_ids: Vec<u64>) {
        let left_face = self.chart.left_face.clone();
        let right_face = self.chart.right_face.clone();
        let faces = [left_face, right_face].concat();
        let disappearing_notes = faces.iter().filter(|i| {
            note_ids.contains(&i.id)
                && self.processed_notes.iter().any(|j| {
                    j.processed_note_id == i.id
                        && j.processed_at_tick.abs_diff(tick.into())
                            < DisapearingNoteEffect::effect_duration()
                })
        });

        for i in disappearing_notes {
            self.ui
                .disappearing_note_effects
                .push_note(self.get_display_note(i, tick), tick as i128)
        }
    }

    pub fn judge(&mut self, janggu: &JangguStateWithTick, tick: i128) {
        let new_accuracies = self.timing_judge.judge(janggu, tick as u64);

        if !new_accuracies.is_empty() {
            self.accuracy = Some((
                new_accuracies.iter().map(|x| x.accuracy).max().unwrap(),
                tick,
            ));
            for i in new_accuracies.clone() {
                self.processed_notes.push(ProcessedNote {
                    processed_note_id: i.note_id,
                    processed_at_tick: tick,
                    accuracy: i.accuracy,
                });

                if !matches!(i.accuracy, NoteAccuracy::Miss) {}
            }
        }

        self.append_disappearing_notes(
            tick,
            new_accuracies
                .iter()
                .filter(|x| !matches!(x.accuracy, NoteAccuracy::Miss))
                .map(|x| x.note_id)
                .collect(),
        );
    }

    fn get_display_note(&self, note: &GameNote, tick: i128) -> DisplayedSongNote {
        DisplayedSongNote {
            face: note.face,
            stick: note.stick,
            distance: note.get_position(
                self.chart.bpm,
                self.chart.delay,
                self.chart.bpm * DEFAULT_BPM,
                tick as u64,
            ),
        }
    }

    fn processed_note_ids(&self) -> Vec<u64> {
        self.processed_notes
            .iter()
            .map(|x| x.processed_note_id)
            .collect()
    }

    fn get_display_notes(&self, tick_now: u64) -> Vec<DisplayedSongNote> {
        let mut display_notes = Vec::<DisplayedSongNote>::new();
        // get positions of the notes
        for i in &self.chart.left_face {
            if !self.processed_note_ids().contains(&i.id) {
                display_notes.push(DisplayedSongNote {
                    face: JangguFace::궁편,
                    stick: i.stick,
                    distance: i.get_position(
                        self.chart.bpm,
                        self.chart.delay,
                        self.chart.bpm * DEFAULT_BPM,
                        tick_now,
                    ),
                });
            }
        }

        for i in &self.chart.right_face {
            if !self.processed_note_ids().contains(&i.id) {
                display_notes.push(DisplayedSongNote {
                    face: JangguFace::열편,
                    stick: i.stick,
                    distance: i.get_position(
                        self.chart.bpm,
                        self.chart.delay,
                        self.chart.bpm * DEFAULT_BPM,
                        tick_now,
                    ),
                });
            }
        }

        display_notes
    }

    pub fn draw(
        &mut self,
        tick: i128,
        canvas: &mut Canvas<Window>,
        overall_tick: u128,
        janggu_state_with_tick: &JangguStateWithTick,
    ) {
        // set ui accuracy effect
        self.ui.accuracy = None;
        self.ui.accuracy_time_progress = None;
        if let Some(accuracy) = self.accuracy {
            if accuracy.1.abs_diff(tick) > ACCURACY_DISPLAY_DURATION.into() {
                self.accuracy = None;
            } else {
                self.ui.accuracy = Some(accuracy.0);
                self.ui.accuracy_time_progress =
                    Some(accuracy.1.abs_diff(tick) as f32 / ACCURACY_DISPLAY_DURATION as f32)
            }
        }

        // draw game play ui
        self.ui.disappearing_note_effects.update_base_tick(tick);
        self.ui.input_effect.update(janggu_state_with_tick, tick);
        self.ui.notes = self.get_display_notes(tick as u64);
        self.ui.overall_effect_tick = overall_tick;
        self.ui.draw(canvas);
    }

    pub fn game_result(&self) -> GameResult {
        self.timing_judge.get_game_result()
    }
}
