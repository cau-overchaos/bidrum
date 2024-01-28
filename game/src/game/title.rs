use std::{path::Path, time::Duration};

use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};

use super::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
};

fn center_rect(rect: Rect, w: u32, h: u32) -> Rect {
    return Rect::new(
        rect.x + (rect.width() / 2 - w / 2) as i32,
        rect.y + (rect.height() / 2 - h / 2) as i32,
        w,
        h,
    );
}

/// Renders logo at center of the given canvas
fn render_logo(canvas: &mut Canvas<Window>) {
    // Load logo
    let path = Path::new("assets/img/logo.png");
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture(path)
        .expect("Logo file not found");

    // Query logo size
    let logo_width = texture.query().width;
    let logo_height = texture.query().height;

    // Render logo
    canvas
        .copy(
            &texture,
            None,
            Some(center_rect(
                canvas.viewport(),
                canvas.viewport().width() / 3,
                (canvas.viewport().width() as f32 / 3.0 * (logo_height as f32 / logo_width as f32))
                    as u32,
            )),
        )
        .expect("Logo rendering failure");
}

pub(crate) enum TitleResult {
    Exit,
    StartGame,
}

pub(crate) fn render_title(common_context: &mut GameCommonContext) -> TitleResult {
    let mut delta: i32 = 0;
    loop {
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                return TitleResult::Exit;
            }
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    if common_context.coins >= common_context.price {
                        common_context.coins -= common_context.price;
                        return TitleResult::StartGame;
                    }
                }
                _ => {}
            }
        }

        let canvas = &mut common_context.canvas;
        canvas.clear();

        let viewport = canvas.viewport();
        for i in 0..(viewport.w / 50) + 5 {
            let mut color = if i % 2 == 0 {
                Color::MAGENTA
            } else {
                Color::WHITE
            };
            for j in 0..(viewport.h / 50) + 5 {
                let x: i32 = viewport.x + 50 * i - delta;
                let y: i32 = viewport.y + 50 * j - delta;
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(x, y, 50, 50)).expect("???");
                color = match color {
                    Color::MAGENTA => Color::WHITE,
                    Color::WHITE => Color::MAGENTA,
                    _ => panic!("?"),
                }
            }
        }

        delta = (delta + 1) % 150;

        render_logo(canvas);
        render_common(common_context);
        common_context.canvas.present();
        std::thread::sleep(Duration::from_millis(3));
    }
}
