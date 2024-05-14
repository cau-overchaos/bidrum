use std::{fs, thread::sleep, time::Duration};

use device_query::{DeviceQuery, DeviceState, Keycode};
use kira::{
    clock::ClockSpeed,
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};
use rand::Rng;

pub fn beep_boop() {
    println!("Beep-boop input measurement program");
    println!("");
    println!("Press d or f or j or k key when you hear beep");
    println!("initializing kira backend");

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
        .expect("Failed to init kira backend");

    println!("loading beep sound file");
    let beep_sound_filenames: Vec<String> = fs::read_dir("beep_sounds")
        .expect("ERROR: No beep_sounds directory found!")
        .filter(|i| i.as_ref().is_ok_and(|x| x.path().is_file()))
        .map(|i| i.unwrap().path().to_str().unwrap().to_string())
        .collect();

    if beep_sound_filenames.len() == 0 {
        panic!("No beep sound file found!");
    }

    let mut delays = vec![];
    for i in 0..10 {
        let beep_index = rand::thread_rng().gen_range(0..beep_sound_filenames.len());
        let start_time = rand::thread_rng().gen_range(1000..5000);
        let clock = manager
            .add_clock(ClockSpeed::TicksPerSecond(1000.0))
            .expect("Failed to initialize clock");
        let sound_settings = StaticSoundSettings::new().start_time(clock.time() + start_time);
        let sound =
            StaticSoundData::from_file(beep_sound_filenames[beep_index].clone(), sound_settings)
                .expect("Failed to load sound");

        let mut sound_handle = manager.play(sound).expect("Failed to play beep sound");
        println!("{}th beep", i + 1);
        clock.start().expect("Failed to start clock");

        while clock.time().ticks < 10000 {
            let tick = clock.time().ticks;
            let pressed = DeviceState::new()
                .get_keys()
                .iter()
                .find(|x| {
                    matches!(x, Keycode::D)
                        || matches!(x, Keycode::F)
                        || matches!(x, Keycode::J)
                        || matches!(x, Keycode::K)
                })
                .is_some();

            if pressed && tick >= start_time {
                delays.push(tick - start_time);
                sound_handle
                    .stop(Tween {
                        duration: Duration::ZERO,
                        easing: kira::tween::Easing::Linear,
                        start_time: kira::StartTime::Immediate,
                    })
                    .expect("Failed to stop sound");
                break;
            }
        }

        clock.stop().expect("Failed to stop clock");
        sleep(Duration::from_millis(800));
    }

    for i in &delays {
        println!("delay measured: {}", i);
    }

    if &delays.len() > &0 {
        println!("delay min: {}", delays.iter().min().unwrap());
        println!("delay max: {}", delays.iter().max().unwrap());
        println!(
            "delay avg: {}",
            (delays.iter().sum::<u64>() / delays.iter().len() as u64)
        );
    }
}
