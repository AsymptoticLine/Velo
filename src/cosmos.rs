//! The grid of runes a vessel travels through.

use crate::rune::Rune;

/// A parsed Velo program, stored as a grid of runes.
#[derive(Debug, Clone)]
pub struct Cosmos {
    runes: Vec<Vec<Rune>>,
    width: usize,
    height: usize,
}

impl Cosmos {
    /// Builds a cosmos from Velo source code.
    ///
    /// Everything from the first `#` on a line is a comment. Rows may have
    /// different lengths; cells outside a row behave like [`Rune::Void`].
    pub fn from_source(source: &str) -> Self {
        let runes: Vec<Vec<Rune>> = source
            .lines()
            .map(|line| {
                line.split_once('#')
                    .map_or(line, |(code, _)| code)
                    .chars()
                    .map(Rune::from_char)
                    .collect()
            })
            .collect();

        let height = runes.len();
        let width = runes.iter().map(Vec::len).max().unwrap_or(0);

        Self {
            runes,
            width,
            height,
        }
    }

    /// The width of the widest row.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// The number of rows.
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the rune at `(x, y)`, or [`Rune::Void`] outside the grid.
    pub fn get(&self, x: usize, y: usize) -> Rune {
        self.runes
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(Rune::Void)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;

    #[test]
    fn strips_comments_and_measures_the_grid() {
        let cosmos = Cosmos::from_source(">+ # a comment\n# only a comment\n  v");

        assert_eq!(cosmos.width(), 3);
        assert_eq!(cosmos.height(), 3);
        assert_eq!(cosmos.get(0, 0), Rune::Thrust(Direction::Right));
        assert_eq!(cosmos.get(1, 0), Rune::EntropyCharge);
        assert_eq!(cosmos.get(2, 0), Rune::Void);
        assert_eq!(cosmos.get(2, 2), Rune::Thrust(Direction::Down));
    }

    #[test]
    fn out_of_bounds_cells_are_void() {
        let cosmos = Cosmos::from_source(">");

        assert_eq!(cosmos.get(0, 0), Rune::Thrust(Direction::Right));
        assert_eq!(cosmos.get(1, 0), Rune::Void);
        assert_eq!(cosmos.get(0, 1), Rune::Void);
        assert_eq!(cosmos.get(100, 100), Rune::Void);
    }

    #[test]
    fn empty_source_has_no_cells() {
        let cosmos = Cosmos::from_source("");

        assert_eq!(cosmos.width(), 0);
        assert_eq!(cosmos.height(), 0);
        assert_eq!(cosmos.get(0, 0), Rune::Void);
    }
}
