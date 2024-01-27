use kira::manager::AudioManager;
use sdl2::{render::Canvas, video::Window, EventPump};

pub(crate) struct GameCommonContext {
    pub(crate) coins: u32,
    pub(crate) price: u32,
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) audio_manager: AudioManager,
    pub(crate) canvas: Canvas<Window>,
    pub(crate) event_pump: EventPump,
}
