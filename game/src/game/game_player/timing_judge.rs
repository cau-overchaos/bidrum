mod hat_timing_judge;

use bidrum_data_struct_lib::{janggu::JangguStick, song::GameChart};

use self::hat_timing_judge::HatTimingJudge;

use super::{game_result::GameResult, janggu_state_with_tick::JangguStateWithTick};
use bidrum_data_struct_lib::song::GameNote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NoteAccuracy {
    /// 1st (the highest accuracy)
    Overchaos,
    /// 2nd
    Perfect,
    /// 3rd
    Great,
    /// 4th
    Good,
    /// 5th (the lowest accuracy)
    Bad,
    /// miss
    Miss,
}

// timings for accuracy judgement
// e.g. 10 means -10ms ~ +10ms
use crate::constants::{
    BAD_COMBO, GOOD_COMBO, GREAT_COMBO, HAT_SCORE, MISS_COMBO, OVERCHAOS_COMBO, PERFECT_COMBO,
};
use crate::constants::{
    BAD_HEALTH, DEFAULT_HEALTH, GOOD_HEALTH, GREAT_HEALTH, MISS_HEALTH, OVERCHAOS_HEALTH,
    PERFECT_HEALTH,
};
use crate::constants::{BAD_TIMING, GOOD_TIMING, GREAT_TIMING, OVERCHAOS_TIMING, PERFECT_TIMING};

// Combination of GameNoteTrack and GameNote
struct NoteForProcessing {
    note: GameNote,
    bpm: u32,
    delay: u64,
    id: u64,
    hit_timing: Option<u64>,
}

/// Judges timing accuracy
pub(crate) struct TimingJudge {
    notes: Vec<NoteForProcessing>,
    hat_judge: HatTimingJudge,
    overchaos_count: u64,
    perfect_count: u64,
    great_count: u64,
    good_count: u64,
    bad_count: u64,
    miss_count: u64,
    combo: u64,
    max_combo: u64,
    score: u64,
    health: i64,
    max_health: u64,
}

#[derive(Clone)]
pub(crate) struct JudgeResult {
    pub accuracy: NoteAccuracy,
    pub note_id: u64,
}

fn note_accuracy_from_time_difference(difference_abs: i64) -> NoteAccuracy {
    if difference_abs <= OVERCHAOS_TIMING {
        NoteAccuracy::Overchaos
    } else if difference_abs <= PERFECT_TIMING {
        NoteAccuracy::Perfect
    } else if difference_abs <= GREAT_TIMING {
        NoteAccuracy::Great
    } else if difference_abs <= GOOD_TIMING {
        NoteAccuracy::Good
    } else if difference_abs <= BAD_TIMING {
        NoteAccuracy::Bad
    } else {
        NoteAccuracy::Miss
    }
}

impl TimingJudge {
    /// Creates new TimingJudge with collection of notes
    pub fn new(chart: &GameChart) -> TimingJudge {
        // flattens GameNote and GameNoteTrack into NoteForProcessing
        let mut notes = Vec::<NoteForProcessing>::new();
        for j in &chart.left_face {
            notes.push(NoteForProcessing {
                note: j.clone(),
                bpm: chart.bpm,
                delay: chart.delay,
                id: j.id,
                hit_timing: None,
            });
        }
        for j in &chart.right_face {
            notes.push(NoteForProcessing {
                note: j.clone(),
                bpm: chart.bpm,
                delay: chart.delay,
                id: j.id,
                hit_timing: None,
            });
        }

        // sort the notes by their precise timings
        notes.sort_by(|a, b| {
            a.note
                .timing_in_ms(a.bpm, a.delay)
                .cmp(&b.note.timing_in_ms(b.bpm, b.delay))
        });

        let hat_judge = HatTimingJudge::new(chart);

        return TimingJudge {
            notes: notes,
            overchaos_count: 0,
            perfect_count: 0,
            great_count: 0,
            good_count: 0,
            bad_count: 0,
            miss_count: 0,
            combo: 0,
            max_combo: 0,
            score: 0,
            health: DEFAULT_HEALTH as i64,
            max_health: DEFAULT_HEALTH,
            hat_judge: hat_judge,
        };
    }

