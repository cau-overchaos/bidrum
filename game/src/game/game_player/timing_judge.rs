use bidrum_data_struct_lib::{janggu::JangguStick, song::GameChart};

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
const OVERCHAOS_TIMING: i64 = 10;
const PERFECT_TIMING: i64 = 40;
const GREAT_TIMING: i64 = 60;
const GOOD_TIMING: i64 = 80;
const BAD_TIMING: i64 = 160;

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
    overchaos_count: u64,
    perfect_count: u64,
    great_count: u64,
    good_count: u64,
    bad_count: u64,
    miss_count: u64,
    combo: u64,
    score: u64,
    health: i64,
    max_health: u64,
}

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

        let all_note_count = chart.left_face.len() + chart.right_face.len();
        let max_health = all_note_count as u64 * 100;

        return TimingJudge {
            notes: notes,
            overchaos_count: 0,
            perfect_count: 0,
            great_count: 0,
            good_count: 0,
            bad_count: 0,
            miss_count: 0,
            combo: 0,
            score: 0,
            health: max_health as i64,
            max_health: max_health,
        };
    }

    /// Checks the notes for judgement
    /// If there's judged note by the given janggu state and timing, return the judgement result
    /// If there's no judged note, return None
    ///
    /// # Arguments
    ///   * `keydown`: the current janggu sate
    ///   * `tick_in_milliseconds` : the current time position of the song
    pub fn judge(
        &mut self,
        keydown: &JangguStateWithTick,
        tick_in_milliseconds: u64,
    ) -> Option<JudgeResult> {
        let mut processed_index: Option<usize> = None;
        let mut result = None;
        for (idx, i) in (&mut self.notes).iter_mut().enumerate() {
            let precise_timing = i.note.timing_in_ms(i.bpm.into(), i.delay);
            let difference = tick_in_milliseconds as i64 - precise_timing as i64;

            // judge the miss
            if difference > (BAD_TIMING) {
                processed_index = Some(idx);
                result = Some(NoteAccuracy::Miss);
                break;
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
            {
                Some(keydown_data.keydown_timing as u64)
            } else {
                i.hit_timing
            };

            // if it's processable note, calculate accuracy
            if let Some(hit_timing) = i.hit_timing {
                let difference_abs = (hit_timing as i64 - precise_timing as i64).abs();

                // calculte score by the accuracy
                self.score += ((f64::abs(160.0 - difference_abs.clamp(10, 160) as f64) / 160.0)
                    * 1000.0) as u64;

                let note_accuracy = note_accuracy_from_time_difference(difference_abs);

                processed_index = Some(idx);
                result = Some(note_accuracy);
                break;
            }
        }

        // if there's any judged note
        if let Some(processed_accuracy) = &result {
            let processed_note_id = self.notes.get(processed_index.unwrap()).unwrap().id;
            self.notes.remove(processed_index.unwrap());
            // increase or set combo and count
            match processed_accuracy {
                NoteAccuracy::Overchaos => {
                    self.combo += 1;
                    self.overchaos_count += 1;
                    self.health += 400;
                }
                NoteAccuracy::Perfect => {
                    self.combo += 1;
                    self.perfect_count += 1;
                    self.health += 200;
                }
                NoteAccuracy::Great => {
                    self.combo += 1;
                    self.great_count += 1;
                    self.health += 100;
                }
                NoteAccuracy::Good => {
                    self.combo += 1;
                    self.good_count += 1;
                }
                NoteAccuracy::Bad => {
                    self.combo = 0;
                    self.bad_count += 1;
                    self.health -= 100;
                }
                NoteAccuracy::Miss => {
                    // miss breaks the combo
                    self.combo = 0;
                    self.miss_count += 1;
                    self.health -= 200;
                }
            }

            // clamp the health between 0 and max_health
            self.health = self.health.clamp(0, self.max_health as i64);

            return Some(JudgeResult {
                accuracy: processed_accuracy.clone(),
                note_id: processed_note_id,
            });
        }

        // return judgement result of the judged note
        return None;
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
            score: self.score,
            health: self.health,
            max_health: self.max_health,
        };
    }
}
