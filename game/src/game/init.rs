use std::path::{Path, self};

use kira::manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend};
use sdl2::{mixer, sys::mixer::MIX_InitFlags_MIX_INIT_OGG};

use super::{game_common_context::GameCommonContext, title::render_title, game_player::{play_song, songs::GameSong}};

pub(crate) fn init_game() {
    let sdl_context = sdl2::init().expect("sdl context initialization Fail");
    let video_subsystem = sdl_context
        .video()
        .expect("sdl video subsystem initialization fail");

    let mut window = video_subsystem
        .window("rust-sdl2 demo: Video", 1920, 1080)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("window initialization fail");

    window
        .set_fullscreen(sdl2::video::FullscreenType::True)
        .expect("Failed to be fullscreen");

    sdl_context.mouse().show_cursor(false);

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("canvas initialization fail");

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context
        .event_pump()
        .expect("event pump initialization fail");
    let mut context = GameCommonContext { coins: 0, price: 2, canvas: canvas, sdl_context: sdl_context, event_pump: event_pump,
    audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("AudioManager initialization failure") };
    'running: loop {
        let title_result = render_title(&mut context);
        match title_result {
            super::title::TitleResult::Exit => {
                break 'running;
            }
            super::title::TitleResult::StartGame => {
                let songs = GameSong::get_songs();
                play_song(&mut context, 
                    &songs[0],
                    1
                )
            }
        }
    }
}
