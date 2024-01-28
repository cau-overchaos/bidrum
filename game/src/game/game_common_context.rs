use std::sync::{atomic::AtomicU8, Arc};

use kira::manager::AudioManager;
use sdl2::{render::Canvas, video::Window, EventPump};

use crate::{janggu::JangguState, serial::parse_janggu_bits};

pub(crate) struct GameCommonContext {
    pub(crate) coins: u32,
    pub(crate) price: u32,
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) audio_manager: AudioManager,
    pub(crate) canvas: Canvas<Window>,
    pub(crate) event_pump: EventPump,
    pub(crate) janggu_bits_ptr: Arc<AtomicU8>,
}

impl GameCommonContext {
    pub(crate) fn read_janggu_state(&self) -> JangguState {
        return parse_janggu_bits(
            self.janggu_bits_ptr
                .load(std::sync::atomic::Ordering::Relaxed),
        );
    }
}
