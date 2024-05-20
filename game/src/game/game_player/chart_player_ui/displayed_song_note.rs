use bidrum_data_struct_lib::janggu::{JangguFace, JangguStick};

#[derive(Clone)]
pub struct DisplayedSongNote {
    pub distance: f64,
    pub face: JangguFace,
    pub stick: JangguStick,
}
