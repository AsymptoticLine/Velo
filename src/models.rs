/// The fundamental elements in the Velo cosmos that affect the Vessel's movement.
#[derive(Debug, Clone, Copy)]
pub enum Rune {
    ThrustUp,    // '^'
    ThrustDown,  // 'v'
    ThrustLeft,  // '<'
    ThrustRight, // '>'
    Boost,       // '+'
    Brake,       // '-'
    Star,        // '*' - Reverses direction
    Parking,     // 'P' - Resets velocity to 1
    Void,        // Other characters - No effect
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

/// Describes the rotational change of the Vessel after impacting a Rune.
#[derive(Debug)]
pub enum Rotation {
    Straight, // No change in direction
    Left,
    Right,
    UTurn,   // 180-degree reversal
    Stopped, // Velocity dropped to zero
}

impl Direction {
    fn consistent_with(self, other: Direction) -> bool {
        self == other
    }

    fn opposite_to(self, other: Direction) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            (_, _) => false,
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

    pub fn calculate_rotation(self, new_direction: Direction) -> Rotation {
        match (self, new_direction) {
            (Direction::Up, Direction::Up)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Left)
            | (Direction::Right, Direction::Right) => Rotation::Straight,
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => Rotation::UTurn,
            (Direction::Up, Direction::Left)
            | (Direction::Left, Direction::Down)
            | (Direction::Down, Direction::Right)
            | (Direction::Right, Direction::Up) => Rotation::Left,
            (Direction::Up, Direction::Right)
            | (Direction::Right, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Left, Direction::Up) => Rotation::Right,
            _ => Rotation::Stopped,
        }
    }
}

/// The main execution entity, an exploration vessel moving through the Cosmos.
#[derive(Debug, Clone, Copy)]
pub struct Vessel {
    x: usize,
    y: usize,
    direction: Direction,
    velocity: i32,
}

impl Vessel {
    /// Creates a new Vessel at the starting coordinates, determining initial state based on the center Rune.
    pub fn new(x: usize, y: usize, starting_rune: Rune) -> Vessel {
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
        }
    }

    /// The Vessel impacts a Rune, modifying its state (direction and velocity).
    /// Returns the resulting Rotation for potential instruction execution.
    pub fn impact_rune(&mut self, rune: Rune) -> Rotation {
        match rune {
            Rune::ThrustUp => self.impact_thust_rune(Direction::Up),
            Rune::ThrustDown => self.impact_thust_rune(Direction::Down),
            Rune::ThrustLeft => self.impact_thust_rune(Direction::Left),
            Rune::ThrustRight => self.impact_thust_rune(Direction::Right),
            Rune::Boost => self.apply_boost(),
            Rune::Brake => self.apply_brake(),
            Rune::Star => self.apply_rebound(),
            Rune::Parking => self.apply_parking(),
            Rune::Void => Rotation::Straight,
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn velocity(&self) -> i32 {
        self.velocity
    }

    /// Calculates the expected next coordinate based on the current direction.
    /// Returns an error if the Vessel is moving out of bounds or has no direction.
    pub fn get_next_coordinate(self) -> Result<(usize, usize), &'static str> {
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

    fn apply_boost(&mut self) -> Rotation {
        self.velocity += 1;
        Rotation::Straight
    }

    fn apply_brake(&mut self) -> Rotation {
        self.velocity -= 1;
        if self.velocity > 0 {
            Rotation::Straight
        } else {
            Rotation::Stopped
        }
    }

    fn turn(&mut self, new_dir: Direction) -> Rotation {
        let rotation = self.direction.calculate_rotation(new_dir);
        self.direction = new_dir;
        rotation
    }

    fn apply_rebound(&mut self) -> Rotation {
        self.direction = self.direction.opposite();
        Rotation::UTurn
    }

    fn apply_parking(&mut self) -> Rotation {
        self.velocity = 1;
        Rotation::Straight
    }

    fn impact_thust_rune(&mut self, rune_dir: Direction) -> Rotation {
        if self.direction.consistent_with(rune_dir) {
            self.apply_boost()
        } else if self.direction.opposite_to(rune_dir) {
            self.apply_brake()
        } else {
            self.turn(rune_dir)
        }
    }
}
