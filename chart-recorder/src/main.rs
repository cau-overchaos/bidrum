mod janggu_state_with_tick;

use std::{collections::HashMap, fs::File, io::Write};

use bidrum_data_struct_lib::{
    janggu::{JangguFace, JangguStick},
    song::{GameChart, GameNote},
};
use clap::Parser;
use kira::{
    clock::ClockSpeed,
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

use crate::janggu_state_with_tick::JangguStateWithTick;

/// Chart recorder for bidrum, which plays the music and generates the chart as you hit the janggu
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path of music file
    #[arg(short, long)]
    music: String,

    /// Path of output file
    /// Default is automatically created temp file
    #[arg(short, long)]
    output: String,

    /// Delay before starting the music (in milliseconds)
    #[arg(long, default_value_t = 0)]
    delay: u16,

    /// How many splits per beat?
    #[arg(short, long, default_value_t = 4)]
    splits: u16,

    /// Music bpm
    #[arg(short, long)]
    bpm: u16,

    /// Chart artist name
    #[arg(short, long)]
    artist: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Introduction
    println!("Bidrum chart recorder");
    println!("");
    println!("This program CANNOT edit the existing chart.");
    println!("but, this program can play music and generate");
    println!("the chart as you hit the janggu (or keyboard, whatever)");
    println!("");

    // Init kira backend
    println!("initializing kira backend");
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
        .expect("kira AudioManager initialization failure");
    let clock_handle = manager
        .add_clock(ClockSpeed::TicksPerSecond(1000.0))
        .expect("kira clock add failure");

    // Load music which starts after 5 sec
    let start_tick = clock_handle.time() + 5000;
    let settings = StaticSoundSettings::new().start_time(start_tick);

    // Init variables
    let mut left_stick = HashMap::new();
    let mut right_stick = HashMap::new();
    let mut janggu_state = JangguStateWithTick::new();

    // Load music
    let music = StaticSoundData::from_file(args.music, settings).expect("Failed to load music");
    let music_duration = music.duration();

    // Start music
    println!("music will start after 5 sec");
    clock_handle.start().expect("Failed to start kira clock");
    let mut beat_and_split_before: Option<u64> = None;
    manager.play(music).expect("Failed to play music");
    loop {
        let tick = {
            let elapsed = clock_handle.time().ticks;

            if elapsed as u128 > start_tick.ticks as u128 + music_duration.as_millis() {
                break;
            }

            let record_start_tick: u64 = start_tick.ticks + args.delay as u64;

            if elapsed < record_start_tick {
                println!(
                    "music will start after {:.2} sec",
                    (record_start_tick - elapsed) as f32 / 1000.0,
                );
                continue;
            }

            elapsed - record_start_tick
        };

        janggu_state.update(tick);

        let beat_and_split =
            ((tick as f64 / (1000.0 * 60.0)) * (args.bpm * args.splits) as f64) as u64;
        if janggu_state.궁채.is_keydown_now && janggu_state.궁채.face.is_some() {
            left_stick.insert(beat_and_split, janggu_state.궁채.face.unwrap());
        }
        if janggu_state.열채.is_keydown_now && janggu_state.열채.face.is_some() {
            right_stick.insert(beat_and_split, janggu_state.열채.face.unwrap());
        }

        if !beat_and_split_before.is_some_and(|x| x == beat_and_split) {
            if let Some(beat_and_split_before_unwrapped) = beat_and_split_before {
                let beat_idx = beat_and_split_before_unwrapped / args.splits as u64;
                let split = beat_and_split_before_unwrapped % args.splits as u64;
                println!(
                    "beat: {} ({} / {}) : left_stick = {:?}, right_stick = {:?}",
                    beat_idx,
                    split,
                    args.splits,
                    left_stick.get(&beat_and_split_before_unwrapped),
                    right_stick.get(&beat_and_split_before_unwrapped)
                )
            }
            beat_and_split_before = Some(beat_and_split);
        }
    }

    println!("Converting to chart json format...");
    let mut left_face = vec![];
    let mut right_face = vec![];
    for i in left_stick.iter() {
        let beat_idx = i.0 / args.splits as u64;
        let split = i.0 % args.splits as u64;

        if matches!(i.1, JangguFace::궁편) {
            left_face.push(GameNote::create_raw_note(
                JangguStick::궁채,
                beat_idx.into(),
                split as i64,
                args.splits.into(),
            ));
        } else if matches!(i.1, JangguFace::열편) {
            right_face.push(GameNote::create_raw_note(
                JangguStick::궁채,
                beat_idx.into(),
                split as i64,
                args.splits.into(),
            ));
        }
    }

    for i in right_stick.iter() {
        let beat_idx = i.0 / args.splits as u64;
        let split = i.0 % args.splits as u64;

        if matches!(i.1, JangguFace::궁편) {
            left_face.push(GameNote::create_raw_note(
                JangguStick::열채,
                beat_idx.into(),
                split as i64,
                args.splits.into(),
            ));
        } else if matches!(i.1, JangguFace::열편) {
            right_face.push(GameNote::create_raw_note(
                JangguStick::열채,
                beat_idx.into(),
                split as i64,
                args.splits.into(),
            ));
        }
    }

    let json = GameChart::to_json_string(
        args.artist.unwrap_or("Team Overchaos".to_string()),
        args.delay.into(),
        args.bpm.into(),
        left_face,
        right_face,
    )
    .unwrap();

    let mut f = File::create(args.output).expect("Failed to create or truncate output file");
    write!(f, "{}", json).expect("Failed to write");

    println!("Done!");
}
