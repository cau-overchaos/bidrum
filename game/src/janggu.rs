
// Korean looks more intutitive than english...
#[derive(Debug, Clone, Copy)]
pub enum DrumPane {
    채편,
    북편,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct JangguState {
    pub 궁채: Option<DrumPane>,
    pub 북채: Option<DrumPane>,
}
