use std::os::unix::process;

use crate::janggu::JangguState;

use super::songs::{GameNote, GameNoteTrack};

#[derive(Debug, Clone, Copy)]
pub(crate) enum NoteAccuracy {
    overchaos,
    perfect,
    great,
    good,
    bad,
    miss
}

const OVERCHAOS_TIMING: i64 = 10;
const PERFECT_TIMING: i64 = 40;
const GREAT_TIMING: i64 = 60;
const GOOD_TIMING :i64 = 80;
const BAD_TIMING: i64 = 160;

struct NoteForProcessing {
    note: GameNote,
    accuracy: Option<NoteAccuracy>,
    bpm: u32,
    delay: u64,
}

pub(crate) struct NoteProcessor {
    notes: Vec<NoteForProcessing>,
    overchaos_count : u64,
    perfect_count : u64,
    great_count : u64,
    good_count : u64,
    bad_count : u64,
    miss_count : u64,
    combo: u64,
}

impl NoteProcessor {
    pub fn new(tracks: &Vec<GameNoteTrack>) -> NoteProcessor {
        let mut notes = Vec::<NoteForProcessing>::new();
        for i in tracks {
            for j in &i.notes {
                notes.push(NoteForProcessing {
                    note: j.clone(),
                    accuracy: None,
                    bpm: i.bpm,
                    delay: i.delay
                });
            }
        }

        notes.sort_by(|a, b| a.note.beat().cmp(
            &b.note.beat()
        ));

        return NoteProcessor {
            notes: notes,
            bad_count: 0,
            combo: 0,
            good_count: 0,
            great_count: 0,
            miss_count: 0,
            overchaos_count: 0,
            perfect_count: 0
        }
    }

    pub fn process(&mut self, keydown: JangguState, tick_in_milliseconds: u64) -> Option<NoteAccuracy> {
        
        let mut processed_index: Option<usize> = None;
        let mut result = None;
        for (idx, i) in (&self.notes).iter().enumerate() {
            let end_time = i.note.end_time_in_ms(i.bpm.into(), i.delay);
            let difference = tick_in_milliseconds as i64 - end_time as i64;

            println!("difference={}", difference);

            // MISS
            if difference > (BAD_TIMING) {
                processed_index = Some(idx);
                result = Some(NoteAccuracy::miss);
                break;
            }

            if i.note.궁채 != keydown.궁채 || i.note.열채 != keydown.북채 {
                continue;
            }
            
            let difference_abs = difference.abs();
            let note_accuracy = if difference_abs <= OVERCHAOS_TIMING {
                Some(NoteAccuracy::overchaos)
            } else if difference_abs <= PERFECT_TIMING {
                Some(NoteAccuracy::perfect)
            } else if difference_abs <= GREAT_TIMING {
                Some(NoteAccuracy::great)
            } else if difference_abs <= GOOD_TIMING {
                Some(NoteAccuracy::good)
            } else if difference_abs <= BAD_TIMING {
                Some(NoteAccuracy::bad)
            } else {
                None
            };
            
            if let Some(accuracy_unwrapped) = note_accuracy {
                processed_index = Some(idx);
                result = Some(accuracy_unwrapped);
                break;
            }
        }

        if let Some(processed_accuracy) = &result {
            self.notes.remove(processed_index.unwrap());
            match processed_accuracy {
                NoteAccuracy::overchaos => {
                    self.combo += 1;
                    self.overchaos_count += 1;
                },
                NoteAccuracy::perfect => {
                    self.combo += 1;
                    self.perfect_count += 1;
                },
                NoteAccuracy::great => {
                    self.combo += 1;
                    self.great_count += 1;
                },
                NoteAccuracy::good => {
                    self.combo += 1;
                    self.good_count += 1;
                },
                NoteAccuracy::bad => {
                    self.combo += 1;
                    self.bad_count += 1;
                },
                NoteAccuracy::miss => {
                    self.combo = 0;
                },
            }
        }

        return result;
    }
}