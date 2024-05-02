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

    // selected song item variables
    let displayed_selected_song_cnt = 5; // it should be odd number for existence of center song item
    let song_selection_item_rect_width = song_display_stand_height / 2 ;
    let song_selection_item_rect_height = song_selection_item_rect_width;

    let selected_song_selection_item_rect_width = song_display_stand_height * 2 / 3;
    let selected_song_selection_item_rect_height = selected_song_selection_item_rect_width;

    // let selected_item_x = viewport.width() as i32 / 2 - song_selection_item_rect_width as i32 / 2; // '- song_selection_item_rect_width / 2' is for positioing at center
    let selected_item_center_x = viewport.width() as i32 / 2;
    let song_selection_item_center_y = (song_display_stand_y + (song_display_stand_y + song_display_stand_height as i32)) / 2;
    let mut selecetd_song_item_idx : i32 = 0; // the center item of displayed song item is selected song item

    // cover image variables
    let cover_img_width = song_selection_item_rect_width * 2 / 3;
    let cover_img_height = cover_img_width;

    // variables for song selection menu moving
    let mut last_key_press_time = Instant::now(); // most recent time when the user press left or right key
    // let mut moving_start_x_pos = 0; // start moving position of song selection item when user press left or right key
    let moving_speed = 1; // moving speed of song selection item
    let song_selection_item_interval = (song_display_stand_width / displayed_selected_song_cnt) as i32; // interval between song selection item, adjusting the interval to display displayed_selected_song_cnt number of items on display
    let moving_distance = song_selection_item_interval; // maximum moving distance of song selection item 
    let mut moving_direction : MovingDirection= MovingDirection::Stop; // moving direction of song selection item
    
    let song_selection_item_size_changing_speed = (selected_song_selection_item_rect_width - song_selection_item_rect_width) as f32 / (moving_distance / moving_speed) as f32;


    let mut selected_item_moving_center_x = selected_item_center_x;
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
                    }
                },
                _ => {
                    
                }
            }
        }

        let elapsed_time = last_key_press_time.elapsed().as_millis() as f32;
        let mut leftmost_item_idx : i32 = selecetd_song_item_idx -(displayed_selected_song_cnt as i32 / 2) - 1;
        let mut right_most_item_idx : i32 = selecetd_song_item_idx + (displayed_selected_song_cnt as i32 / 2) + 1;
        if moving_direction == MovingDirection::Left { // if user press right key, then song menu moves to right for specific distance
            let current_moved_distance = elapsed_time * moving_speed as f32;
            if current_moved_distance <= moving_distance as f32 { // until 
                selected_item_moving_center_x = (selected_item_center_x as f32 - current_moved_distance) as i32;
            } else { // after the song selection item moved, the seleceted song is changed
                moving_direction = MovingDirection::Stop;
                selecetd_song_item_idx +=1;
                if selecetd_song_item_idx >= song_selection_items.len() as i32 { // for circular array
                    selecetd_song_item_idx = 0;
                }
                selected_item_moving_center_x = selected_item_center_x; // TODO not chainging
            }
       } else if moving_direction == MovingDirection::Right { // if user press left key, then song menu moves to left for specific distance
            let current_moved_distance = elapsed_time * moving_speed as f32;
            if current_moved_distance <= moving_distance as f32 {
                selected_item_moving_center_x = (selected_item_center_x as f32 + current_moved_distance) as i32;
            } else { // after the song selection item moved, the seleceted song is changed
                moving_direction = MovingDirection::Stop;
                selecetd_song_item_idx -=1;
                if selecetd_song_item_idx < 0 { // for circular array
                    selecetd_song_item_idx = song_selection_items.len() as i32 - 1;
                }
                selected_item_moving_center_x = selected_item_center_x; // TODO not chainging
            }
        }

        common_context.canvas.clear();

        // drawing background image
        common_context.canvas.copy(&select_song_background_img_texture, None, None)
        .expect("Failed to render background image");

        // drawing song selection standing menu
        common_context.canvas.set_draw_color(Color::RGBA(200, 200, 200, song_display_stand_background_alpha));
        common_context.canvas.fill_rect(Rect::new(song_display_stand_x, song_display_stand_y, song_display_stand_width, song_display_stand_height))
        .unwrap();

        common_context.canvas.set_draw_color(Color::RGBA(200, 200, 200, 240));
        for i in leftmost_item_idx .. right_most_item_idx + 1 {
            let item_center_x = selected_item_moving_center_x + (i - selecetd_song_item_idx)* song_selection_item_interval;
            let mut item_rect = Rect::new(-1 , -1, song_selection_item_rect_width, song_selection_item_rect_height);
            
            if moving_direction == MovingDirection::Stop {
                if i == (leftmost_item_idx + right_most_item_idx) / 2 {
                    item_rect.set_width(selected_song_selection_item_rect_width);
                    item_rect.set_height(selected_song_selection_item_rect_height);
                }
            } else if moving_direction == MovingDirection::Left {
                if i == (leftmost_item_idx + right_most_item_idx) / 2 {
                    item_rect.w = selected_song_selection_item_rect_width as i32 - (elapsed_time * song_selection_item_size_changing_speed) as i32;
                    item_rect.set_height(item_rect.w as u32);
                } else if i == (leftmost_item_idx + right_most_item_idx) / 2 + 1 {
                    item_rect.w = song_selection_item_rect_width as i32 + (elapsed_time * song_selection_item_size_changing_speed) as i32;
                    item_rect.set_height(item_rect.w as u32);
                }
            } else if moving_direction == MovingDirection::Right {
                if i == (leftmost_item_idx + right_most_item_idx) / 2 {
                    item_rect.w = selected_song_selection_item_rect_width as i32 - (elapsed_time * song_selection_item_size_changing_speed) as i32;
                    item_rect.set_height(item_rect.w as u32);
                } else if i == (leftmost_item_idx + right_most_item_idx) / 2 - 1 {
                    item_rect.w = song_selection_item_rect_width as i32 + (elapsed_time * song_selection_item_size_changing_speed) as i32;
                    item_rect.set_height(item_rect.w as u32);
                }
            }
            
            set_center_x_of_rect(&mut item_rect, item_center_x);
            set_center_y_of_rect(&mut item_rect, song_selection_item_center_y);
            
            common_context.canvas.fill_rect(item_rect).unwrap();
            
            // convert the position index to index for song_selection_item vectors
            let mut real_song_selection_idx = i % song_selection_items.len() as i32;
            if real_song_selection_idx < 0 {
                real_song_selection_idx += song_selection_items.len() as i32;
            }


            // let cover_img_center_x = (item_center_x + (item_center_x + song_selection_item_rect_width as i32)) / 2;
            // let cover_img_center_y: i32 = (song_selection_item_upper_y + (song_selection_item_upper_y + song_selection_item_rect_height as i32)) / 2;
            // let mut cover_img_rect = Rect::new(-1, -1, cover_img_width, cover_img_height);
            // set_center_x_of_rect(&mut cover_img_rect, cover_img_center_x);
            // set_center_y_of_rect(&mut cover_img_rect, cover_img_center_y);
            // common_context.canvas.copy(&song_selection_items[real_song_selection_idx as usize].cover_img_texture, None, cover_img_rect).expect("Failed to render cover image");
        }


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

pub(crate) fn set_center_y_of_rect(rect: &mut Rect, y : i32) {
    rect.set_y(y - rect.h/2);
}