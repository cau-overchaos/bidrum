use bidrum_data_struct_lib::janggu::JangguInputState;
use bidrum_data_struct_lib::janggu::{JangguFace, JangguStick};

/// Stick state of Janggu
#[derive(Debug, Clone, Copy)]
pub struct JangguStickStateWithTick {
    /// timing when the stick is started to touch the face
    pub keydown_timing: i128,
    /// face which the stick is touching
    ///
    /// If the stick is touching nothing, the value is None.
    pub face: Option<JangguFace>,
    /// Whether it's the EXACT time the stick is started to touch the face right now
    pub is_keydown_now: bool,
}

/// Processes keyup, keypress, keydown of janggu
///
/// Note that, for ease of implementation. "None" is also interpreted as keydown
#[derive(Debug)]
pub struct JangguStateWithTick {
    pub 궁채: JangguStickStateWithTick,
    pub 열채: JangguStickStateWithTick,
}

impl JangguStickStateWithTick {
    fn empty() -> JangguStickStateWithTick {
        JangguStickStateWithTick {
            keydown_timing: 0,
            face: None,
            is_keydown_now: false,
        }
    }
    fn toggle_keydown(self, keydown_value: bool) -> JangguStickStateWithTick {
        JangguStickStateWithTick {
            keydown_timing: self.keydown_timing,
            face: self.face,
            is_keydown_now: keydown_value,
        }
    }
    fn change_keydown_timing_and_face(
        self,
        timing: i128,
        face: Option<JangguFace>,
    ) -> JangguStickStateWithTick {
        JangguStickStateWithTick {
            keydown_timing: timing,
            face: face,
            is_keydown_now: self.is_keydown_now,
        }
    }
}

impl JangguStateWithTick {
    pub(crate) fn new() -> JangguStateWithTick {
        JangguStateWithTick {
            궁채: JangguStickStateWithTick::empty(),
            열채: JangguStickStateWithTick::empty(),
        }
    }

    pub(crate) fn get_by_stick(&self, stick: JangguStick) -> JangguStickStateWithTick {
        match stick {
            JangguStick::궁채 => self.궁채,
            JangguStick::열채 => self.열채,
        }
    }

    pub(crate) fn update(&mut self, state: JangguInputState, time: i128) {
        self.궁채 = if state.궁채 == self.궁채.face {
            self.궁채.toggle_keydown(false)
        } else {
            // When user hit and take stick off the janggu, state.궁채 is None, and self.궁채.face is not None.
            // So that state is not hit event. So make toggle_keydown false
            if state.궁채 == None {
                self.궁채
                    .toggle_keydown(false)
                    .change_keydown_timing_and_face(time, None)
            } else {
                self.궁채
                    .toggle_keydown(true)
                    .change_keydown_timing_and_face(time, state.궁채)
            }
        };
        self.열채 = if state.열채 == self.열채.face {
            self.열채.toggle_keydown(false)
        } else {
            // When user hit and take stick off the janggu, state.궁채 is None, and self.궁채.face is not None.
            // So that state is not hit event. So make toggle_keydown false
            if state.열채 == None {
                self.열채
                    .toggle_keydown(false)
                    .change_keydown_timing_and_face(time, None)
            } else {
                self.열채
                    .toggle_keydown(true)
                    .change_keydown_timing_and_face(time, state.열채)
            }
        };
    }
}
