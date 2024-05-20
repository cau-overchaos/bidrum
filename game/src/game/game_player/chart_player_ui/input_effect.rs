use bidrum_data_struct_lib::janggu::JangguFace;

use crate::game::game_player::janggu_state_with_tick::JangguStateWithTick;

#[derive(Clone)]
pub struct InputEffectItem {
    pub(super) pressed: bool,
    pub(super) keydown_timing: Option<i128>,
}

#[derive(Clone)]
pub struct InputEffect {
    pub(super) left_face: InputEffectItem,
    pub(super) right_face: InputEffectItem,
    pub(super) base_tick: i128,
}

impl InputEffect {
    pub fn new() -> InputEffect {
        return InputEffect {
            base_tick: 0,
            left_face: InputEffectItem {
                pressed: false,
                keydown_timing: None,
            },
            right_face: InputEffectItem {
                pressed: false,
                keydown_timing: None,
            },
        };
    }
    pub fn update(&mut self, janggu: &JangguStateWithTick, tick_now: i128) {
        self.base_tick = tick_now;

        // Process left face
        self.left_face.pressed = false;
        if janggu.궁채.face.is_some_and(|x| x == JangguFace::궁편) {
            self.left_face.pressed = true;
            self.left_face.keydown_timing =
                Some(if let Some(prev) = self.left_face.keydown_timing {
                    prev.max(janggu.궁채.keydown_timing)
                } else {
                    janggu.궁채.keydown_timing
                })
        }
        if janggu.열채.face.is_some_and(|x| x == JangguFace::궁편) {
            self.left_face.pressed = true;
            self.left_face.keydown_timing =
                Some(if let Some(prev) = self.left_face.keydown_timing {
                    prev.max(janggu.열채.keydown_timing)
                } else {
                    janggu.열채.keydown_timing
                })
        }

        // Process right face
        self.right_face.pressed = false;
        if janggu.궁채.face.is_some_and(|x| x == JangguFace::열편) {
            self.right_face.pressed = true;
            self.right_face.keydown_timing =
                Some(if let Some(prev) = self.right_face.keydown_timing {
                    prev.max(janggu.궁채.keydown_timing)
                } else {
                    janggu.궁채.keydown_timing
                })
        }
        if janggu.열채.face.is_some_and(|x| x == JangguFace::열편) {
            self.right_face.pressed = true;
            self.right_face.keydown_timing =
                Some(if let Some(prev) = self.right_face.keydown_timing {
                    prev.max(janggu.열채.keydown_timing)
                } else {
                    janggu.열채.keydown_timing
                })
        }
    }
}
