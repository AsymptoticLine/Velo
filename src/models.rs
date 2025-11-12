#[derive(Debug, Clone, Copy)]
pub enum Rune {
    ThrustUp,
    ThrustDown,
    ThrustLeft,
    ThrustRight,
    Boost,
    Brake,
    Star,
    Parking,
    Void,
}

pub type Cosmos = Vec<Vec<Rune>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug)]
pub enum Rotation {
    Straight,
    Left,
    Right,
    UTurn,
    Stopped,
}

impl Dir {
    fn consistent_with(self, other: Dir) -> bool {
        self == other
    }

    fn opposite_to(self, other: Dir) -> bool {
        match (self, other) {
            (Dir::Up, Dir::Down) => true,
            (Dir::Down, Dir::Up) => true,
            (Dir::Left, Dir::Right) => true,
            (Dir::Right, Dir::Left) => true,
            (_, _) => false,
        }
    }

    fn opposite(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::None => Dir::None,
        }
    }

    pub fn rotation(self, new_dir: Dir) -> Rotation {
        match (self, new_dir) {
            (Dir::Up, Dir::Up)
            | (Dir::Down, Dir::Down)
            | (Dir::Left, Dir::Left)
            | (Dir::Right, Dir::Right) => Rotation::Straight,
            (Dir::Up, Dir::Down)
            | (Dir::Down, Dir::Up)
            | (Dir::Left, Dir::Right)
            | (Dir::Right, Dir::Left) => Rotation::UTurn,
            (Dir::Up, Dir::Left)
            | (Dir::Left, Dir::Down)
            | (Dir::Down, Dir::Right)
            | (Dir::Right, Dir::Up) => Rotation::Left,
            (Dir::Up, Dir::Right)
            | (Dir::Right, Dir::Down)
            | (Dir::Down, Dir::Left)
            | (Dir::Left, Dir::Up) => Rotation::Right,
            _ => Rotation::Stopped,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vessel {
    x: usize,
    y: usize,
    dir: Dir,
    vel: i32,
}

impl Vessel {
    pub fn new(x: usize, y: usize, starting_rune: Rune) -> Vessel {
        let (dir, vel) = match starting_rune {
            Rune::ThrustUp => (Dir::Up, 1),
            Rune::ThrustDown => (Dir::Down, 1),
            Rune::ThrustLeft => (Dir::Left, 1),
            Rune::ThrustRight => (Dir::Right, 1),
            _ => (Dir::None, 0),
        };
        Vessel { x, y, dir, vel }
    }

    pub fn meet_rune(&mut self, rune: Rune) -> Rotation {
        match rune {
            Rune::ThrustUp => self.meet_thust(Dir::Up),
            Rune::ThrustDown => self.meet_thust(Dir::Down),
            Rune::ThrustLeft => self.meet_thust(Dir::Left),
            Rune::ThrustRight => self.meet_thust(Dir::Right),
            Rune::Boost => self.boost(),
            Rune::Brake => self.brake(),
            Rune::Star => self.rebound(),
            Rune::Parking => self.park(),
            Rune::Void => Rotation::Straight,
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn dir(&self) -> Dir {
        self.dir
    }

    pub fn vel(&self) -> i32 {
        self.vel
    }

    pub fn get_next_coordinate(self) -> Result<(usize, usize), &'static str> {
        match self.dir {
            Dir::Up => {
                if self.y < 1 {
                    return Err(
                        "`y` is less than 1, the vessel was going to travel out of the cosmos.",
                    );
                }
                Ok((self.x, self.y - 1))
            }
            Dir::Down => Ok((self.x, self.y + 1)),
            Dir::Left => {
                if self.x < 1 {
                    return Err(
                        "`x` is less than 1, the vessel was going to travel out of the cosmos.",
                    );
                }
                Ok((self.x - 1, self.y))
            }
            Dir::Right => Ok((self.x + 1, self.y)),
            Dir::None => Err("No direction."),
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

    fn boost(&mut self) -> Rotation {
        self.vel += 1;
        Rotation::Straight
    }

    fn brake(&mut self) -> Rotation {
        self.vel -= 1;
        if self.vel > 0 {
            Rotation::Straight
        } else {
            Rotation::Stopped
        }
    }

    fn turn(&mut self, new_dir: Dir) -> Rotation {
        let rotation = self.dir.rotation(new_dir);
        self.dir = new_dir;
        rotation
    }

    fn rebound(&mut self) -> Rotation {
        self.dir = self.dir.opposite();
        Rotation::UTurn
    }

    fn park(&mut self) -> Rotation {
        self.vel = 1;
        Rotation::Straight
    }

    fn meet_thust(&mut self, rune_dir: Dir) -> Rotation {
        if self.dir.consistent_with(rune_dir) {
            self.boost()
        } else if self.dir.opposite_to(rune_dir) {
            self.brake()
        } else {
            self.turn(rune_dir)
        }
    }
}
