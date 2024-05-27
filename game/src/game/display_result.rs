use std::path::Path;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    video::Window,
};

use crate::constants::DEFAULT_FONT_PATH as FONT_PATH;
use crate::constants::{DEFAULT_FONT_OUTLINE_SIZE, GAME_RESULT_FONT_SIZE};

use super::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
    game_player::game_result::GameResult,
    util::create_outlined_font_texture::create_font_texture,
    util::render_game_assets::{render_cover_image_at, render_level_image_at},
};

use bidrum_data_struct_lib::song::GameSong;

fn render_game_result(
    font: &cairo::freetype::face::Face,
    canvas: &mut Canvas<Window>,
    result: &GameResult,
) {
    let texture_creator = canvas.texture_creator();

    let texts =
        format!(
        "{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{:<9} {:>4}\n{}/{}",
        "Overchaos", result.overchaos_count,
        "Perfect", result.perfect_count,
        "Great", result.great_count,
        "Good", result.good_count,
        "Bad", result.bad_count,
        "Miss", result.miss_count,
        "Combo", result.max_combo,
        "Score", result.score,
        result.health, result.max_health
    );

    for (idx, text) in texts.split("\n").enumerate() {
        let texture = create_font_texture(
            &texture_creator,
            &font,
            text,
            GAME_RESULT_FONT_SIZE,
            DEFAULT_FONT_OUTLINE_SIZE,
            Color::WHITE,
            Some(Color::GRAY),
        )
        .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let canvas_size = canvas.viewport();
        let target = Rect::new(
            ((canvas_size.width() as f32 / 10.0) * 5.0) as i32,
            ((canvas_size.height() as f32 / 16.0) * (4.0 + idx as f32)) as i32,
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
    let font = common_context
        .freetype_library
        .new_face(FONT_PATH.to_owned() + "/coin.ttf", 0)
        .unwrap();
    loop {
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                return;
            }
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
        let canvas_size = canvas.viewport();

        let canvas_cover_image_ratio = canvas_size.height() as f32 / 2.0;
        render_cover_image_at(
            canvas,
            Path::new(&song_data.cover_image_filename),
            (canvas_size.width() as f32 / 10.0) as i32,
            (canvas_size.height() as f32 / 4.0) as i32,
            canvas_cover_image_ratio as u32,
            canvas_cover_image_ratio as u32,
        );

        let canvas_difficulty_image_ratio = canvas_size.height() as f32 / 8.0;
        render_level_image_at(
            canvas,
            selected_level as i32,
            (canvas_size.width() as f32 / 10.0 - canvas_difficulty_image_ratio / 2.0) as i32,
            (canvas_size.height() as f32 / 4.0 - canvas_difficulty_image_ratio / 2.0) as i32,
            canvas_difficulty_image_ratio as u32,
            canvas_difficulty_image_ratio as u32,
        );
        render_game_result(&font, canvas, &result);
        render_common(common_context);
        common_context.canvas.present();
    }
}
