use bidrum_data_struct_lib::song::{GameChart, GameHatNote};

use crate::constants::HAT_TIMING;

use super::NoteAccuracy;

// Combination of GameNoteTrack and GameNote
struct HatNoteForProcessing {
    note: GameHatNote,
    bpm: u32,
    delay: u64,
    id: u64,
}

/// Judges timing accuracy
pub(super) struct HatTimingJudge {
    notes: Vec<HatNoteForProcessing>,
}

#[derive(Clone)]
pub(super) struct HatJudgeResult {
    pub accuracy: NoteAccuracy,
    pub note_id: u64,
}

impl HatTimingJudge {
    /// Creates new TimingJudge with collection of notes
    pub fn new(chart: &GameChart) -> HatTimingJudge {
        // flattens GameNote and GameNoteTrack into NoteForProcessing
        let mut notes = Vec::<HatNoteForProcessing>::new();
        for j in &chart.hats {
            notes.push(HatNoteForProcessing {
                note: j.clone(),
                bpm: chart.bpm,
                delay: chart.delay,
                id: j.id,
            });
        }

        // sort the notes by their precise timings
        notes.sort_by(|a, b| {
            a.note
                .timing_in_ms(a.bpm, a.delay)
                .cmp(&b.note.timing_in_ms(b.bpm, b.delay))
        });

        return HatTimingJudge { notes: notes };
    }

    /// Checks the notes for judgement
    /// If there's judged note by the given janggu state and timing, return the judged elements
    /// If there's no judged note, return empty vector
    ///
    /// # Arguments
    ///   * `spinning`: the current hat sate
    ///   * `tick_in_milliseconds` : the current time position of the song
    pub fn judge(&mut self, spinning: bool, tick_in_milliseconds: u64) -> Vec<HatJudgeResult> {
        let mut judged_notes = vec![];

        // if sticks are not keydown, there's no need to process the stick
        for i in &mut self.notes {
            let precise_timing = i.note.timing_in_ms(i.bpm.into(), i.delay);
            let difference = tick_in_milliseconds as i64 - precise_timing as i64;

            // judge the miss
            if difference > (HAT_TIMING) {
                judged_notes.push(HatJudgeResult {
                    note_id: i.id,
                    accuracy: NoteAccuracy::Miss,
                });
                continue;
            }

            // skip not-yet notes
            if difference < -HAT_TIMING {
                continue;
            }

            // process the timings
            if spinning {
                judged_notes.push(HatJudgeResult {
                    note_id: i.id,
                    accuracy: NoteAccuracy::Perfect,
                });

                break;
            }
        }

        // process combo and delete judged notes
        for i in &judged_notes {
            // delete judged note
            self.notes
                .remove(self.notes.iter().position(|x| x.id == i.note_id).unwrap());
        }

        // return judgement result of the judged note
        judged_notes
    }
}
