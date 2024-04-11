use std::path::Path;

use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::{Canvas, TextureQuery},
    video::Window,
};

use super::{
    common::render_common,
    game_common_context::GameCommonContext, 
    game_player::game_result::GameResult,
    util::create_outlined_font_texture::create_outlined_font_texture,
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
                (canvas_size.width() as f32 / 10.0) as i32,
                (canvas_size.height() as f32 / 4.0) as i32,
                canvas_cover_image_ratio as u32,
                canvas_cover_image_ratio as u32,
            )),
        )
        .expect("Cover image rendering failure");
}

fn render_difficulty_image(canvas: &mut Canvas<Window>, difficulty_image_path: &Path){
    let texture_creator = canvas.texture_creator();
    let difficulty_img_texture = texture_creator
        .load_texture(difficulty_image_path)
        .expect("Difficulty image file not found");

    let canvas_size = canvas.viewport();
    let canvas_difficulty_image_ratio = canvas_size.height() as f32 / 8.0;

    canvas
        .copy(
            &difficulty_img_texture,
            None,
            Some(Rect::new(
                (canvas_size.width() as f32 / 10.0 - canvas_difficulty_image_ratio / 2.0) as i32,
                (canvas_size.height() as f32 / 4.0 - canvas_difficulty_image_ratio / 2.0) as i32,
                canvas_difficulty_image_ratio as u32,
                canvas_difficulty_image_ratio as u32,
            )),
        )
        .expect("Difficulty image rendering failure");
}

fn render_game_result(canvas: &mut Canvas<Window>, result: &GameResult) {
    let ttf_context = sdl2::ttf::init().expect("ttf context initialization failure");

    let texture_creator = canvas.texture_creator();

    let mut font = ttf_context
        .load_font("assets/coin.ttf", 40)
        .expect("Font loading failure");

    let texts = format!(
        "{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}",
        "Overchaos", result.overchaos_count,
        "Perfect", result.perfect_count,
        "Great", result.great_count,
        "Good", result.good_count,
        "Bad", result.bad_count,
        "Miss", result.miss_count,
        "Combo", result.combo,
    );

    for (idx, text) in texts.split("\n").enumerate() {
        let texture = create_outlined_font_texture(
            &texture_creator,
            &mut font,
            text,
            2,
            Color::WHITE,
            Color::GRAY,
        )
        .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let canvas_size = canvas.viewport();
        let target = Rect::new(
            ((canvas_size.width() as f32/ 10.0) * 5.0) as i32,
            ((canvas_size.height() as f32/ 16.0)* (4.0 + idx as f32)) as i32,
            width as u32,
            height as u32,
        );
        canvas.copy(&texture, None, Some(target)).expect("Failure");
    }
}

pub(crate) fn display_result(
    common_context: &mut GameCommonContext,
    result: GameResult,
    song_data: &GameSong,
    selected_level: u32,
) {
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
        render_difficulty_image(canvas, Path::new(&format!("assets/img/difficulty/{}.png", selected_level)));
        render_game_result(canvas, &result);
        render_common(common_context);
        common_context.canvas.present();
    }
}
