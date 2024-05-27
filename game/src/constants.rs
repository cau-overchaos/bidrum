use const_format::concatcp;
use sdl2::pixels::Color;

// file path
// asset path
pub const DEFAULT_ASSET_PATH: &str = "assets";
pub const DEFAULT_AUDIO_PATH: &str = concatcp!(DEFAULT_ASSET_PATH, "/audio");
pub const DEFAULT_DIALOG_PATH: &str = concatcp!(DEFAULT_ASSET_PATH, "/dialog");
pub const DEFAULT_FONT_PATH: &str = concatcp!(DEFAULT_ASSET_PATH, "/font");
pub const DEFAULT_IMG_PATH: &str = concatcp!(DEFAULT_ASSET_PATH, "/img");
pub const DEFAULT_SOUND_PATH: &str = concatcp!(DEFAULT_ASSET_PATH, "/sound");
pub const DEFAULT_VIDEO_PATH: &str = concatcp!(DEFAULT_ASSET_PATH, "/video");

// judgement_time
pub const OVERCHAOS_TIMING: i64 = 10;
pub const PERFECT_TIMING: i64 = 44;
pub const GREAT_TIMING: i64 = 70;
pub const GOOD_TIMING: i64 = 95;
pub const BAD_TIMING: i64 = 160;

// add combo value
pub const OVERCHAOS_COMBO: u64 = 1;
pub const PERFECT_COMBO: u64 = 1;
pub const GREAT_COMBO: u64 = 1;
pub const GOOD_COMBO: u64 = 1;
pub const BAD_COMBO: u64 = 0;
pub const MISS_COMBO: u64 = 0;

// add health value
pub const DEFAULT_HEALTH: u64 = 1000;
pub const OVERCHAOS_HEALTH: i64 = 50;
pub const PERFECT_HEALTH: i64 = 50;
pub const GREAT_HEALTH: i64 = 10;
pub const GOOD_HEALTH: i64 = 0;
pub const BAD_HEALTH: i64 = -20;
pub const MISS_HEALTH: i64 = -50;

// note height
pub const NOTE_HEIGHT: u32 = 120;
// note accuracy width
pub const NOTE_ACCURACY_WIDTH: u32 = 200;

// judgement is visible time
pub const ACCURACY_DISPLAY_DURATION: u32 = 800;

// default bpm
pub const DEFAULT_BPM: u32 = 4;

// font size
pub const DEFAULT_FONT_SIZE: u16 = 20;
pub const CREDIT_FONT_SIZE: u16 = 30;
pub const GAME_RESULT_FONT_SIZE: u16 = 35;
pub const COMBO_FONT_SIZE: u16 = 40;

// font outline size
pub const DEFAULT_FONT_OUTLINE_SIZE: u16 = 4;

// font color
pub const DEFAULT_FONT_COLOR: Color = Color::WHITE;
pub const SELECT_SONG_FONT_COLOR: Color = Color::BLACK;

// font outline color
pub const DEFAULT_FONT_OUTLINE_COLOR: Color = Color::BLACK;
