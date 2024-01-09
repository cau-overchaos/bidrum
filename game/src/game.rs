use std::{
    sync::{mpsc, Arc, RwLock},
    time::Duration,
};

use sdl2::{event::Event, keyboard::Keycode};

use crate::serial::ControllerState;

pub(crate) fn init_game(controller_lock: Arc<RwLock<ControllerState>>) {
    let sdl_context = sdl2::init().expect("sdl context initialization Fail");
    let video_subsystem = sdl_context
        .video()
        .expect("sdl video subsystem initialization fail");

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .expect("window initialization fail");

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

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if let Ok(guard) = controller_lock.try_read() {
            println!("{:?} {:?}", guard.궁채, guard.북채);
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
}
