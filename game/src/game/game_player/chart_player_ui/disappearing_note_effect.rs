use super::displayed_song_note::DisplayedSongNote;

#[derive(Clone)]
pub struct DisapearingNoteEffectItem {
    pub(super) tick: i128,
    pub(super) note: DisplayedSongNote,
}

#[derive(Clone)]
pub struct DisapearingNoteEffect {
    pub(super) base_tick: i128,
    pub(super) notes: Vec<DisapearingNoteEffectItem>,
}
impl DisapearingNoteEffect {
    pub fn effect_duration() -> u128 {
        150
    }

    pub fn new() -> DisapearingNoteEffect {
        DisapearingNoteEffect {
            base_tick: 0,
            notes: vec![],
        }
    }

    pub fn push_note(&mut self, note: DisplayedSongNote, tick: i128) {
        self.notes.push(DisapearingNoteEffectItem {
            tick: tick,
            note: note,
        });
    }

    pub fn update_base_tick(&mut self, tick: i128) {
        self.base_tick = tick;
    }
}
