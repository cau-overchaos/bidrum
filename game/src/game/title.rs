use std::{path::Path, time::Duration};

use num_rational::Rational64;
use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};

use crate::create_streaming_iyuv_texture;

use super::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
    render_video::VideoFileRenderer,
};

fn get_logo_rect(rect: Rect, w: u32, h: u32) -> Rect {
    return Rect::new(
        rect.x + (rect.width() / 2 - w / 2) as i32,
        rect.y + (rect.height() / 2 - h) as i32,
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
            Some(get_logo_rect(
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
    let texture_creator = common_context.canvas.texture_creator();
    let mut background_video =
        VideoFileRenderer::new(Path::new("assets/video/title_bga.mkv"), true);
    let background_video_size = background_video.get_size();
    let mut background_video_texture = create_streaming_iyuv_texture!(
        texture_creator,
        background_video_size.0,
        background_video_size.1
    )
    .expect("Failed to create texture for title background video");
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

        common_context.canvas.clear();

        background_video.wanted_time_in_second = Rational64::new(
            common_context.game_initialized_at.elapsed().as_millis() as i64,
            1000,
        );
        background_video.render_frame(&mut background_video_texture);
        common_context
            .canvas
            .copy(&background_video_texture, None, None)
            .expect("Failed to render title background video");

        common_context
            .canvas
            .set_blend_mode(sdl2::render::BlendMode::Blend);
        let easing_value = {
            let duration = 1200.0;
            let remainder = (common_context.game_initialized_at.elapsed().as_millis() as f32
                % (duration * 2.0))
                / duration;
            ezing::cubic_inout(if remainder > 1.0 {
                2.0 - remainder
            } else {
                remainder
            })
        };
        common_context.canvas.set_draw_color(Color::RGBA(
            255,
            255,
            255,
            (120.0 + easing_value * 30.0) as u8,
        ));

        let viewport = common_context.canvas.viewport();
        common_context
            .canvas
            .fill_rect(Rect::new(
                viewport.width() as i32 / 4,
                0,
                viewport.width() / 2,
                viewport.height(),
            ))
            .expect("Failed to fill rect");

        render_logo(&mut common_context.canvas);
        render_common(common_context);
        common_context.canvas.present();
        std::thread::sleep(Duration::from_millis(3));
    }
}
