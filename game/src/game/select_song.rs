use std::path::Path;

use bidrum_data_struct_lib::song::GameSong;
use sdl2::{ image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect};

use super::{common::{event_loop_common, render_common}, game_common_context::GameCommonContext};

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
    
    // draw background of select song menu
    // let canvas  = &mut common_context.canvas;
    let select_song_background_img_path = Path::new("assets/img/select_song_ui/select_song_background.png");
    let texture_creator =  common_context.canvas.texture_creator();
    let select_song_background_img_texture = texture_creator
        .load_texture(select_song_background_img_path)
        .expect("Background img file not found");
    let viewport = common_context.canvas.viewport();
    
    // enable alpha blending
    common_context.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    // TODO songs selection menu background line drawing
    let song_display_stand_width = viewport.width();
    let song_display_stand_height = viewport.height() / 2;
    let song_display_stand_x = 0;
    let song_display_stand_y = viewport.height() / 3;
    let song_display_stand_background_alpha :u8 = 100;
    

    // waiting user input
    'running: loop {
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                break 'running;
            }
        }
        
        common_context.canvas.clear();
        common_context.canvas.copy(&select_song_background_img_texture, None, None)
        .expect("Failed to render background image");

        common_context.canvas.set_draw_color(Color::RGBA(200, 200, 200, song_display_stand_background_alpha));
        common_context.canvas.fill_rect(Rect::new(song_display_stand_x, song_display_stand_y as i32, song_display_stand_width, song_display_stand_height))
        .unwrap();
        render_common(common_context);
        common_context.canvas.present();
    }

    return SongSelectionResult {
        selected_level: songs[0].get_chart_levels().unwrap()[0],
        selected_song: songs[0].clone(),
    };
}
