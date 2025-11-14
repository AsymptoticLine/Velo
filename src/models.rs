use std::io::{self, Read};

/// The fundamental elements in the Velo cosmos that affect the Vessel's movement.
#[derive(Debug, Clone, Copy)]
pub enum Rune {
    ThrustUp,        // '^' - Changes direction/speed, changes Resonance Frequency (Pointer)
    ThrustDown,      // 'v' - Changes direction/speed, changes Resonance Frequency (Pointer)
    ThrustLeft,      // '<' - Changes direction/speed, affecting Resonance Frequency
    ThrustRight,     // '>' - Changes direction/speed, affecting Resonance Frequency
    Star,            // '*' - Reverses direction (Rebound)
    Parking,         // 'P' - Resets velocity to 1
    EntropyIncrease, // '+' - Increases current data cell's entropy level by 1
    EntropyDecrease, // '-' - Decreases current data cell's entropy level by 1
    SteerLeft,       // '[' - Conditional 90-degree left turn
    SteerRight,      // ']' - Conditional 90-degree right turn
    Input,           // ',' - Reads a byte from input to the current data cell
    Output,          // '.' - Prints the current data cell's value as an ASCII character
    Void,            // Other characters - No effect
}

impl Rune {
    /// Executes the action associated with this Rune on the Vessel.
    fn act_on(&self, vessel: &mut Vessel) {
        match self {
            Self::ThrustUp => vessel.apply_directional_thrust(Direction::Up),
            Self::ThrustDown => vessel.apply_directional_thrust(Direction::Down),
            Self::ThrustLeft => vessel.apply_directional_thrust(Direction::Left),
            Self::ThrustRight => vessel.apply_directional_thrust(Direction::Right),
            Self::Star => vessel.apply_rebound(),
            Self::Parking => vessel.apply_parking(),
            Self::EntropyIncrease => vessel.charge_entropy(),
            Self::EntropyDecrease => vessel.drain_entropy(),
            Self::SteerLeft => {
                // If Entropy Level is NOT zero (i.e., NOT stable), force a left turn.
                if !vessel.is_stable() {
                    vessel.rotate_vessel(Rotation::Left);
                }
            }
            Self::SteerRight => {
                // If Entropy Level is NOT zero (i.e., NOT stable), force a right turn.
                if !vessel.is_stable() {
                    vessel.rotate_vessel(Rotation::Right);
                }
            }
            Self::Input => {
                // Reads the first available byte from stdin into the current data cell.
                let mut buffer = [0; 1];

                match io::stdin().read_exact(&mut buffer) {
                    Ok(_) => {
                        vessel.set_entropy_level(buffer[0] as i32);
                    }
                    Err(_) => {
                        // On EOF or read error, set the cell value to 0.
                        vessel.set_entropy_level(0);
                    }
                }
            }
            Self::Output => {
                // Prints the current data cell's entropy level as an ASCII character.
                let value = vessel.current_entropy();
                if let Some(c) = char::from_u32(value as u32) {
                    print!("{}", c);
                } else {
                    eprintln!("Velo Warning: Cannot output valid ASCII value: {}", value);
                }
            }
            Self::Void => (),
        }
    }
}

/// The Velo universe, represented as a grid of Runes.
pub type Cosmos = Vec<Vec<Rune>>;

/// The direction of the Vessel's travel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None, // Initial state or stopped
}

impl Direction {
    fn to_i32(self) -> i32 {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
            Self::None => -1,
        }
    }

    fn from_i32(n: i32) -> Self {
        match n {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => Self::None,
        }
    }

    fn consistent_with(self, other: Direction) -> bool {
        self == other
    }

    fn opposite_to(self, other: Direction) -> bool {
        match (self, other) {
            (Self::Up, Self::Down) => true,
            (Self::Down, Self::Up) => true,
            (Self::Right, Self::Left) => true,
            (Self::Left, Self::Right) => true,
            _ => false,
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }

    /*
    fn calculate_rotation(self, new_direction: Direction) -> Rotation {
        Rotation::from_i32((self.to_i32() - new_direction.to_i32() + 4) % 4)
    }
    // */

    fn rotate(self, rotation: Rotation) -> Direction {
        Direction::from_i32((self.to_i32() + rotation.to_i32()) % 4)
    }
}

/// Describes the rotational change of the Vessel after impacting a Rune.
#[derive(Debug)]
pub enum Rotation {
    Straight, // No change in direction
    Right,
    UTurn, // 180-degree reversal
    Left,
    None,
}

impl Rotation {
    fn to_i32(&self) -> i32 {
        match self {
            Self::Straight => 0,
            Self::Right => 1,
            Self::UTurn => 2,
            Self::Left => 3,
            Self::None => -1,
        }
    }

    /*
    fn from_i32(n: i32) -> Self {
        match n {
            0 => Self::Straight,
            1 => Self::Right,
            2 => Self::UTurn,
            3 => Self::Left,
            _ => Self::None,
        }
    }
    // */
}

