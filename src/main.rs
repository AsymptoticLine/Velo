use std::process::ExitCode;
use std::{env, fs, io};

use velo::models::{Rune, Vessel};
use velo::sail::{Termination, sail};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [/path/to/code.velo]", args[0]);
        return ExitCode::FAILURE;
    }

    let path = &args[1];
    match load_velo_code(path) {
        Err(msg) => {
            eprintln!("Failed to load velo file. {:}", msg);
            ExitCode::FAILURE
        }
        Ok(raw_code) => {
            let code_lines = harmonize_runes(raw_code);

            let cosmos = materialize_runes(code_lines);

            let start_rune = cosmos[0][0];

            let vessel = Vessel::new(0, 0, start_rune);

            match sail(cosmos, vessel) {
                Termination::Stopped => ExitCode::SUCCESS,
                Termination::NoSignal => {
                    eprintln!("The vessel traveled out of the cosmos.");
                    ExitCode::FAILURE
                }
                Termination::NoInitialVelocityOrDirection => {
                    eprintln!("Here was no Thrust rune at the top left corner of the cosmos.");
                    ExitCode::FAILURE
                }
            }
        }
    }
}

fn load_velo_code(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;

    Ok(content)
}

fn harmonize_runes(raw_code: String) -> Vec<String> {
    // Harmonizes the raw Velo code into a standardized m x n cosmos.
    let lines: Vec<String> = raw_code.lines().map(|line| line.to_string()).collect();

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let height = lines.len();

    let mut result = Vec::with_capacity(height);

    lines.iter().for_each(|line| {
        let mut padded = line.clone();
        if padded.len() < width {
            // Pad the line with ' ' (Void Runes) to meet the required width.
            padded.push_str(&" ".repeat(width - padded.len()));
        }
        result.push(padded);
    });

    result
}

fn materialize_runes(lines: Vec<String>) -> Vec<Vec<Rune>> {
    lines
        .iter()
        .map(|line| line.chars().map(|c| char_to_rune(c)).collect())
        .collect()
}

fn char_to_rune(c: char) -> Rune {
    match c {
        '^' => Rune::ThrustUp,
        'v' => Rune::ThrustDown,
        '<' => Rune::ThrustLeft,
        '>' => Rune::ThrustRight,
        '*' => Rune::Star,
        'P' => Rune::Parking,
        '+' => Rune::EntropyIncrease,
        '-' => Rune::EntropyDecrease,
        '[' => Rune::SteerLeft,
        ']' => Rune::SteerRight,
        ',' => Rune::Input,
        '.' => Rune::Output,
        _ => Rune::Void,
    }
}
