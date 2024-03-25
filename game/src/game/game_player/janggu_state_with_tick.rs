use bidrum_data_struct_lib::janggu::JangguInputState;
use bidrum_data_struct_lib::janggu::{JangguFace, JangguStick};

/// Processes keyup, keypress, keydown of janggu
///
/// First item of the tuple means the keydown time, second item means the pressed key
/// Note that, for ease of implementation. "None" is also interpreted as keydown
/// For example, if the player release the 궁채 at time 120, the value of 궁채 is (120, None)
#[derive(Debug)]
pub(crate) struct JangguStateWithTick {
    pub 궁채: (i128, Option<JangguFace>),
    pub 열채: (i128, Option<JangguFace>),
    궁채_keydown: bool,
    열채_keydown: bool,
}

impl JangguStateWithTick {
    pub(crate) fn new() -> JangguStateWithTick {
        JangguStateWithTick {
            궁채: (0, None),
            열채: (0, None),
            궁채_keydown: false,
            열채_keydown: false,
        }
    }

    pub(crate) fn is_keydown(&self, stick: JangguStick) -> bool {
        match stick {
            JangguStick::궁채 => self.궁채_keydown,
            JangguStick::열채 => self.열채_keydown,
        }
    }

    pub(crate) fn update(&mut self, state: JangguInputState, time: i128) {
        self.궁채 = if state.궁채 == self.궁채.1 {
            self.궁채_keydown = false;
            self.궁채
        } else {
            self.궁채_keydown = true;
            (time, state.궁채)
        };
        self.열채 = if state.열채 == self.열채.1 {
            self.열채_keydown = false;
            self.열채
        } else {
            self.열채_keydown = true;
            (time, state.열채)
        };
    }
}
