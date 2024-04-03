use std::{path::Path, time};

use bidrum_data_struct_lib::song::GameSong;
use num_rational::Rational64;
use sdl2::{image::LoadTexture, keyboard::Keycode, pixels::PixelFormatEnum};

use crate::load_menu_texture;

use super::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
    util::render_menu::{draw_menu, MenuAnimation, MenuItem},
    video_file_renderer::VideoFileRenderer,
};

pub(crate) struct SongSelectionResult {
    pub selected_song: GameSong,
    pub selected_level: u32,
    // TO-DO: add velocity modifier (e.g. x1, x1.5, x2)
}

pub(crate) fn select_song(
    common_context: &mut GameCommonContext,
    songs: &Vec<GameSong>,
) -> SongSelectionResult {
    // States
    let mut selected_song_index: usize = 0;
    let mut new_selected_song_index: usize = 0;
    let mut move_animation_started_at: Option<time::Instant> = None;
    let screen_entered_at = time::Instant::now(); // Used for rendering background video

    // Load background video
    let mut background_video = VideoFileRenderer::new(Path::new("assets/background.mkv"));
    background_video.infinite = true;

    let background_video_size = background_video.get_size();

    // Create texture for background video
    let texture_creator = common_context.canvas.texture_creator();
    let mut background_video_texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::IYUV,
            background_video_size.0,
            background_video_size.1,
        ) // the texture should be streaming IYUV format
        .expect("Failed to create texture");

    // Prepare for menu rendering
    let menu_textures = load_menu_texture!(texture_creator);
    let menu_items_vec = {
        let mut vec: Vec<MenuItem> = vec![];

        for i in songs {
            vec.push(MenuItem {
                texture: texture_creator
                    .load_texture(i.cover_image_filename.clone())
                    .expect("Cover image load failure"),
                label: i.title.clone(),
            });
        }

        vec
    };
    let menu_items = menu_items_vec.as_slice();

    'running: loop {
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                std::process::exit(0);
            }
            match event {
                // Select left song
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if matches!(move_animation_started_at, None) {
                        let mut tmp = new_selected_song_index as i32;
                        tmp = selected_song_index as i32 - 1;
                        while tmp < 0 {
                            tmp += songs.len() as i32;
                        }

                        new_selected_song_index = tmp as usize;
                        move_animation_started_at = Some(time::Instant::now());
                    }
                }
                // Select right song
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if matches!(move_animation_started_at, None) {
                        new_selected_song_index = selected_song_index + 1;
                        while new_selected_song_index >= songs.len() {
                            new_selected_song_index -= songs.len();
                        }
                        move_animation_started_at = Some(time::Instant::now());
                    }
                }
                // Return selected song
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    if matches!(move_animation_started_at, Some(_)) {
                        selected_song_index = new_selected_song_index;
                    }
                    break 'running;
                }
                _ => {}
            }
        }

        // Render background video frame
        background_video.wanted_time_in_second =
            Rational64::new(screen_entered_at.elapsed().as_millis() as i64, 1000);
        background_video.render_frame(&mut background_video_texture);

        // Clear canvas
        common_context.canvas.clear();

        // Copy background video into the canvas
        common_context
            .canvas
            .copy(&background_video_texture, None, None)
            .expect("Background video rendering failure");

        // Render album jacket arts
        let mut item_move_easing = 0.0;
        if let Some(move_started_at) = move_animation_started_at {
            let millis = move_started_at.elapsed().as_millis();
            if millis > 200 {
                move_animation_started_at = None;
                selected_song_index = new_selected_song_index;
            } else {
                item_move_easing = millis as f64 / 200.0
                    * if new_selected_song_index == selected_song_index + 1
                        || (new_selected_song_index == 0 && selected_song_index == songs.len() - 1)
                    {
                        1.0
                    } else {
                        -1.0
                    }
            }
        }

        draw_menu(
            &menu_textures,
            common_context,
            menu_items,
            selected_song_index,
            MenuAnimation {
                item_move_easing: item_move_easing,
            },
        );

        render_common(common_context);

        common_context.canvas.present();
    }

    return SongSelectionResult {
        selected_level: songs[selected_song_index].get_chart_levels().unwrap()[0],
        selected_song: songs[selected_song_index].clone(),
    };
}
