use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::TextureQuery};

use super::game_common_context::GameCommonContext;

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
    let ttf_context = sdl2::ttf::init().expect("ttf context initialization failure");

    let canvas = &mut context.canvas;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context
        .load_font("assets/sans.ttf", 32)
        .expect("Unable to load font");
    font.set_style(sdl2::ttf::FontStyle::BOLD);

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

    let surface = font
        .render(text.as_str())
        .blended(Color::RGBA(255, 0, 0, 255))
        .expect("Font rendering failure");
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .expect("Font rendering failure");

    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(
        ((canvas.viewport().width() - width) / 2) as i32,
        (canvas.viewport().height() - height - 64) as i32,
        width as u32,
        height as u32,
    );
    canvas.copy(&texture, None, Some(target)).expect("Failure");
}
