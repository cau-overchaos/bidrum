use std::sync::{atomic::AtomicU8, Arc};

use bidrum_data_struct_lib::song::GameSong;
use kira::manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings};

use super::{game_common_context::GameCommonContext, game_player::play_song, title::render_title};

pub(crate) fn init_game(janggu_bits: Arc<AtomicU8>) {
    // init sdl
    let sdl_context = sdl2::init().expect("sdl context initialization Fail");

    // init video
    let video_subsystem = sdl_context
        .video()
        .expect("sdl video subsystem initialization fail");

    // create window
    let mut window = video_subsystem
        .window("rust-sdl2 demo: Video", 1920, 1080)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("window initialization fail");

    // set window fullscreen
    window
        .set_fullscreen(sdl2::video::FullscreenType::True)
        .expect("Failed to be fullscreen");

    // get dpi
    let dpi = video_subsystem
        .display_dpi(window.display_index().expect("Failed to display index"))
        .expect("Failed to get display dpi");

    // hide cursor
    sdl_context.mouse().show_cursor(false);

    // create canvas
    let mut canvas = window
        .into_canvas()
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

    // create GameCommonContext object
    let mut context = GameCommonContext {
        coins: 0,
        price: 2,
        canvas: canvas,
        dpi: dpi,
        sdl_context: sdl_context,
        event_pump: event_pump,
        audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .expect("AudioManager initialization failure"),
        janggu_bits_ptr: janggu_bits,
    };

    // enter game loop
    'running: loop {
        let title_result = render_title(&mut context);
        match title_result {
            super::title::TitleResult::Exit => {
                break 'running;
            }
            super::title::TitleResult::StartGame => {
                let songs = GameSong::get_songs();
                play_song(&mut context, &songs[0], 1)
            }
        }
    }
}
