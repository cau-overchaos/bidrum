/// Game reuslt
pub(crate) struct GameResult {
    pub overchaos_count: u64,
    pub perfect_count: u64,
    pub great_count: u64,
    pub good_count: u64,
    pub bad_count: u64,
    pub miss_count: u64,
    pub combo: u64,
    pub max_combo: u64,
    pub score: u64,
    pub health: i64,
    pub max_health: u64,
}

impl GameResult {
    pub fn total_judged_note_count(&self) -> u64 {
        return self.overchaos_count
            + self.perfect_count
            + self.great_count
            + self.good_count
            + self.bad_count
            + self.miss_count;
    }
}
