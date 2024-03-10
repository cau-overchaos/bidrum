use bidrum_data_struct_lib::janggu::JangguStick;

use super::janggu_state_with_tick::JangguStateWithTick;
use bidrum_data_struct_lib::song::{GameNote, GameNoteTrack};

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
    궁채_timing: Option<u64>,
    열채_timing: Option<u64>,
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
}

pub(crate) struct JudgeResult {
    pub accuracy: NoteAccuracy,
    pub note_id: u64,
}

fn note_accuracy_from_time_difference(difference: i64) -> NoteAccuracy {
    let difference_abs = difference.abs();
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
    pub fn new(tracks: &Vec<GameNoteTrack>) -> TimingJudge {
        // flattens GameNote and GameNoteTrack into NoteForProcessing
        let mut notes = Vec::<NoteForProcessing>::new();
        for i in tracks {
            for j in &i.notes {
                notes.push(NoteForProcessing {
                    note: j.clone(),
                    bpm: i.bpm,
                    delay: i.delay,
                    id: j.id,
                    궁채_timing: None,
                    열채_timing: None,
                });
            }
        }

        // sort the notes by their precise timings
        notes.sort_by(|a, b| {
            a.note
                .timing_in_ms(a.bpm, a.delay)
                .cmp(&b.note.timing_in_ms(b.bpm, b.delay))
        });

        return TimingJudge {
            notes: notes,
            bad_count: 0,
            combo: 0,
            good_count: 0,
            great_count: 0,
            miss_count: 0,
            overchaos_count: 0,
            perfect_count: 0,
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
            let 궁채_new_timing =
                if keydown.is_keydown(JangguStick::궁채) && i.note.궁채 == keydown.궁채.1 {
                    Some(keydown.궁채.0 as u64)
                } else {
                    i.궁채_timing
                };
            i.궁채_timing = if matches!(i.note.열채, Some(_)) {
                // the note is 쿵 kind
                if let Some(열채_timing_saved) = i.열채_timing {
                    if 열채_timing_saved == keydown.열채.0 as u64 && i.note.열채 == keydown.열채.1
                    {
                        // if the another stick input was already processed
                        // the another stick should be pressed
                        궁채_new_timing
                    } else {
                        i.궁채_timing
                    }
                } else {
                    궁채_new_timing
                }
            } else {
                // the note is not 쿵 kind
                궁채_new_timing
            };

            let 열채_new_timing =
                if keydown.is_keydown(JangguStick::열채) && i.note.열채 == keydown.열채.1 {
                    Some(keydown.열채.0 as u64)
                } else {
                    i.열채_timing
                };
            i.열채_timing = if matches!(i.note.궁채, Some(_)) {
                // the note is 쿵 kind
                if let Some(궁채_timing_saved) = i.궁채_timing {
                    if 궁채_timing_saved == keydown.궁채.0 as u64 && i.note.궁채 == keydown.궁채.1
                    {
                        // if the another stick input was already processed
                        // the another stick should be pressed
                        열채_new_timing
                    } else {
                        i.열채_timing
                    }
                } else {
                    열채_new_timing
                }
            } else {
                // the note is not 쿵 kind
                열채_new_timing
            };

            // if it's processable note, calculate accuracy
            if (matches!(i.note.궁채, None) || matches!(i.궁채_timing, Some(_)))
                && (matches!(i.note.열채, None) || matches!(i.열채_timing, Some(_)))
            {
                let note_accuracy_궁채 = if let Some(input_timing) = i.궁채_timing {
                    Some(note_accuracy_from_time_difference(
                        input_timing as i64 - precise_timing as i64,
                    ))
                } else {
                    None
                };

                let note_accuracy_열채 = if let Some(input_timing) = i.열채_timing {
                    Some(note_accuracy_from_time_difference(
                        input_timing as i64 - precise_timing as i64,
                    ))
                } else {
                    None
                };

                let note_accuracy = if let Some(note_accuracy_궁채_unwrapped) = note_accuracy_궁채
                {
                    if let Some(note_accuracy_열채_unwrapped) = note_accuracy_열채 {
                        Some(std::cmp::max(
                            note_accuracy_궁채_unwrapped,
                            note_accuracy_열채_unwrapped,
                        ))
                    } else {
                        Some(note_accuracy_궁채_unwrapped)
                    }
                } else if let Some(note_accuracy_열채_unwrapped) = note_accuracy_열채 {
                    Some(note_accuracy_열채_unwrapped)
                } else {
                    // unreachable code
                    panic!()
                };

                // if any judgement is done, break the loop
                if let Some(accuracy_unwrapped) = note_accuracy {
                    processed_index = Some(idx);
                    result = Some(accuracy_unwrapped);
                    break;
                }
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
                }
                NoteAccuracy::Perfect => {
                    self.combo += 1;
                    self.perfect_count += 1;
                }
                NoteAccuracy::Great => {
                    self.combo += 1;
                    self.great_count += 1;
                }
                NoteAccuracy::Good => {
                    self.combo += 1;
                    self.good_count += 1;
                }
                NoteAccuracy::Bad => {
                    self.combo += 1;
                    self.bad_count += 1;
                }
                NoteAccuracy::Miss => {
                    // miss breaks the combo
                    self.combo = 0;
                    self.miss_count += 1;
                }
            }

            return Some(JudgeResult {
                accuracy: processed_accuracy.clone(),
                note_id: processed_note_id,
            });
        }

        // return judgement result of the judged note
        return None;
    }
}
