use std::path::Path;

use bidrum_data_struct_lib::song::GameSong;
use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, rect::Rect};

use super::game_common_context::GameCommonContext;

pub(crate) struct SongSelectionResult {
    pub selected_song: GameSong,
    pub selected_level: u32,
    // TO-DO: add velocity modifier (e.g. x1, x1.5, x2)
}

pub(crate) fn select_song(
    common_context: &mut GameCommonContext,
    songs: &Vec<GameSong>,
) -> SongSelectionResult {
    // TO-DO: render screen, process user input, eventaully return selected song and options
    let select_song_background_img_path = Path::new("assets/img/select_song_ui/select_song_background.png");
    let texture_creator =  common_context.canvas.texture_creator();
    let select_song_background_img_texture = texture_creator
        .load_texture(select_song_background_img_path)
        .expect("Background img file not found");

    common_context.canvas.clear();
    common_context.canvas.copy(&select_song_background_img_texture, None, None)
    .expect("Failed to render background image");

    common_context.canvas.present();

    
    'running: loop {
        for event in common_context.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::KeyDown { 
                        keycode: Some(Keycode::Escape), 
                        ..
                    }  => {
                        break 'running;
                    }
                    _ => {}
            }
         }
    }

    return SongSelectionResult {
        selected_level: songs[0].get_chart_levels().unwrap()[0],
        selected_song: songs[0].clone(),
    };
}
