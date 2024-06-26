pub mod disappearing_note_effect;
pub mod displayed_song_note;
pub mod input_effect;
mod resources;

use ezing::expo_out;
use num_rational::Rational32;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureQuery},
    video::{Window, WindowContext},
};

use crate::{
    constants::{
        COMBO_FONT_SIZE, DEFAULT_FONT_COLOR, DEFAULT_FONT_OUTLINE_COLOR, DEFAULT_FONT_OUTLINE_SIZE,
        NOTE_ACCURACY_HEIGHT, NOTE_HEIGHT,
    },
    game::util::create_outlined_font_texture::create_font_texture,
};

use bidrum_data_struct_lib::janggu::{JangguFace, JangguStick};

use self::{
    disappearing_note_effect::DisapearingNoteEffect, displayed_song_note::DisplayedSongNote,
    input_effect::InputEffect, resources::ChartPlayerUIResources,
};

use super::timing_judge::NoteAccuracy;

pub struct BeatGuideline {
    pub position: f64,
    pub length: f64,
    pub even_beat: bool,
}

pub struct ChartPlayerUI<'a> {
    pub notes: Vec<DisplayedSongNote>,
    pub remaining_hat_ticks: Vec<i64>,
    pub accuracy: Option<NoteAccuracy>,
    pub combo: Option<u64>,
    pub accuracy_and_combo_time_progress: Option<f32>,
    pub input_effect: InputEffect,
    pub overall_effect_tick: u128,
    pub disappearing_note_effects: DisapearingNoteEffect,
    pub beat_guideline: Option<BeatGuideline>,
    resources: ChartPlayerUIResources<'a>,
}

