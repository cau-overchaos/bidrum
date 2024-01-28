use crate::janggu::JangguState;

use super::songs::{GameNote, GameNoteTrack};

#[derive(Debug, Clone, Copy)]
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
        keydown: JangguState,
        tick_in_milliseconds: u64,
    ) -> Option<NoteAccuracy> {
        let mut processed_index: Option<usize> = None;
        let mut result = None;
        for (idx, i) in (&self.notes).iter().enumerate() {
            let precise_timing = i.note.timing_in_ms(i.bpm.into(), i.delay);
            let difference = tick_in_milliseconds as i64 - precise_timing as i64;

            // judge the miss
            if difference > (BAD_TIMING) {
                processed_index = Some(idx);
                result = Some(NoteAccuracy::Miss);
                break;
            }

            // not a processable note
            if i.note.궁채 != keydown.궁채 || i.note.북채 != keydown.북채 {
                continue;
            }

            // if it's processable note, calculate accuracy
            let difference_abs = difference.abs();
            let note_accuracy = if difference_abs <= OVERCHAOS_TIMING {
                Some(NoteAccuracy::Overchaos)
            } else if difference_abs <= PERFECT_TIMING {
                Some(NoteAccuracy::Perfect)
            } else if difference_abs <= GREAT_TIMING {
                Some(NoteAccuracy::Great)
            } else if difference_abs <= GOOD_TIMING {
                Some(NoteAccuracy::Good)
            } else if difference_abs <= BAD_TIMING {
                Some(NoteAccuracy::Bad)
            } else {
                // Miss is calculated before this if-elif-...-else logic.
                None
            };

            // if any judgement is done, break the loop
            if let Some(accuracy_unwrapped) = note_accuracy {
                processed_index = Some(idx);
                result = Some(accuracy_unwrapped);
                break;
            }
        }

        // if there's any judged note
        if let Some(processed_accuracy) = &result {
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
        }

        // return judgement result of the judged note
        return result;
    }
}
