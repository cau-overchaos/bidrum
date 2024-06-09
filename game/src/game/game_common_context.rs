use std::{
    sync::{atomic::AtomicU8, Arc},
    time::Instant,
};

use bidrum_hat::BidrumHat;
use kira::manager::AudioManager;
use sdl2::{render::Canvas, video::Window, EventPump};

use crate::serial::parse_janggu_bits;
use bidrum_data_struct_lib::janggu::JangguInputState;

pub(crate) struct GameCommonContext {
    pub(crate) coins: u32,
    pub(crate) price: u32,
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) audio_manager: AudioManager,
    pub(crate) canvas: Canvas<Window>,
    pub(crate) event_pump: EventPump,
    pub(crate) janggu_bits_ptr: Arc<AtomicU8>,
    /// ddpi, hdpi, vdpi
    pub(crate) dpi: (f32, f32, f32),
    pub(crate) game_initialized_at: Instant,
    pub(crate) hat: BidrumHat,
    pub(crate) freetype_library: cairo::freetype::Library,
}

impl GameCommonContext {
    pub(crate) fn read_janggu_state(&self) -> JangguInputState {
        return parse_janggu_bits(
            self.janggu_bits_ptr
                .load(std::sync::atomic::Ordering::Relaxed),
        );
    }
}
