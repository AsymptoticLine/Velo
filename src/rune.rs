//! The instruction set of Velo.

use crate::direction::{Direction, Rotation};
use crate::io::Io;
use crate::vessel::Vessel;

/// A single instruction in a Velo program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rune {
    /// `^v<>` — accelerate, decelerate, or turn depending on the heading.
    Thrust(Direction),
    /// `P` — reset velocity to 1.
    Park,
    /// `+` — increase the current cell's entropy level.
    EntropyCharge,
    /// `-` — decrease the current cell's entropy level.
    EntropyDrain,
    /// `[` — turn left when the current cell is not stable.
    SteerLeft,
    /// `]` — turn right when the current cell is not stable.
    SteerRight,
    /// `,` — read one byte from input into the current cell.
    Input,
    /// `.` — write the current cell's entropy level as a character.
    Output,
    /// `D` — print the vessel state when `--debug` is enabled.
    Debug,
    /// Anything else — no effect.
    Void,
}

impl Rune {
    /// Maps a source character to its rune.
    pub const fn from_char(character: char) -> Self {
        match character {
            '^' => Self::Thrust(Direction::Up),
            'v' => Self::Thrust(Direction::Down),
            '<' => Self::Thrust(Direction::Left),
            '>' => Self::Thrust(Direction::Right),
            'P' => Self::Park,
            '+' => Self::EntropyCharge,
            '-' => Self::EntropyDrain,
            '[' => Self::SteerLeft,
            ']' => Self::SteerRight,
            ',' => Self::Input,
            '.' => Self::Output,
            'D' => Self::Debug,
            _ => Self::Void,
        }
    }

    /// The heading encoded by a thrust rune, if this is one.
    pub const fn thrust_direction(self) -> Option<Direction> {
        match self {
            Self::Thrust(direction) => Some(direction),
            _ => None,
        }
    }

    /// Applies this rune's effect to the vessel.
    pub(crate) fn apply(self, vessel: &mut Vessel, io: &mut dyn Io) {
        match self {
            Self::Thrust(direction) => vessel.apply_thrust(direction),
            Self::Park => vessel.park(),
            Self::EntropyCharge => vessel.charge_entropy(),
            Self::EntropyDrain => vessel.drain_entropy(),
            Self::SteerLeft => {
                if !vessel.is_stable() {
                    vessel.rotate(Rotation::Left);
                }
            }
            Self::SteerRight => {
                if !vessel.is_stable() {
                    vessel.rotate(Rotation::Right);
                }
            }
            Self::Input => vessel.set_entropy(io.read_byte().map_or(0, u32::from)),
            Self::Output => {
                let value = vessel.entropy();
                match char::from_u32(value) {
                    Some(character) => io.write_char(character),
                    None => io.write_diagnostic(&format!(
                        "velo: cannot output the invalid character value {value}"
                    )),
                }
            }
            Self::Debug | Self::Void => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_runes_symbol() {
        let cases = [
            ('^', Rune::Thrust(Direction::Up)),
            ('v', Rune::Thrust(Direction::Down)),
            ('<', Rune::Thrust(Direction::Left)),
            ('>', Rune::Thrust(Direction::Right)),
            ('P', Rune::Park),
            ('+', Rune::EntropyCharge),
            ('-', Rune::EntropyDrain),
            ('[', Rune::SteerLeft),
            (']', Rune::SteerRight),
            (',', Rune::Input),
            ('.', Rune::Output),
            ('D', Rune::Debug),
            (' ', Rune::Void),
            ('|', Rune::Void),
        ];

        for (symbol, rune) in cases {
            assert_eq!(Rune::from_char(symbol), rune);
        }
    }

    #[test]
    fn only_thrust_runes_carry_a_direction() {
        assert_eq!(Rune::from_char('^').thrust_direction(), Some(Direction::Up));
        assert_eq!(Rune::from_char('P').thrust_direction(), None);
    }
}
