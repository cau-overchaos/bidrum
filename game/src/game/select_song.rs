use std::{path::Path, time::Instant};

use bidrum_data_struct_lib::song::GameSong;
use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::Texture,
};

use crate::constants::DEFAULT_FONT_PATH as FONT_PATH;
use crate::constants::DEFAULT_IMG_PATH as IMG_PATH;
use crate::constants::SELECT_SONG_FONT_COLOR;

use super::util::create_outlined_font_texture::create_font_texture;
use super::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
};

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
    pub levels: Vec<u32>,
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
    let texture_creator = common_context.canvas.texture_creator();

    // font information
    let font_path = &(FONT_PATH.to_owned() + "/sans.ttf");
    let title_font_size = 30;
    let artist_font_size = 20;

    // convert GameSong vector to SongSelectionItem vector
    let song_selection_items = {
        let mut song_selection_item_vec: Vec<SongSelectionItem> = vec![];
        for song in songs {
            song_selection_item_vec.push(SongSelectionItem {
                title: song.title.clone(),
                artist: song.artist.clone(),
                cover_img_texture: texture_creator
                    .load_texture(song.cover_image_filename.clone())
                    .expect("failed to load cover image"),
                levels: song.levels.clone(),
            });
        }
        song_selection_item_vec
    };

    // background of select song screen information
    let binding = IMG_PATH.to_owned() + "/select_song_ui/select_song_background.png";
    let background_img_path = Path::new(&binding);
    let background_img_texture = texture_creator
        .load_texture(background_img_path)
        .expect("failed to find ackground image file");
    let viewport = common_context.canvas.viewport();

    // variables for song display area
    let song_display_area_width = viewport.width();
    let song_display_area_height = viewport.height() / 2;
    let song_display_area_y = (viewport.height() / 4) as i32;

    // song item image information
    let song_item_texture = texture_creator
        .load_texture(IMG_PATH.to_owned() + "/select_song_ui/song_item_scroll.png")
        .expect("failed to load song item image");
    let song_item_texture_original_width = song_item_texture.query().width;
    let song_item_texture_original_height = song_item_texture.query().height;
    let song_item_center_y =
        (song_display_area_y + (song_display_area_y + song_display_area_height as i32)) / 2;

    let song_item_width = (song_display_area_height) * 3 / 4;
    let song_item_height =
        song_item_width * (song_item_texture_original_height / song_item_texture_original_width); // height with ratio with original image file

    // selected song item variables
    let mut selecetd_song_item_idx: i32 = 0; // the center item of displayed song item is selected song item

    let selected_song_item_width = song_display_area_height;
    let selected_song_item_height = selected_song_item_width;
    let selected_song_item_center_x = viewport.width() as i32 / 2;

    // variables for song selection menu moving
    let mut last_key_press_time = Instant::now(); // most recent time when the user press left or right key

    let moving_speed = 1; // moving speed of song selection item
    let song_selection_item_interval = (song_display_area_width / 4) as i32; // interval between song selection item
    let moving_distance = song_selection_item_interval; // moving distance of song selection item
    let mut moving_direction: MovingDirection = MovingDirection::Stop; // moving direction of song selection item

    let song_item_size_changing_speed = (selected_song_item_width - song_item_width) as f32
        / (moving_distance / moving_speed) as f32; // changing speed of song item size according to the moving speed

    let mut selected_song_item_moving_center_x = selected_song_item_center_x; // x position of center of moving selected song item

    // enable alpha blending
    common_context
        .canvas
        .set_blend_mode(sdl2::render::BlendMode::Blend);
    'running: loop {
        // waiting user input
        for event in common_context.event_pump.poll_iter() {
            if event_loop_common(&event, &mut common_context.coins) {
                break 'running;
            }

            match event {
                Event::KeyDown {
                    // if user press right key, then song menu moves to right for specific distance
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    if moving_direction == MovingDirection::Stop {
                        // to prevent changing direction when moving
                        moving_direction = MovingDirection::Right;
                        last_key_press_time = Instant::now();
                    }
                }
                Event::KeyDown {
                    // if user press right key, then song menu moves to left for specific distance
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    if moving_direction == MovingDirection::Stop {
                        // to prevent changing direction when moving
                        moving_direction = MovingDirection::Left;
                        last_key_press_time = Instant::now();
                    }
                }
                Event::KeyDown {
                    // if user press enter key, then song is selected
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    if moving_direction == MovingDirection::Stop {
                        break 'running;
                    }
                }
                _ => {}
            }
        }

        let elapsed_time = last_key_press_time.elapsed().as_millis() as f32;
        if moving_direction == MovingDirection::Left {
            // if user press right key, then song items move to right for specific distance
            let current_moved_distance = elapsed_time * moving_speed as f32;
            if current_moved_distance <= moving_distance as f32 {
                // until song items move moving_distance
                selected_song_item_moving_center_x =
                    (selected_song_item_center_x as f32 - current_moved_distance) as i32;
            } else {
                // after the song items moved, the seleceted song is changed
                moving_direction = MovingDirection::Stop;
                selecetd_song_item_idx += 1;
                if selecetd_song_item_idx >= song_selection_items.len() as i32 {
                    // for circular array
                    selecetd_song_item_idx = 0;
                }
                selected_song_item_moving_center_x = selected_song_item_center_x;
            }
        } else if moving_direction == MovingDirection::Right {
            // if user press left key, then song items move to left for specific distance
            let current_moved_distance = elapsed_time * moving_speed as f32;
            if current_moved_distance <= moving_distance as f32 {
                // until song items move moving_distance
                selected_song_item_moving_center_x =
                    (selected_song_item_center_x as f32 + current_moved_distance) as i32;
            } else {
                // after the song items moved, the seleceted song is changed
                moving_direction = MovingDirection::Stop;
                selecetd_song_item_idx -= 1;
                if selecetd_song_item_idx < 0 {
                    // for circular array
                    selecetd_song_item_idx = song_selection_items.len() as i32 - 1;
                }
                selected_song_item_moving_center_x = selected_song_item_center_x;
            }
        }

        let leftmost_item_idx: i32 = selecetd_song_item_idx - 2 - 1; // index of leftmost song item
        let right_most_item_idx: i32 = selecetd_song_item_idx + 2 + 1; // index of rightmost song item

        common_context.canvas.clear();

        // drawing background image
        common_context
            .canvas
            .copy(&background_img_texture, None, None)
            .expect("failed to render background image");

        // draw all displayed song items
        for i in leftmost_item_idx..right_most_item_idx + 1 {
            // draw song item
            let item_center_x = selected_song_item_moving_center_x
                + (i - selecetd_song_item_idx) * song_selection_item_interval;
            let mut item_rect = Rect::new(-1, -1, song_item_width, song_item_height);

            if moving_direction == MovingDirection::Stop {
                if i == (leftmost_item_idx + right_most_item_idx) / 2 {
                    item_rect.set_width(selected_song_item_width);
                    item_rect.set_height(selected_song_item_height);
                }
            } else if moving_direction == MovingDirection::Left {
                if i == (leftmost_item_idx + right_most_item_idx) / 2 {
                    // if you press left, then the size of center song item will be decreased
                    item_rect.w = selected_song_item_width as i32
                        - (elapsed_time * song_item_size_changing_speed) as i32;
                    item_rect.set_height(item_rect.w as u32);
                } else if i == (leftmost_item_idx + right_most_item_idx) / 2 + 1 {
                    // if you press left, then the size of right item of center song item will be increased
                    item_rect.w = song_item_width as i32
                        + (elapsed_time * song_item_size_changing_speed) as i32;
                    item_rect.set_height(item_rect.w as u32);
                }
            } else if moving_direction == MovingDirection::Right {
                if i == (leftmost_item_idx + right_most_item_idx) / 2 {
                    // if you press right, then the size of center song item will be decreased
                    item_rect.w = selected_song_item_width as i32
                        - (elapsed_time * song_item_size_changing_speed) as i32;
                    item_rect.set_height(item_rect.w as u32);
                } else if i == (leftmost_item_idx + right_most_item_idx) / 2 - 1 {
                    // if you press right, then the size of left item of center song item will be increased
                    item_rect.w = song_item_width as i32
                        + (elapsed_time * song_item_size_changing_speed) as i32;
                    item_rect.set_height(item_rect.w as u32);
                }
            }
            set_center_x_of_rect(&mut item_rect, item_center_x);
            set_center_y_of_rect(&mut item_rect, song_item_center_y);
            common_context
                .canvas
                .copy(&song_item_texture, None, item_rect)
                .unwrap();

            // convert the position index to index for song_selection_item vectors
            let mut real_song_selection_idx = i % song_selection_items.len() as i32;
            if real_song_selection_idx < 0 {
                // for circular array
                real_song_selection_idx += song_selection_items.len() as i32;
            }

            // draw cover image
            let cover_img_center_x = item_center_x;
            let cover_img_center_y: i32 = song_item_center_y;
            let cover_img_width = item_rect.w as u32 * 3 / 8;
            let cover_img_height = cover_img_width;
            let mut cover_img_rect: Rect = Rect::new(-1, -1, cover_img_width, cover_img_height);
            set_center_x_of_rect(&mut cover_img_rect, cover_img_center_x);
            set_center_y_of_rect(&mut cover_img_rect, cover_img_center_y);
            common_context
                .canvas
                .copy(
                    &song_selection_items[real_song_selection_idx as usize].cover_img_texture,
                    None,
                    cover_img_rect,
                )
                .expect("failed to render cover image");

            // font information
            let font = common_context
                .freetype_library
                .new_face(font_path, 0)
                .expect("failed to load font"); // change font size according to the item_rect size
            let title_font_size =
                (title_font_size as f32 * (item_rect.w as f32 / song_item_width as f32)) as u16;
            let artist_font_size =
                (artist_font_size as f32 * (item_rect.w as f32 / song_item_width as f32)) as u16;

            // draw title font
            let title_str_center_x = item_center_x;
            let title_str_center_y = song_item_center_y + item_rect.h / 4;
            //
            let texture = create_font_texture(
                &texture_creator,
                &font,
                &song_selection_items[real_song_selection_idx as usize].title,
                title_font_size,
                0,
                SELECT_SONG_FONT_COLOR,
                None,
            )
            .unwrap();
            let texture_query = texture.query();
            let mut title_str_rect: Rect =
                Rect::new(-1, -1, texture_query.width, texture_query.height);
            set_center_x_of_rect(&mut title_str_rect, title_str_center_x);
            set_center_y_of_rect(&mut title_str_rect, title_str_center_y);
            common_context
                .canvas
                .copy(&texture, None, title_str_rect)
                .expect("failed to render title texture");

            // draw artist font
            let artist_str_center_x = item_center_x;
            let artist_str_center_y = title_str_center_y
                + (title_str_center_y as f32 * (item_rect.w as f32 / song_item_width as f32)
                    / 25_f32) as i32; // change interval between title and artist fort according to the item_rect size
            let texture = create_font_texture(
                &texture_creator,
                &font,
                &song_selection_items[real_song_selection_idx as usize].artist,
                artist_font_size,
                0,
                SELECT_SONG_FONT_COLOR,
                None,
            )
            .unwrap();
            let texture_query = texture.query();
            let mut artist_str_rect: Rect =
                Rect::new(-1, -1, texture_query.width, texture_query.height);
            set_center_x_of_rect(&mut artist_str_rect, artist_str_center_x);
            set_center_y_of_rect(&mut artist_str_rect, artist_str_center_y);
            common_context
                .canvas
                .copy(&texture, None, artist_str_rect)
                .expect("failed to render artist texture");
        }

        // drawing common
        render_common(common_context);

        common_context.canvas.present();
    }

    SongSelectionResult {
        selected_level: songs[selecetd_song_item_idx as usize]
            .get_chart_levels()
            .unwrap()[0],
        selected_song: songs[selecetd_song_item_idx as usize].clone(),
    }
}

pub(crate) fn set_center_x_of_rect(rect: &mut Rect, x: i32) {
    rect.set_x(x - rect.w / 2);
}

pub(crate) fn set_center_y_of_rect(rect: &mut Rect, y: i32) {
    rect.set_y(y - rect.h / 2);
}
