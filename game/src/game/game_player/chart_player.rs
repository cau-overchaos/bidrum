use bidrum_data_struct_lib::{
    janggu::JangguFace,
    song::{GameChart, GameNote},
};
use num_rational::Rational64;
use sdl2::{render::Canvas, video::Window};

use crate::constants::{ACCURACY_DISPLAY_DURATION, DEFAULT_BPM};

use crate::game::game_player::{
    chart_player_ui::displayed_song_note::DisplayedSongNote, timing_judge::NoteAccuracy,
};

use super::{
    chart_player_ui::{
        disappearing_note_effect::DisapearingNoteEffect, BeatGuideline, ChartPlayerUI,
    },
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
    combo: Option<u64>,
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
            combo: None,
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

    fn beat_guideline(&self, tick: i128) -> Option<BeatGuideline> {
        if tick < 0 {
            return None;
        }

        // bpm = beat / minute
        // minute-per-beat = 1 / bpm
        // timing-in-minute = beat * minute-per-beat
        // timing-in-millisecond = timing-in-minute (minute) * ( 60000(millisecond) / 1(minute) )
        // timing = timing-in-millisecond
        let timing_of_one_beat = Rational64::new(60000, self.chart.bpm as i64);

        // beat_per_millisecond = (display_bpm / 60000)
        // millisecond_per_beat = 1/ beat_per_millisecond
        // speed = 1 / millisecond_per_beat
        let speed_ratio = Rational64::new((self.chart.bpm * DEFAULT_BPM) as i64, 60000);

        // convert the ratio into floating value
        let speed = *speed_ratio.numer() as f64 / *speed_ratio.denom() as f64;

        let length =
            (*timing_of_one_beat.numer() as f64 / *timing_of_one_beat.denom() as f64) * speed;

        let position = {
            let timing_of_one_beat =
                *timing_of_one_beat.numer() as f64 / *timing_of_one_beat.denom() as f64;
            let mut position = tick % timing_of_one_beat as i128;
            while position < 0 {
                position += timing_of_one_beat as i128;
            }

            position = length as i128 - position;
            length as f64 * (position as f64 / timing_of_one_beat)
        };

        println!("beat_guideline: len = {}, pos = {}", length, position);
        Some(BeatGuideline { length, position })
    }

    pub fn draw(
        &mut self,
        tick: i128,
        canvas: &mut Canvas<Window>,
        overall_tick: u128,
        janggu_state_with_tick: &JangguStateWithTick,
    ) {
        // set ui accuracy and combo effect
        self.ui.accuracy = None;
        self.ui.combo = None;
        self.ui.accuracy_and_combo_time_progress = None;
        if let Some(accuracy) = self.accuracy {
            if accuracy.1.abs_diff(tick) > ACCURACY_DISPLAY_DURATION.into() {
                self.accuracy = None;
                self.combo = None;
            } else {
                self.ui.accuracy = Some(accuracy.0);
                self.ui.combo = Some(self.timing_judge.get_game_result().combo);
                self.ui.accuracy_and_combo_time_progress =
                    Some(accuracy.1.abs_diff(tick) as f32 / ACCURACY_DISPLAY_DURATION as f32)
            }
        }

        // draw game play ui
        if tick >= 0 {
            self.ui.disappearing_note_effects.update_base_tick(tick);
            self.ui.input_effect.update(janggu_state_with_tick, tick);
            self.ui.notes = self.get_display_notes(tick as u64);
            self.ui.beat_guideline = self.beat_guideline(tick);
        }
        self.ui.overall_effect_tick = overall_tick;
        self.ui.draw(canvas);
    }

    pub fn game_result(&self) -> GameResult {
        self.timing_judge.get_game_result()
    }
}
