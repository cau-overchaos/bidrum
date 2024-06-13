use std::time::Instant;

use bidrum_hat::BidrumHat;
use kira::manager::AudioManager;
use sdl2::{render::Canvas, video::Window, EventPump};

use bidrum_data_struct_lib::janggu::JangguInputState;

use crate::controller_wrapper::ControllerWrapper;

pub(crate) struct GameCommonContext {
    pub(crate) coin_and_janggu: ControllerWrapper,
    pub(crate) price: u32,
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) audio_manager: AudioManager,
    pub(crate) canvas: Canvas<Window>,
    pub(crate) event_pump: EventPump,
    /// ddpi, hdpi, vdpi
    pub(crate) dpi: (f32, f32, f32),
    pub(crate) game_initialized_at: Instant,
    pub(crate) hat: BidrumHat,
    pub(crate) freetype_library: cairo::freetype::Library,
}

impl GameCommonContext {
    pub(crate) fn read_janggu_state(&self) -> JangguInputState {
        self.coin_and_janggu.read_janggu_state()
    }
}
