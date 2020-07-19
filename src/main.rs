use itertools::Itertools;
use regex::Regex;
use std::process::Command;

fn main() {
    use std::env::args;

    let (sound_card_num, control_name) = args().skip(1).next_tuple().expect("WIP");

    let output = Command::new("amixer")
        .args(&["-c", &sound_card_num, "sget", &control_name])
        .output()
        .expect("Failed to execute amixer");

    let raw_output = String::from_utf8_lossy(&output.stdout);

    let channels = raw_output
        .lines()
        .find(|line| line.contains("Playback channels:"))
        .expect("Could not find line specifing playback channels in the output of `amixer`")
        .split(':')
        .skip(1)
        .join("");

    let channels: Vec<_> = channels.split('-').map(|s| s.trim()).collect();

    let first_volume = raw_output
        .lines()
        .filter(|l| {
            let channel = l.split(':').next().unwrap().trim();
            channels.contains(&channel)
        })
        .map(|l| {
            Regex::new("\\[(\\d+)%\\]")
                .unwrap()
                .captures(l)
                .unwrap()
                .get(1)
                .map_or_else(
                    || panic!("Could not find percentage volume in channel line"),
                    |m| m.as_str(),
                )
        })
        .next() // Take the first albitrarily
        .unwrap();

    println!("{}", first_volume);
}