    /// Checks the notes for judgement
    /// If there's judged note by the given janggu state and timing, return the judged elements
    /// If there's no judged note, return empty vector
    ///
    /// # Arguments
    ///   * `keydown`: the current janggu sate
    ///   * `tick_in_milliseconds` : the current time position of the song
    pub fn judge(
        &mut self,
        keydown: &JangguStateWithTick,
        spinning: bool,
        tick_in_milliseconds: u64,
    ) -> Vec<JudgeResult> {
        let mut judged_notes = vec![];

        // process hat notes first
        let hat_judge_result = self.hat_judge.judge(spinning, tick_in_milliseconds);
        for i in hat_judge_result
            .iter()
            .filter(|x| !matches!(x.accuracy, NoteAccuracy::Miss))
        {
            self.score += HAT_SCORE;
        }

        // if sticks are not keydown, there's no need to process the stick
        let mut processed_left_stick = false;
        let mut processed_right_stick = false;
        for i in &mut self.notes {
            // continue if two sticks are processed
            if processed_left_stick && processed_right_stick {
                break;
            }

            let precise_timing = i.note.timing_in_ms(i.bpm.into(), i.delay);
            let difference = tick_in_milliseconds as i64 - precise_timing as i64;

            // judge the miss
            if difference > (BAD_TIMING) {
                judged_notes.push(JudgeResult {
                    note_id: i.id,
                    accuracy: NoteAccuracy::Miss,
                });
                continue;
            }

            // skip not-yet notes
            if difference < -BAD_TIMING {
                continue;
            }

            // process the timings
            let keydown_data = match i.note.stick {
                JangguStick::궁채 => keydown.궁채,
                JangguStick::열채 => keydown.열채,
            };
            i.hit_timing = if keydown.get_by_stick(i.note.stick).is_keydown_now
                && keydown_data.face.is_some_and(|x| x == i.note.face)
                && !(match i.note.stick {
                    JangguStick::궁채 => processed_left_stick,
                    JangguStick::열채 => processed_right_stick,
                }) {
                match i.note.stick {
                    JangguStick::궁채 => {
                        processed_left_stick = true;
                    }
                    JangguStick::열채 => {
                        processed_right_stick = true;
                    }
                }
                Some(keydown_data.keydown_timing as u64)
            } else {
                i.hit_timing
            };

            // if it's processable note, calculate accuracy
            if let Some(hit_timing) = i.hit_timing {
                let difference_abs = (hit_timing as i64 - precise_timing as i64).abs();

                // calculte score by the accuracy
                self.score += ((f64::abs(
                    BAD_TIMING as f64 - difference_abs.clamp(OVERCHAOS_TIMING, BAD_TIMING) as f64,
                ) / (BAD_TIMING - OVERCHAOS_TIMING) as f64)
                    * 1000.0) as u64;

                let note_accuracy = note_accuracy_from_time_difference(difference_abs);

                judged_notes.push(JudgeResult {
                    note_id: i.id,
                    accuracy: note_accuracy,
                });
            }
        }

        // process combo and delete judged notes
        for i in &judged_notes {
            // delete judged note
            self.notes
                .remove(self.notes.iter().position(|x| x.id == i.note_id).unwrap());

            let is_health_zero = self.health == 0;
            // increase or set combo and count
            match i.accuracy {
                NoteAccuracy::Overchaos => {
                    if OVERCHAOS_COMBO == 0 {
                        self.max_combo = self.max_combo.max(self.combo);
                        self.combo = 0;
                    } else {
                        self.combo += OVERCHAOS_COMBO;
                    }
                    self.health += OVERCHAOS_HEALTH;
                    self.overchaos_count += 1;
                }
                NoteAccuracy::Perfect => {
                    if PERFECT_COMBO == 0 {
                        self.max_combo = self.max_combo.max(self.combo);
                        self.combo = 0;
                    } else {
                        self.combo += PERFECT_COMBO;
                    }
                    self.health += PERFECT_HEALTH;
                    self.perfect_count += 1;
                }
                NoteAccuracy::Great => {
                    if GREAT_COMBO == 0 {
                        self.max_combo = self.max_combo.max(self.combo);
                        self.combo = 0;
                    } else {
                        self.combo += GREAT_COMBO;
                    }
                    self.health += GREAT_HEALTH;
                    self.great_count += 1;
                }
                NoteAccuracy::Good => {
                    if GOOD_COMBO == 0 {
                        self.max_combo = self.max_combo.max(self.combo);
                        self.combo = 0;
                    } else {
                        self.combo += GOOD_COMBO;
                    }
                    self.health += GOOD_HEALTH;
                    self.good_count += 1;
                }
                NoteAccuracy::Bad => {
                    if BAD_COMBO == 0 {
                        self.max_combo = self.max_combo.max(self.combo);
                        self.combo = 0;
                    } else {
                        self.combo += BAD_COMBO;
                    }
                    self.health += BAD_HEALTH;
                    self.bad_count += 1;
                }
                NoteAccuracy::Miss => {
                    if MISS_COMBO == 0 {
                        self.max_combo = self.max_combo.max(self.combo);
                        self.combo = 0;
                    } else {
                        self.combo += MISS_COMBO;
                    }
                    self.health += MISS_HEALTH;
                    self.miss_count += 1;
                }
            }

            // check if the health is zero -> already died
            if is_health_zero {
                self.health = 0;
            } else {
                // clamp the health between 0 and max_health
                self.health = self.health.clamp(0, self.max_health as i64);
            }
        }

        // return judgement result of the judged note
        judged_notes
    }

    /// Creates game result
    pub fn get_game_result(&self) -> GameResult {
        return GameResult {
            overchaos_count: self.overchaos_count,
            perfect_count: self.perfect_count,
            great_count: self.great_count,
            good_count: self.good_count,
            bad_count: self.bad_count,
            miss_count: self.miss_count,
            combo: self.combo,
            max_combo: self.max_combo,
            score: self.score,
            health: self.health,
            max_health: self.max_health,
        };
    }
}
