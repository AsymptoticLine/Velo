use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use velo::interpreter::{self, Config, Termination};
use velo::io::{Io, StdIo};
use velo::{Cosmos, Vessel};

/// Run a Velo program.
#[derive(Debug, Parser)]
#[command(
    version,
    about = "Run a Velo program written in the cosmic-velocity esoteric language"
)]
struct Args {
    /// Path to a `.velo` source file.
    file_path: PathBuf,

    /// Print the vessel state whenever a `D` rune is reached.
    #[arg(short, long)]
    debug: bool,

    /// Print the vessel state after every step.
    #[arg(short, long)]
    trace: bool,

    /// With `--trace`, skip the state printed for void runes.
    #[arg(long, requires = "trace")]
    ignore_void: bool,
}

pub fn run() -> ExitCode {
    let args = Args::parse();

    let source = match fs::read_to_string(&args.file_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!(
                "error: failed to read `{}`: {error}",
                args.file_path.display()
            );
            return ExitCode::FAILURE;
        }
    };

    let cosmos = Cosmos::from_source(&source);
    let mut vessel = Vessel::new(0, 0, cosmos.get(0, 0));
    let config = Config::new(args.debug || args.trace, args.trace, args.ignore_void);

    let mut io = StdIo::new();
    let termination = interpreter::run(&cosmos, &mut vessel, config, &mut io);

    match termination {
        Termination::Halted => ExitCode::SUCCESS,
        other => {
            io.write_diagnostic(&format!("error: {other}"));
            ExitCode::FAILURE
        }
    }
}
