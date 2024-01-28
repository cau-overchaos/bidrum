use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum JangguStick {
    궁채,
    북채,
}

// Korean looks more intutitive than english...
/// 장구의 치는 곳
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum JangguFace {
    채편,
    북편,
}

/// 장구의 상태
#[derive(Debug, Clone, Copy)]
pub(crate) struct JangguState {
    pub 궁채: Option<JangguFace>,
    pub 북채: Option<JangguFace>,
}
