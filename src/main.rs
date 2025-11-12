use std::{env, fs, io::Result};

use velo::models::{Dir, Rune, Vessel};
use velo::sail::sail;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [/path/to/code.velo]", args[0]);
        return;
    }

    let path = &args[1];
    match load_velo_code(path) {
        Err(msg) => {
            eprintln!("Failed to read file: {:}", msg);
            return;
        }
        Ok(raw_code) => {
            let code_lines = harmonize_runes(raw_code);

            let cosmos = materialize_runes(code_lines);

            let width = cosmos[0].len();
            let height = cosmos.len();
            let center_x = width / 2;
            let center_y = height / 2;
            let center_rune = cosmos[center_y][center_x];

            let initial_dir = match center_rune {
                Rune::ThrustUp => Dir::Up,
                Rune::ThrustDown => Dir::Down,
                Rune::ThrustLeft => Dir::Left,
                Rune::ThrustRight => Dir::Right,
                _ => {
                    eprintln!(
                        "The center rune is not Thrust(^v<>). Center ({:}, {:}) : {:?}",
                        center_x, center_y, center_rune
                    );
                    return;
                }
            };

            let vessel = Vessel::new(center_x, center_y, initial_dir, 1);

            sail(cosmos, vessel);
        }
    }
}

fn load_velo_code(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)?;

    Ok(content)
}

fn harmonize_runes(raw_code: String) -> Vec<String> {
    let lines: Vec<String> = raw_code.lines().map(|line| line.to_string()).collect();

    let code_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let width = if code_width % 2 == 1 {
        code_width
    } else {
        code_width + 1
    };

    let code_height = lines.len();
    let height = if code_height % 2 == 1 {
        code_height
    } else {
        code_height + 1
    };

    let mut result = Vec::with_capacity(height);

    for i in 0..height {
        if let Some(line) = lines.get(i) {
            let mut padded = line.clone();
            if padded.len() < width {
                padded.push_str(&" ".repeat(width - padded.len()));
            }
            result.push(padded);
        } else {
            result.push(" ".repeat(width));
        }
    }

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
        '+' => Rune::Boost,
        '-' => Rune::Brake,
        '*' => Rune::Star,
        'P' => Rune::Parking,
        _ => Rune::Void,
    }
}
