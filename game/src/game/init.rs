use std::{
    sync::{
        atomic::{AtomicU32, AtomicU8},
        Arc,
    },
    time::{Duration, Instant},
};

use device_query::{DeviceQuery, DeviceState, Keycode};
use kira::manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings};

use super::{game_common_context::GameCommonContext, start::start_game, title::render_title};

pub struct InitGameOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fullscreen: bool,
    pub vsync: bool,
}

pub(crate) fn init_game(janggu_bits: Arc<AtomicU8>, options: InitGameOptions) {
    // init sdl
    let sdl_context = sdl2::init().expect("sdl context initialization Fail");

    // init video
    let video_subsystem = sdl_context
        .video()
        .expect("sdl video subsystem initialization fail");

    // create window
    let mut window = video_subsystem
        .window("BIDRUM", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("window initialization fail");

    // fit window size into screen resolution
    let display_index = window.display_index().expect("Failed to display index");
    let display_mode = video_subsystem
        .desktop_display_mode(display_index)
        .expect("Failed to get desktop display mode");
    window
        .set_size(
            if let Some(width) = options.width {
                width
            } else {
                display_mode.w as u32
            },
            if let Some(height) = options.height {
                height
            } else {
                display_mode.h as u32
            },
        )
        .expect("Failed to set window size into display resolution");

    // set window fullscreen
    if options.fullscreen {
        window.set_position(
            sdl2::video::WindowPos::Positioned(0),
            sdl2::video::WindowPos::Positioned(0),
        );
        window
            .set_fullscreen(sdl2::video::FullscreenType::True)
            .expect("Failed to be fullscreen");
    }

    // get dpi
    let dpi = video_subsystem
        .display_dpi(display_index)
        .expect("Failed to get display dpi");

    // hide cursor
    sdl_context.mouse().show_cursor(false);

    // create canvas
    let mut canvas = if options.vsync {
        window.into_canvas().present_vsync()
    } else {
        window.into_canvas()
    }
    .build()
    .map_err(|e| e.to_string())
    .expect("canvas initialization fail");

    canvas.clear();
    canvas.present();

    // create event pump
    // event pump is used to receive input events (e.g. keyboard)
    let event_pump = sdl_context
        .event_pump()
        .expect("event pump initialization fail");

    // create freetype library
    let freetype_library = cairo::freetype::Library::init().expect("Failed to init FreeType");

    // create coin variable
    let coins = Arc::new(AtomicU32::new(0));
    {
        let coins_for_thread = coins.clone();

        std::thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut pressed = false;
            loop {
                let new_pressed = device_state.get_keys().contains(&Keycode::C);
                if new_pressed && !pressed {
                    // increase one on keydown
                    coins_for_thread.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                pressed = new_pressed;
                std::thread::sleep(Duration::from_millis(10));
            }
        });
    }

    // create GameCommonContext object
    let mut context = GameCommonContext {
        coins: coins,
        price: 2,
        canvas: canvas,
        dpi: dpi,
        sdl_context: sdl_context,
        event_pump: event_pump,
        audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .expect("AudioManager initialization failure"),
        janggu_bits_ptr: janggu_bits,
        game_initialized_at: Instant::now(),
        freetype_library: freetype_library,
    };

    // enter game loop
    'running: loop {
        let title_result = render_title(&mut context);
        match title_result {
            super::title::TitleResult::Exit => {
                break 'running;
            }
            super::title::TitleResult::StartGame => {
                start_game(&mut context);
            }
        }
    }
}
