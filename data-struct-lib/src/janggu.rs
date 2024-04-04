use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum JangguStick {
    궁채,
    열채,
}

// Korean looks more intutitive than english...
/// 장구의 치는 곳
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum JangguFace {
    궁편,
    열편,
}

/// 장구의 상태
#[derive(Debug, Clone, Copy)]
pub struct JangguInputState {
    pub 궁채: Option<JangguFace>,
    pub 열채: Option<JangguFace>,
}

// Need for serde default value
impl JangguFace {
    pub(crate) fn default() -> JangguFace {
        JangguFace::궁편
    }
}
