//! The moving program state.

use crate::direction::{Direction, Rotation};
use crate::rune::Rune;

/// Number of data cells allocated when the vessel is created.
const INITIAL_LATTICE_SIZE: usize = 16;
/// Number of extra cells allocated whenever the lattice needs to grow.
const LATTICE_GROWTH: usize = 16;

/// The vessel's position, heading, velocity, and expandable data lattice.
///
/// `velocity` doubles as the data pointer. It is also the halt condition: the
/// program stops once velocity reaches zero.
#[derive(Debug, Clone)]
pub struct Vessel {
    x: usize,
    y: usize,
    direction: Option<Direction>,
    velocity: usize,
    data_lattice: Vec<u32>,
}

impl Vessel {
    /// Creates a vessel at `(x, y)`.
    ///
    /// The starting rune supplies the initial heading. If it is not a thrust
    /// rune the vessel has no heading and zero velocity, so execution cannot
    /// start.
    pub fn new(x: usize, y: usize, start_rune: Rune) -> Self {
        let direction = start_rune.thrust_direction();

        Self {
            x,
            y,
            direction,
            velocity: if direction.is_some() { 1 } else { 0 },
            data_lattice: vec![0; INITIAL_LATTICE_SIZE],
        }
    }

    pub const fn x(&self) -> usize {
        self.x
    }

    pub const fn y(&self) -> usize {
        self.y
    }

    pub const fn direction(&self) -> Option<Direction> {
        self.direction
    }

    pub const fn velocity(&self) -> usize {
        self.velocity
    }

    /// Returns the entropy level of the current data cell.
    pub fn entropy(&mut self) -> u32 {
        self.ensure_cell();
        self.data_lattice[self.velocity]
    }

    /// Sets the entropy level of the current data cell.
    pub fn set_entropy(&mut self, value: u32) {
        self.ensure_cell();
        self.data_lattice[self.velocity] = value;
    }

    /// A cell is stable when its entropy level is zero.
    pub(crate) fn is_stable(&mut self) -> bool {
        self.entropy() == 0
    }

    pub(crate) fn charge_entropy(&mut self) {
        let value = self.entropy() + 1;
        self.set_entropy(value);
    }

    pub(crate) fn drain_entropy(&mut self) {
        let value = self.entropy();
        if value > 0 {
            self.set_entropy(value - 1);
        }
    }

    /// Resets the data pointer to 1.
    pub(crate) fn park(&mut self) {
        self.velocity = 1;
    }

    pub(crate) fn rotate(&mut self, rotation: Rotation) {
        if let Some(direction) = self.direction {
            self.direction = Some(direction.rotate(rotation));
        }
    }

    /// Applies a thrust rune to the vessel.
    ///
    /// Thrusting in the current direction increases velocity, thrusting in the
    /// opposite direction decreases it, and thrusting perpendicular turns the
    /// vessel without changing velocity.
    pub(crate) fn apply_thrust(&mut self, rune_direction: Direction) {
        match self.direction {
            Some(current) if current == rune_direction => self.velocity += 1,
            Some(current) if current.is_opposite(rune_direction) => {
                self.velocity = self.velocity.saturating_sub(1);
            }
            Some(_) => self.direction = Some(rune_direction),
            None => {
                // Defensive: allow a heading-less vessel to start moving.
                self.direction = Some(rune_direction);
                self.velocity = 1;
            }
        }
    }

    /// The next cell in the current heading, or `None` at a grid edge.
    pub fn next_coordinate(&self) -> Option<(usize, usize)> {
        let direction = self.direction?;
        let (dx, dy) = direction.delta();

        Some((
            self.x.checked_add_signed(dx)?,
            self.y.checked_add_signed(dy)?,
        ))
    }

    pub(crate) fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    /// Grows the lattice so that `velocity` is a valid index.
    fn ensure_cell(&mut self) {
        if self.velocity >= self.data_lattice.len() {
            self.data_lattice.resize(self.velocity + LATTICE_GROWTH, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vessel_heading(direction: Direction) -> Vessel {
        Vessel::new(0, 0, Rune::Thrust(direction))
    }

    #[test]
    fn starting_thrust_rune_sets_heading_and_velocity() {
        let vessel = vessel_heading(Direction::Right);

        assert_eq!(vessel.direction(), Some(Direction::Right));
        assert_eq!(vessel.velocity(), 1);
    }

    #[test]
    fn non_thrust_starting_rune_has_no_velocity() {
        let vessel = Vessel::new(0, 0, Rune::Park);

        assert_eq!(vessel.direction(), None);
        assert_eq!(vessel.velocity(), 0);
    }

    #[test]
    fn thrusting_forward_accelerates() {
        let mut vessel = vessel_heading(Direction::Right);

        vessel.apply_thrust(Direction::Right);
        vessel.apply_thrust(Direction::Right);

        assert_eq!(vessel.velocity(), 3);
        assert_eq!(vessel.direction(), Some(Direction::Right));
    }

    #[test]
    fn thrusting_backwards_decelerates_without_underflow() {
        let mut vessel = vessel_heading(Direction::Right);

        vessel.apply_thrust(Direction::Left);
        vessel.apply_thrust(Direction::Left);

        assert_eq!(vessel.velocity(), 0);
        assert_eq!(vessel.direction(), Some(Direction::Right));
    }

    #[test]
    fn perpendicular_thrust_turns_without_changing_velocity() {
        let mut vessel = vessel_heading(Direction::Right);

        vessel.apply_thrust(Direction::Down);

        assert_eq!(vessel.velocity(), 1);
        assert_eq!(vessel.direction(), Some(Direction::Down));
    }

    #[test]
    fn parking_resets_the_data_pointer() {
        let mut vessel = vessel_heading(Direction::Right);
        vessel.apply_thrust(Direction::Right);
        vessel.apply_thrust(Direction::Right);

        vessel.park();

        assert_eq!(vessel.velocity(), 1);
    }

    #[test]
    fn entropy_is_kept_per_data_cell() {
        let mut vessel = vessel_heading(Direction::Right);

        vessel.set_entropy(7);
        vessel.apply_thrust(Direction::Right);
        vessel.set_entropy(9);

        assert_eq!(vessel.entropy(), 9);
        vessel.park();
        assert_eq!(vessel.entropy(), 7);
    }

    #[test]
    fn data_lattice_grows_with_the_pointer() {
        let mut vessel = vessel_heading(Direction::Right);
        for _ in 0..100 {
            vessel.apply_thrust(Direction::Right);
        }

        vessel.set_entropy(42);

        assert_eq!(vessel.velocity(), 101);
        assert_eq!(vessel.entropy(), 42);
    }

    #[test]
    fn next_coordinate_follows_the_heading() {
        let mut vessel = vessel_heading(Direction::Right);
        vessel.move_to(3, 4);
        assert_eq!(vessel.next_coordinate(), Some((4, 4)));

        vessel.rotate(Rotation::Right);
        assert_eq!(vessel.next_coordinate(), Some((3, 5)));
    }

    #[test]
    fn next_coordinate_is_none_at_the_upper_or_left_edge() {
        let vessel = vessel_heading(Direction::Up);
        assert_eq!(vessel.next_coordinate(), None);

        let vessel = vessel_heading(Direction::Left);
        assert_eq!(vessel.next_coordinate(), None);
    }
}
