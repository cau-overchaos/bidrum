use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::TextureQuery};

use super::{
    game_common_context::GameCommonContext,
    util::create_outlined_font_texture::create_outlined_font_texture,
};
use crate::constants::DEFAULT_FONT_PATH as FONT_PATH;

pub(crate) fn event_loop_common(event: &Event, coins: &mut u32) -> bool {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => return true,
        Event::KeyDown {
            keycode: Some(Keycode::C),
            ..
        } => {
            *coins += 1;
        }
        _ => {}
    }

    return false;
}

pub(crate) fn render_common(context: &mut GameCommonContext) {
    let ttf_context = &context.ttf_context;

    let canvas = &mut context.canvas;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context
        .load_font(
            FONT_PATH.to_owned() + "coin.ttf",
            (context.dpi.0 / 6.0) as u16,
        )
        .expect("Unable to load font");

    // render a surface, and convert it to a texture bound to the canvas
    let text = if context.price == 1 {
        format!("CREDIT: {}", context.coins)
    } else {
        format!(
            "CREDIT: {} ({}/{})",
            context.coins / context.price,
            context.coins % context.price,
            context.price
        )
    };

    let texture = create_outlined_font_texture(
        &texture_creator,
        &mut font,
        text.as_str(),
        2,
        Color::WHITE,
        Color::GRAY,
    )
    .unwrap();

    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(
        ((canvas.viewport().width() - width) / 2) as i32,
        (canvas.viewport().height() - height - 20) as i32,
        width as u32,
        height as u32,
    );
    canvas.copy(&texture, None, Some(target)).expect("Failure");
}
