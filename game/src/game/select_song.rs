use std::{path::Path, time::{Duration, Instant}};

use bidrum_data_struct_lib::song::GameSong;
use sdl2::{ event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::Texture, sys::False};

use super::{common::{event_loop_common, render_common}, game_common_context::GameCommonContext};

#[derive(PartialEq)]
enum MovingDirection {
    Left,
    Right,
    Stop,
}

struct SongSelectionItem<'a> {
    pub title: String,
    pub artist: String,
    pub cover_img_texture: Texture<'a>,
    pub levels: Vec<u32>
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


    let texture_creator =  common_context.canvas.texture_creator();

    // convert GameSong vector to SongSelectionItem vector
    let song_selection_items = {
        let mut song_selection_item_vec : Vec<SongSelectionItem> = vec![];
        for song in songs {
            song_selection_item_vec.push(
                SongSelectionItem {
                title: song.title.clone(),
                artist: song.artist.clone(),
                cover_img_texture: texture_creator.
                    load_texture(song.cover_image_filename.clone())
                    .expect("Cover image load fail"),
                levels: song.levels.clone(),
            });
        }

        song_selection_item_vec
    };

    // draw background of select song menu
    let select_song_background_img_path = Path::new("assets/img/select_song_ui/select_song_background.png");
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
    let song_display_stand_y = (viewport.height() / 3) as i32;
    let song_display_stand_background_alpha :u8 = 100;
    
    // variables for song selection menu moving
    let mut last_key_press_time = Instant::now(); // most recent time when the user press left or right key
    // let mut moving_start_x_pos = 0; // start moving position of song selection item when user press left or right key
    let moving_speed = 1; // moving speed of song selection item
    let item_interval = 100; // interval between song selection item
    let moving_distance = item_interval; // maximum moving distance of song selection item 
    let mut moving_direction : MovingDirection= MovingDirection::Stop; // moving direction of song selection item

    // let mut song_selection_item_rect = Rect::new(0, 0, 300, 300); // rectangle object of song selection item

    let song_selection_item_rect_width = song_display_stand_height / 2 ;
    let song_selection_item_rect_height = song_selection_item_rect_width;
    let mut selected_item_center_x = viewport.width() / 2;
    let first_song_selection_item_upper_y = (song_display_stand_y + (song_display_stand_y + song_display_stand_height as i32)) / 3;
    let mut selected_song_idx : i32 = 0; // the center item of displayed song item is selected song item
    

    // song_selection_item_rect.set_y((song_display_stand_y + (song_display_stand_y + song_display_stand_height as i32)) / 3);
    // set_center_x_of_rect(&mut song_selection_item_rect, (viewport.width()/2) as i32);


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
                        // last_key_press_time = Instant::now();
                        // moving_start_x_pos = first_song_selection_item_x;
                    }
                },
                Event::KeyDown { // if user press right key, then song menu moves to left for specific distance
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    if moving_direction == MovingDirection::Stop {
                        moving_direction = MovingDirection::Left;
                        // last_key_press_time = Instant::now();
                        // moving_start_x_pos = first_song_selection_item_x;
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
    //             first_song_selection_item_x = (moving_start_x_pos as f32 - current_moved_distance) as u32;
            } else { // after the song selection item moved, the seleceted song is changed
                moving_direction = MovingDirection::Stop;
                selected_song_idx +=1;
                if selected_song_idx >= song_selection_items.len() as i32 {
                    selected_song_idx = 0;
                }
                println!("{}", selected_song_idx);
            }
       } else if moving_direction == MovingDirection::Right { // if user press left key, then song menu moves to left for specific distance
            let elapsed_time = last_key_press_time.elapsed().as_millis() as f32;
            let current_moved_distance = elapsed_time * moving_speed as f32;
            if current_moved_distance <= moving_distance as f32 {
    //             first_song_selection_item_x = (moving_start_x_pos as f32 + current_moved_distance) as u32;
            } else { // after the song selection item moved, the seleceted song is changed
                moving_direction = MovingDirection::Stop;
                selected_song_idx -=1;
                if selected_song_idx < 0 {
                    selected_song_idx = song_selection_items.len() as i32 - 1;
                }
                println!("{}", selected_song_idx);
            }
       }

        common_context.canvas.clear();

    //     // drawing background image
        common_context.canvas.copy(&select_song_background_img_texture, None, None)
        .expect("Failed to render background image");

    //     // drawing song selection standing menu
    //     common_context.canvas.set_draw_color(Color::RGBA(200, 200, 200, song_display_stand_background_alpha));
    //     common_context.canvas.fill_rect(Rect::new(song_display_stand_x, song_display_stand_y, song_display_stand_width, song_display_stand_height))
    //     .unwrap();

    //     // drawing song select item
    //     common_context.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        

    //     // TODO DRAWING SEVERAL ITEMS, NOT WORKING
    //    let mut item_x = first_song_selection_item_x as i32;
    //     for item in &song_selection_items {
    //         common_context.canvas.fill_rect(Rect::new(item_x, song_display_stand_y, song_display_stand_width, song_display_stand_height)).unwrap();
    //         // item_x += item_interval;
    //     }

        // drawing common
        render_common(common_context);

        common_context.canvas.present();
    }

    return SongSelectionResult {
        selected_level: songs[0].get_chart_levels().unwrap()[0],
        selected_song: songs[0].clone(),
    };
}

pub(crate) fn set_center_x_of_rect(rect: &mut Rect, x : i32) {
    rect.set_x(x - rect.w/2);
}