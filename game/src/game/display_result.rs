use std::path::Path;

use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};

use super::{
    common::{render_common},
    game_common_context::GameCommonContext, 
    game_player::game_result::GameResult,
};

use bidrum_data_struct_lib::song::GameSong;

fn render_cover_image(canvas: &mut Canvas<Window>, cover_image_path: &Path){
    let texture_creator = canvas.texture_creator();
    let cover_img_texture = texture_creator
        .load_texture(cover_image_path)
        .expect("Cover image file not found");

    let canvas_size = canvas.viewport();
    let canvas_cover_image_ratio = canvas_size.height() as f32 / 2.0;

    canvas
        .copy(
            &cover_img_texture,
            None,
            Some(Rect::new(
                (canvas_size.width() as f32/ 10.0) as i32,
                (canvas_size.height() as f32/ 4.0) as i32,
                canvas_cover_image_ratio as u32,
                canvas_cover_image_ratio as u32,
            )),
        )
        .expect("Cover image rendering failure");
}

fn render_score(result: &GameResult) {
    // TO-DO: render score
}

pub(crate) fn display_result(
    common_context: &mut GameCommonContext,
    result: GameResult,
    song_data: &GameSong
) {
    // TO-DO: display result screen
    loop {
        for event in common_context.event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    return; 
                }
                _ => {}
            }
        }

        let canvas = &mut common_context.canvas;
        canvas.clear();

        render_cover_image(canvas, Path::new(&song_data.cover_image_filename));
        render_score(&result);
        render_common(common_context);
        common_context.canvas.present();
    }
}
