use std::{path::Path, time::{Duration, Instant}};

use bidrum_data_struct_lib::song::GameSong;
use sdl2::{ event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, sys::False};

use super::{common::{event_loop_common, render_common}, game_common_context::GameCommonContext};

#[derive(PartialEq)]
enum MovingDirection {
    Left,
    Right,
    Stop,
}
pub(crate) struct SongSelectionResult {
    pub selected_song: GameSong,
    pub selected_level: u32,
    // TO-DO: add velocity modifier (e.g. x1, x1.5, x2)
}

pub(crate) fn select_song(
    common_context: &mut GameCommonContext,
    songs: &Vec<GameSong>,
) -> SongSelectionResult {
    
    // draw background of select song menu
    let select_song_background_img_path = Path::new("assets/img/select_song_ui/select_song_background.png");
    let texture_creator =  common_context.canvas.texture_creator();
    let select_song_background_img_texture = texture_creator
        .load_texture(select_song_background_img_path)
        .expect("Background img file not found");
    let viewport = common_context.canvas.viewport();
    
    // enable alpha blending
    common_context.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    // variables for song selection menu background line drawing
    let song_display_stand_width = viewport.width();
    let song_display_stand_height = viewport.height() / 2;
    let song_display_stand_x = 0;
    let song_display_stand_y = viewport.height() / 3;
    let song_display_stand_background_alpha :u8 = 100;
    
    // variables for song selection menu moving
    let mut last_key_press_time = Instant::now(); // most recent time when the user press left or right key
    let mut moving_start_x_pos = 0; // start moving position of song selection item when user press left or right key
    let moving_distance = 100; // maximum moving distance of song selection item 
    let moving_speed = 1; // moving speed of song selection item
    let mut song_selection_item_rect = Rect::new(0, 0, 300, 300); // rectangle object of song selection item
    let mut moving_direction : MovingDirection= MovingDirection::Stop; // moving direction of song selection item


    'running: loop {
        // waiting user input
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                break 'running;
            }
            
            match event {
                Event::KeyDown { // if user press right key, then song menu moves to right for specific distance
                    keycode: Some(Keycode::Right), 
                    repeat: false,
                    ..
                } => {
                    if moving_direction == MovingDirection::Stop {
                        moving_direction = MovingDirection::Right;
                        last_key_press_time = Instant::now();
                        moving_start_x_pos = song_selection_item_rect.x;
                    }
                },
                Event::KeyDown { // if user press right key, then song menu moves to left for specific distance
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    if moving_direction == MovingDirection::Stop {
                        moving_direction = MovingDirection::Left;
                        last_key_press_time = Instant::now();
                        moving_start_x_pos = song_selection_item_rect.x;
                    }
                },
                _ => {
                    
                }
            }
        }

        if moving_direction == MovingDirection::Left { // if user press right key, then song menu moves to right for specific distance
            let elapsed_time = last_key_press_time.elapsed().as_millis() as f32;
            let current_moved_distance = elapsed_time * moving_speed as f32;
            if current_moved_distance <= moving_distance as f32 { // until 
                rect.set_x((moving_start_x_pos as f32 - current_moved_distance) as i32);
            } else {
                moving_direction = MovingDirection::Stop;
            }
       } else if moving_direction == MovingDirection::Right { // if user press left key, then song menu moves to left for specific distance
            let elapsed_time = last_key_press_time.elapsed().as_millis() as f32;
            let current_moved_distance = elapsed_time * moving_speed as f32;
            if current_moved_distance <= moving_distance as f32 {
                rect.set_x((moving_start_x_pos as f32 + current_moved_distance) as i32);
            } else {
                moving_direction = MovingDirection::Stop;
            }
       }

        common_context.canvas.clear();

        // drawing background image
        common_context.canvas.copy(&select_song_background_img_texture, None, None)
        .expect("Failed to render background image");

        // drawing song selection standing menu
        common_context.canvas.set_draw_color(Color::RGBA(200, 200, 200, song_display_stand_background_alpha));
        common_context.canvas.fill_rect(Rect::new(song_display_stand_x, song_display_stand_y as i32, song_display_stand_width, song_display_stand_height))
        .unwrap();

        // drawing song select item
        common_context.canvas.fill_rect(song_selection_item_rect)
        .unwrap();

        // drawing common
        render_common(common_context);

        common_context.canvas.present();
    }

    return SongSelectionResult {
        selected_level: songs[0].get_chart_levels().unwrap()[0],
        selected_song: songs[0].clone(),
    };
}
