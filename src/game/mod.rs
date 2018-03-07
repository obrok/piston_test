extern crate rand;
extern crate std;

use self::rand::random;

const X_SIZE: u8 = 10;
const Y_SIZE: u8 = 10;

#[derive(Debug)]
pub struct Game {
    player: Position,
    obstacles: Vec<Position>,
    time: f64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Position {
                x: XPosition::middle(),
                y: YPosition::bottom(),
            },
            obstacles: vec![],
            time: 0.0,
        }
    }

    pub fn step(&mut self, dt: f64) {
        let previous_step = self.time.trunc();
        self.time += dt;
        let next_step = self.time.trunc();

        if next_step > previous_step {
            self.obstacles = self.obstacles.iter().filter_map(Position::down).collect();
            self.obstacles.push(random_obstacle());
        }
    }

    pub fn grid_height(&self) -> u8 {
        Y_SIZE
    }

    pub fn grid_width(&self) -> u8 {
        X_SIZE
    }

    pub fn obstacles(&self) -> Vec<(u8, u8)> {
        self.obstacles
            .iter()
            .map(
                |&Position {
                     x: XPosition(x),
                     y: YPosition(y),
                 }| (x, y),
            )
            .collect()
    }
}

#[derive(Debug)]
pub struct Position {
    x: XPosition,
    y: YPosition,
}

impl Position {
    fn down(&self) -> Option<Position> {
        self.y.down().map(|y| Position { x: self.x, y })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XPosition(u8);

impl XPosition {
    fn middle() -> XPosition {
        XPosition(X_SIZE / 2)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct YPosition(u8);

impl YPosition {
    fn bottom() -> YPosition {
        YPosition(0)
    }

    fn top() -> YPosition {
        YPosition(Y_SIZE - 1)
    }

    fn down(&self) -> Option<YPosition> {
        if self.0 <= 0 {
            None
        } else {
            Some(YPosition(self.0 - 1))
        }
    }
}

fn random_obstacle() -> Position {
    Position {
        x: XPosition(random::<u8>() % X_SIZE),
        y: YPosition::top(),
    }
}
