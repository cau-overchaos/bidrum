use std::{
    path::Path,
    time::{self, Duration},
};

use num_rational::Rational64;
use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect,
    render::Canvas, video::Window,
};

use super::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
    video_file_renderer::VideoFileRenderer,
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
    let title_start_time = time::Instant::now();
    let mut background_video = VideoFileRenderer::new(Path::new("assets/background.mkv"));
    background_video.infinite = true;

    let background_video_size = background_video.get_size();

    let texture_creator = common_context.canvas.texture_creator();
    let mut background_video_frame = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::IYUV,
            background_video_size.0,
            background_video_size.1,
        ) // the texture should be streaming IYUV format
        .expect("Failed to create texture");

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

        let _viewport = canvas.viewport();
        background_video.wanted_time_in_second =
            Rational64::new(title_start_time.elapsed().as_millis() as i64, 1000);
        background_video.render_frame(&mut background_video_frame);
        canvas.copy(&background_video_frame, None, None);

        render_logo(canvas);
        render_common(common_context);
        common_context.canvas.present();
        std::thread::sleep(Duration::from_millis(3));
    }
}
