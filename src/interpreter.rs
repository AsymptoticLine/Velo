//! The execution loop that sails a vessel through a cosmos.

use std::fmt;

use crate::cosmos::Cosmos;
use crate::io::Io;
use crate::rune::Rune;
use crate::vessel::Vessel;

/// Options controlling diagnostics while a program runs.
#[derive(Debug, Clone, Copy, Default)]
pub struct Config {
    /// Print the vessel state whenever a `D` rune is reached.
    pub debug: bool,
    /// Print the vessel state after every step.
    pub trace: bool,
    /// With `trace`, skip the state printed for void runes.
    pub ignore_void: bool,
}

impl Config {
    pub const fn new(debug: bool, trace: bool, ignore_void: bool) -> Self {
        Self {
            debug,
            trace,
            ignore_void,
        }
    }
}

/// Why a program stopped running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Termination {
    /// Velocity reached zero.
    Halted,
    /// The vessel tried to leave the cosmos. Coordinates are the last valid cell.
    LeftCosmos { x: usize, y: usize },
    /// The top-left rune was not a thrust rune.
    MissingInitialThrust,
}

impl fmt::Display for Termination {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halted => write!(formatter, "the vessel stopped"),
            Self::LeftCosmos { x, y } => write!(
                formatter,
                "the vessel traveled out of the cosmos (last coordinate: x = {x}, y = {y})"
            ),
            Self::MissingInitialThrust => {
                write!(
                    formatter,
                    "the top-left rune is not a thrust rune (^, v, <, >)"
                )
            }
        }
    }
}

/// Runs `vessel` through `cosmos` until it halts.
pub fn run(cosmos: &Cosmos, vessel: &mut Vessel, config: Config, io: &mut dyn Io) -> Termination {
    let termination = sail(cosmos, vessel, config, io);
    io.flush();
    termination
}

fn sail(cosmos: &Cosmos, vessel: &mut Vessel, config: Config, io: &mut dyn Io) -> Termination {
    if vessel.velocity() == 0 {
        return Termination::MissingInitialThrust;
    }

    while vessel.velocity() > 0 {
        let Some((x, y)) = vessel.next_coordinate() else {
            return Termination::LeftCosmos {
                x: vessel.x(),
                y: vessel.y(),
            };
        };

        if x >= cosmos.width() || y >= cosmos.height() {
            return Termination::LeftCosmos {
                x: x.min(cosmos.width().saturating_sub(1)),
                y: y.min(cosmos.height().saturating_sub(1)),
            };
        }

        let rune = cosmos.get(x, y);
        vessel.move_to(x, y);
        rune.apply(vessel, io);

        if rune == Rune::Debug && config.debug {
            io.write_diagnostic(&format!("[Debug] Vessel: {vessel:?}. Rune: {rune:?}"));
        }

        if config.trace && !(config.ignore_void && rune == Rune::Void) {
            io.write_diagnostic(&format!("Vessel: {vessel:?}. Rune: {rune:?}"));
        }
    }

    Termination::Halted
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SilentIo;

    impl Io for SilentIo {
        fn read_byte(&mut self) -> Option<u8> {
            None
        }

        fn write_char(&mut self, _character: char) {}

        fn write_diagnostic(&mut self, _message: &str) {}
    }

    fn run_source(source: &str) -> Termination {
        let cosmos = Cosmos::from_source(source);
        let mut vessel = Vessel::new(0, 0, cosmos.get(0, 0));
        let mut io = SilentIo;

        run(&cosmos, &mut vessel, Config::default(), &mut io)
    }

    #[test]
    fn program_without_initial_thrust_is_rejected() {
        assert_eq!(
            run_source("P"),
            Termination::MissingInitialThrust,
            "the top-left rune must provide an initial heading"
        );
    }

    #[test]
    fn zero_velocity_halts_the_program() {
        assert_eq!(run_source("><"), Termination::Halted);
    }

    #[test]
    fn leaving_the_cosmos_is_reported() {
        assert_eq!(run_source(">."), Termination::LeftCosmos { x: 1, y: 0 });
    }
}