/// draws chart player ui into canvas
impl ChartPlayerUI<'_> {
    /// creates new GamePlayUI
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> ChartPlayerUI {
        ChartPlayerUI::with_resources(ChartPlayerUIResources::new(texture_creator))
    }

    pub fn with_resources(resources: ChartPlayerUIResources) -> ChartPlayerUI {
        return ChartPlayerUI {
            notes: vec![],
            remaining_hat_ticks: vec![],
            accuracy: None,
            combo: None,
            accuracy_and_combo_time_progress: None,
            input_effect: InputEffect::new(),
            overall_effect_tick: 0,
            disappearing_note_effects: DisapearingNoteEffect::new(),
            beat_guideline: None,
            resources: resources,
        };
    }

    /// renders game play ui with notes
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        // loads texture of judgement line
        let judgement_line_texture = &self.resources.judgement_line_texture;

        // enable alpha blending
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        // load textures
        let note_textures = &mut self.resources.note_textures;
        let accuracy_textures = &mut self.resources.accuray_textures;
        let janggu_texture = &self.resources.janggu_texture;

        // get note size
        let right_stick_note_height = NOTE_HEIGHT;
        let right_stick_note_width = (note_textures.right_stick.query().width as f32
            / note_textures.right_stick.query().height as f32
            * right_stick_note_height as f32) as u32;
        let left_stick_note_height = (note_textures.left_stick.query().height as f32
            * (right_stick_note_height as f32 / note_textures.right_stick.query().height as f32))
            as u32;
        let left_stick_note_width = (note_textures.left_stick.query().width as f32
            / note_textures.left_stick.query().height as f32
            * left_stick_note_height as f32) as u32;
        let max_stick_note_height = std::cmp::max(left_stick_note_height, right_stick_note_height);

        // calculate background height
        let background_padding = 15;
        let background_border_height = 5 as u32;
        let background_height_without_border = max_stick_note_height + background_padding * 2;
        let background_height_with_border =
            background_height_without_border + background_border_height * 2;

        // calculate janggu width
        let janggu_width_min = ((janggu_texture.query().width as f32
            / janggu_texture.query().height as f32)
            * background_height_with_border as f32) as u32;
        let janggu_width_max = janggu_width_min + 20;

        // calculate janggu icon size
        let janggu_width = janggu_width_min
            + ((janggu_width_max - janggu_width_min) as f64 * {
                if self.input_effect.left_face.keydown_timing.is_none()
                    && self.input_effect.right_face.keydown_timing.is_none()
                {
                    0.0
                } else {
                    // animation duration
                    let animation_duration = 250;

                    // last keydown timing
                    let last_keydown_timing = self
                        .input_effect
                        .left_face
                        .keydown_timing
                        .unwrap_or(0)
                        .max(self.input_effect.right_face.keydown_timing.unwrap_or(0));

                    // elapsed time since last keydown timing
                    let delta = self.input_effect.base_tick - last_keydown_timing;

                    // return easing value
                    if delta < animation_duration {
                        1.0 - ezing::bounce_inout(delta as f64 / animation_duration as f64)
                    } else {
                        0.0
                    }
                }
            }) as u32;
        let janggu_height = ((janggu_texture.query().height as f32
            / janggu_texture.query().width as f32)
            * janggu_width as f32) as u32;

        // get viewport
        let viewport = canvas.viewport();

        // draw backgrounds
        let background_width = (viewport.width() - janggu_width_min) / 2;
        let background_y =
            (canvas.viewport().height() as i32 - (background_height_without_border as i32)) / 2;
        let background_x = [
            0,                                                 /* x coordinate of left background */
            background_width as i32 + janggu_width_min as i32, /* x coordinate of right background */
        ];
        for background_x in background_x {
            let background_alpha = {
                // is the face hitted?
                let hitting = if background_x == 0 {
                    self.input_effect.left_face.pressed
                } else {
                    self.input_effect.right_face.pressed
                };

                // base which changed by whether it's hitted or not
                let base = if hitting { 200 } else { 100 };

                // effect that changes by time
                let effect_by_time = (50.0 * {
                    ezing::quad_out({
                        let blinking_duration = 300;
                        let total_duration = 1000;
                        assert!(0 < blinking_duration && blinking_duration <= total_duration);

                        let remainder = (self.overall_effect_tick % total_duration) as f64;
                        if remainder < blinking_duration as f64 {
                            (blinking_duration as f64 - remainder) / blinking_duration as f64
                        } else {
                            0.0
                        }
                    })
                }) as u8;

                base + effect_by_time
            };
            canvas.set_draw_color(Color::RGBA(200, 200, 200, background_alpha));
            canvas
                .fill_rect(Rect::new(
                    background_x,
                    background_y,
                    background_width,
                    background_height_without_border,
                ))
                .unwrap();

            // draw border, too
            canvas.set_draw_color(Color::RGBA(120, 120, 120, background_alpha));
            canvas
                .fill_rect(Rect::new(
                    background_x,
                    background_y - background_border_height as i32,
                    background_width,
                    background_border_height,
                ))
                .unwrap();
            canvas
                .fill_rect(Rect::new(
                    background_x,
                    background_y + background_height_without_border as i32,
                    background_width,
                    background_border_height,
                ))
                .unwrap();
        }

        // judgement line info / max note width is required for beat guideline
        let judgement_line_height = max_stick_note_height;
        let judgement_line_padding_px = 20;
        let judgement_line_width = ((judgement_line_texture.query().width as f32
            / judgement_line_texture.query().height as f32)
            * max_stick_note_height as f32) as u32;
        let judgeline_line_ypos = background_y + background_padding as i32;
        let judgement_line_xposes = [
            background_width as i32 - judgement_line_width as i32 - judgement_line_padding_px, /* left judgement line */
            background_width as i32 + janggu_width_min as i32 + judgement_line_padding_px, /* right judgement line */
        ];
        let note_width_max = std::cmp::max(left_stick_note_width, right_stick_note_width);

        // draw beat guideline
        if let Some(beat_guideline) = &self.beat_guideline {
            let mut position = beat_guideline.position % beat_guideline.length;
            while position < 0.0 {
                position += beat_guideline.length;
            }
            let thicknesses: [u32; 2] = [4, 4];
            let mut white = beat_guideline.even_beat;
            loop {
                let distance_between_centers = (position * note_width_max as f64) as i32;
                let thickness = thicknesses[if white { 0 } else { 1 }];
                let color = if white {
                    Color::RGBA(255, 255, 255, 60)
                } else {
                    Color::RGBA(255, 255, 255, 30)
                };

                if distance_between_centers
                    > (background_width + (judgement_line_width + thickness) / 2) as i32
                {
                    break;
                }
                for line_x in [
                    judgement_line_xposes[0] - distance_between_centers
                        + (judgement_line_width / 2 + thickness / 2) as i32,
                    judgement_line_xposes[1]
                        + distance_between_centers
                        + (judgement_line_width / 2 - thickness / 2) as i32,
                ] {
                    canvas.set_draw_color(color);
                    canvas
                        .fill_rect(Rect::new(
                            line_x,
                            background_y,
                            thickness,
                            background_height_without_border,
                        ))
                        .unwrap();
                }

                position += beat_guideline.length;
                white = !white;
            }
        }

        // draw judgement line
        for judgement_line_xpos in judgement_line_xposes {
            canvas
                .copy(
                    &judgement_line_texture,
                    None,
                    Rect::new(
                        judgement_line_xpos,
                        judgeline_line_ypos,
                        judgement_line_width,
                        judgement_line_height,
                    ),
                )
                .unwrap();
        }

        // draw note
        let mut draw_note = |i: &DisplayedSongNote, disappearing_effect: Option<f32>| {
            let note_texture = match i.stick {
                JangguStick::궁채 => &mut note_textures.left_stick,
                JangguStick::열채 => &mut note_textures.right_stick,
            };
            let note_width = match i.stick {
                JangguStick::궁채 => left_stick_note_width,
                JangguStick::열채 => right_stick_note_width,
            };
            let note_height = match i.stick {
                JangguStick::궁채 => left_stick_note_height,
                JangguStick::열채 => right_stick_note_height,
            };
            let note_ypos = background_y
                + (background_height_without_border as i32 - note_height as i32) as i32 / 2
                + if let Some(disappearing_effect_progress) = disappearing_effect {
                    ((background_height_with_border - note_height + 20) as f32
                        * -ezing::circ_out(disappearing_effect_progress)) as i32
                } else {
                    0
                };

            /*
             *   note_xpos                                           judgement_line_xpos
             *      +---------                                              +----
             *      -        -          distance_between_centers            -   -
             *      -    *----------------------------------------------------* -
             *      -        -                                              -   -
             *      ----------                                              -----
             */
            let distance_between_centers = i.distance * note_width_max as f64;
            let note_xpos = (match i.face {
                JangguFace::궁편 => judgement_line_xposes[0],
                JangguFace::열편 => judgement_line_xposes[1],
            } as f64
                + (judgement_line_width / 2) as f64
                + distance_between_centers
                    * match i.face {
                        JangguFace::궁편 => -1.0,
                        JangguFace::열편 => 1.0,
                    }
                - (note_width / 2) as f64) as i32;

            // Do not render note if the note is on janggu icon
            let near_center_edge_x_pos = match i.face {
                JangguFace::궁편 => note_xpos + note_width as i32,
                JangguFace::열편 => note_xpos,
            };
            let distance_with_center = match i.face {
                JangguFace::궁편 => (viewport.width() / 2) as i32 - near_center_edge_x_pos,
                JangguFace::열편 => near_center_edge_x_pos - (viewport.width() / 2) as i32,
            };
            if distance_with_center <= (janggu_width_min / 2) as i32 {
                return;
            }

            // draw note
            if let Some(disappearing_effect_progress) = disappearing_effect {
                note_texture.set_alpha_mod(
                    (255.0 * (1.0 - ezing::circ_out(disappearing_effect_progress))) as u8,
                );
            } else {
                note_texture.set_alpha_mod(255);
            }

            canvas
                .copy(
                    note_texture,
                    None,
                    Rect::new(note_xpos, note_ypos, note_width, note_height),
                )
                .unwrap();
        };

        // draw right-stick first, and then draw left-stick.
        for i in &self.notes {
            if matches!(i.stick, JangguStick::열채) {
                draw_note(i, None);
            }
        }
        for i in &self.notes {
            if matches!(i.stick, JangguStick::궁채) {
                draw_note(i, None);
            }
        }

        // draw disappearing notes, too
        let note_disappearing_duration = DisapearingNoteEffect::effect_duration();
        for i in &self.disappearing_note_effects.notes {
            let tick_delta = i.tick.abs_diff(self.disappearing_note_effects.base_tick);
            if (matches!(i.note.stick, JangguStick::열채)
                && tick_delta < note_disappearing_duration)
            {
                draw_note(
                    &i.note,
                    Some(tick_delta as f32 / note_disappearing_duration as f32),
                )
            }
        }
        for i in &self.disappearing_note_effects.notes {
            let tick_delta = i.tick.abs_diff(self.disappearing_note_effects.base_tick);
            if (matches!(i.note.stick, JangguStick::궁채)
                && tick_delta < note_disappearing_duration)
            {
                draw_note(
                    &i.note,
                    Some(tick_delta as f32 / note_disappearing_duration as f32),
                )
            }
        }

        // draw janggu icon on center
        let janggu_hidden = if self.remaining_hat_ticks.is_empty()
            || self
                .remaining_hat_ticks
                .iter()
                .map(|i| i.abs())
                .all(|i| i > 3000)
        {
            false
        } else {
            (self
                .remaining_hat_ticks
                .iter()
                .map(|i| i.abs())
                .min()
                .unwrap()
                / 100)
                % 2
                == 0
        };

        if !janggu_hidden {
            canvas
                .copy(
                    &janggu_texture,
                    None,
                    Rect::new(
                        (viewport.width() - janggu_width) as i32 / 2,
                        (viewport.height() - janggu_height) as i32 / 2,
                        janggu_width,
                        janggu_height,
                    ),
                )
                .expect("Failed to draw janggu icon");
        }

        // draw note accuracy
        if let Some(accuracy) = self.accuracy {
            let accuracy_texture = match accuracy {
                NoteAccuracy::Overchaos => &mut accuracy_textures.overchaos,
                NoteAccuracy::Perfect => &mut accuracy_textures.perfect,
                NoteAccuracy::Great => &mut accuracy_textures.great,
                NoteAccuracy::Good => &mut accuracy_textures.good,
                NoteAccuracy::Bad => &mut accuracy_textures.bad,
                NoteAccuracy::Miss => &mut accuracy_textures.miss,
            };

            let height = NOTE_ACCURACY_HEIGHT as i32;
            let width = (Rational32::new(
                accuracy_texture.query().width as i32 * height as i32,
                accuracy_texture.query().height as i32,
            ))
            .to_integer() as u32;
            let x = (viewport.width() - width) as i32 / 2;
            let y_start =
                (viewport.height() - background_height_with_border) as i32 / 2 - (height / 2);
            let y_end = y_start - height as i32 - 10;
            let y = y_start
                + ((y_end - y_start) as f32
                    * expo_out(self.accuracy_and_combo_time_progress.unwrap()))
                    as i32;

            accuracy_texture.set_alpha_mod(
                (expo_out(self.accuracy_and_combo_time_progress.unwrap()) * 255.0) as u8,
            );

            canvas
                .copy(
                    &accuracy_texture,
                    None,
                    Rect::new(x, y, width as u32, height as u32),
                )
                .unwrap();
        }

        // draw combo
        if let Some(combo) = self.combo {
            let texture_creator = &canvas.texture_creator();
            let mut combo_texture = create_font_texture(
                texture_creator,
                &self.resources.combo_font,
                &format!("COMBO: {}", combo),
                COMBO_FONT_SIZE,
                DEFAULT_FONT_OUTLINE_SIZE,
                DEFAULT_FONT_COLOR,
                Some(DEFAULT_FONT_OUTLINE_COLOR),
            )
            .unwrap();

            let TextureQuery { width, height, .. } = combo_texture.query();
            let x = (viewport.width() - width) as i32 / 2;
            let y_start = (viewport.height() - background_height_with_border) as i32 / 2
                - (height as i32 / 2);
            let y_end = y_start - height as i32 + 300;
            let y = y_start
                + ((y_end - y_start) as f32
                    * expo_out(self.accuracy_and_combo_time_progress.unwrap()))
                    as i32;

            combo_texture.set_alpha_mod(
                (expo_out(self.accuracy_and_combo_time_progress.unwrap()) * 255.0) as u8,
            );

            canvas
                .copy(&combo_texture, None, Rect::new(x, y, width, height))
                .unwrap();
        }
    }
}
