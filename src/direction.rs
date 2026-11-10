//! Compass directions and quarter-turn rotations used by the vessel.

/// A cardinal direction in the cosmos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// A turn applied to a [`Direction`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Straight,
    Right,
    UTurn,
    Left,
}

impl Direction {
    /// The `(x, y)` offset produced by moving one cell in this direction.
    ///
    /// The y axis grows downwards, matching the source layout.
    pub const fn delta(self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    /// Returns `true` when `other` points the opposite way.
    pub const fn is_opposite(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Up, Self::Down)
                | (Self::Down, Self::Up)
                | (Self::Left, Self::Right)
                | (Self::Right, Self::Left)
        )
    }

    /// Rotates this direction by `rotation`.
    pub const fn rotate(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Straight => self,
            Rotation::Right => match self {
                Self::Up => Self::Right,
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
            },
            Rotation::UTurn => match self {
                Self::Up => Self::Down,
                Self::Down => Self::Up,
                Self::Left => Self::Right,
                Self::Right => Self::Left,
            },
            Rotation::Left => match self {
                Self::Up => Self::Left,
                Self::Left => Self::Down,
                Self::Down => Self::Right,
                Self::Right => Self::Up,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    #[test]
    fn right_rotation_turns_clockwise() {
        assert_eq!(Direction::Up.rotate(Rotation::Right), Direction::Right);
        assert_eq!(Direction::Right.rotate(Rotation::Right), Direction::Down);
        assert_eq!(Direction::Down.rotate(Rotation::Right), Direction::Left);
        assert_eq!(Direction::Left.rotate(Rotation::Right), Direction::Up);
    }

    #[test]
    fn left_rotation_is_the_inverse_of_right_rotation() {
        for direction in DIRECTIONS {
            assert_eq!(
                direction.rotate(Rotation::Left).rotate(Rotation::Right),
                direction
            );
        }
    }

    #[test]
    fn u_turn_reverses_direction() {
        assert_eq!(Direction::Up.rotate(Rotation::UTurn), Direction::Down);
        assert_eq!(Direction::Left.rotate(Rotation::UTurn), Direction::Right);
    }

    #[test]
    fn opposite_directions_are_detected() {
        assert!(Direction::Up.is_opposite(Direction::Down));
        assert!(Direction::Left.is_opposite(Direction::Right));
        assert!(!Direction::Up.is_opposite(Direction::Left));
        assert!(!Direction::Up.is_opposite(Direction::Up));
    }

    #[test]
    fn deltas_match_the_source_layout() {
        assert_eq!(Direction::Up.delta(), (0, -1));
        assert_eq!(Direction::Down.delta(), (0, 1));
        assert_eq!(Direction::Left.delta(), (-1, 0));
        assert_eq!(Direction::Right.delta(), (1, 0));
    }
}