/// The main execution entity, an exploration vessel moving through the Cosmos.
#[derive(Debug, Clone)]
pub struct Vessel {
    x: usize,
    y: usize,
    direction: Direction,
    // velocity serves as the Cosmic Resonance Frequency (data pointer).
    // The Vessel's physical movement step size is always 1, regardless of this value.
    velocity: usize,
    // The potentially infinite data storage (Data Lattice).
    data_lattice: Vec<i32>,
}

impl Vessel {
    /// Creates a new Vessel at the starting coordinates.
    pub fn new(x: usize, y: usize, starting_rune: Rune) -> Vessel {
        // Initial direction and velocity are determined by the top left corner Rune.
        let (direction, velocity) = match starting_rune {
            Rune::ThrustUp => (Direction::Up, 1),
            Rune::ThrustDown => (Direction::Down, 1),
            Rune::ThrustLeft => (Direction::Left, 1),
            Rune::ThrustRight => (Direction::Right, 1),
            _ => (Direction::None, 0),
        };
        Vessel {
            x,
            y,
            direction,
            velocity,
            // Initializes the data lattice with 16 starting data cells.
            data_lattice: vec![0; 16],
        }
    }

    // --- Accessors ---

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn velocity(&self) -> usize {
        self.velocity
    }

    // --- Data Lattice Management ---

    fn check_and_expand_data_lattice(&mut self) {
        if self.velocity >= self.data_lattice.len() {
            self.data_lattice.resize_with(self.velocity + 16, || 0);
        }
    }

    pub fn current_entropy(&mut self) -> i32 {
        self.check_and_expand_data_lattice();
        self.data_lattice[self.velocity]
    }

    pub fn set_entropy_level(&mut self, new_entropy_level: i32) {
        self.check_and_expand_data_lattice();
        self.data_lattice[self.velocity] = new_entropy_level;
    }

    pub fn is_stable(&mut self) -> bool {
        self.current_entropy() == 0
    }

    // --- Movement and Velocity/Pointer Modification ---

    // Note: All movement methods ensure the Vessel only moves 1 unit per execution cycle.

    /// The Vessel impacts a Rune, modifying its state (direction and velocity).
    pub fn impact_rune(&mut self, rune: Rune) {
        rune.act_on(self);
    }

    fn increase_velority(&mut self) {
        // Increases the Resonance Frequency (moves the data pointer right).
        self.velocity += 1;
    }

    fn decrease_velocity(&mut self) {
        // Decreases the Resonance Frequency (moves the data pointer left).
        self.velocity -= 1;
    }

    fn charge_entropy(&mut self) {
        // Increases the entropy level of the current data cell by 1.
        let new_entropy_level = self.current_entropy() + 1;
        self.set_entropy_level(new_entropy_level);
    }

    fn drain_entropy(&mut self) {
        // Decreases the entropy level of the current data cell by 1.
        let new_entropy_level = self.current_entropy() - 1;
        self.set_entropy_level(new_entropy_level);
    }

    fn apply_rebound(&mut self) {
        // Reverses the vessel's direction (180-degree turn).
        self.direction = self.direction.opposite();
    }

    fn apply_parking(&mut self) {
        // Resets the velocity/pointer to the base value.
        self.velocity = 1;
    }

    fn turn_to(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    fn rotate_vessel(&mut self, rotation: Rotation) {
        self.direction = self.direction.rotate(rotation);
    }

    /// Handles the impact with a Thrust Rune (`^v<>`), modifying both movement and velocity/pointer.
    fn apply_directional_thrust(&mut self, rune_direction: Direction) {
        if self.direction.consistent_with(rune_direction) {
            // Same direction: Increase velocity/pointer.
            self.increase_velority();
        } else if self.direction.opposite_to(rune_direction) {
            // Opposite direction: Decrease velocity/pointer.
            self.decrease_velocity();
        } else {
            // Perpendicular direction: Turn, velocity/pointer is unchanged.
            self.turn_to(rune_direction);
        }
    }

    /// Calculates the expected next coordinate based on the current direction.
    /// Returns an error if the Vessel is moving out of bounds or has no direction.
    pub fn get_next_coordinate(&self) -> Result<(usize, usize), &'static str> {
        match self.direction {
            Direction::Up => {
                if self.y < 1 {
                    return Err(
                        "`y` is less than 1, the vessel was going to travel out of the cosmos.",
                    );
                }
                Ok((self.x, self.y - 1))
            }
            Direction::Down => Ok((self.x, self.y + 1)),
            Direction::Left => {
                if self.x < 1 {
                    return Err(
                        "`x` is less than 1, the vessel was going to travel out of the cosmos.",
                    );
                }
                Ok((self.x - 1, self.y))
            }
            Direction::Right => Ok((self.x + 1, self.y)),
            Direction::None => Err("No direction."),
        }
    }

    pub fn move_to(&mut self, new_x: usize, new_y: usize) {
        self.x = new_x;
        self.y = new_y;
    }

    pub fn move_forward(&mut self) -> Result<(), &str> {
        let (new_x, new_y) = self.get_next_coordinate()?;
        self.move_to(new_x, new_y);
        Ok(())
    }
}
