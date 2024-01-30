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
        .load_font("assets/coin.ttf", 24)
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

    font.set_outline_width(0);
    let surface = font
        .render(text.as_str())
        .blended(Color::WHITE)
        .expect("Font rendering failure");

    font.set_outline_width(2);
    let mut outline_surface = font
        .render(text.as_str())
        .blended(Color::GRAY)
        .expect("Font rendering failure");

    let surface_width = surface.width();
    let surface_height = surface.height();
    let outline_surface_width = outline_surface.width();
    let outline_surface_height = outline_surface.height();

    surface
        .blit(
            None,
            &mut outline_surface,
            Rect::new(
                ((outline_surface_width - surface_width) / 2) as i32,
                ((outline_surface_height - surface_height) / 2) as i32,
                surface_width,
                surface_height,
            ),
        )
        .unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&outline_surface)
        .expect("Font rendering failure");

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
