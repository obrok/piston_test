extern crate rand;
extern crate std;

use self::rand::random;

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
            for ref mut obstacle in self.obstacles.iter_mut() {
                obstacle.y.down()
            }
            self.obstacles.push(random_obstacle());
        }
    }

    pub fn grid_height(&self) -> u8 {
        10
    }

    pub fn grid_width(&self) -> u8 {
        10
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

#[derive(Debug)]
pub struct XPosition(u8);

impl XPosition {
    fn middle() -> XPosition {
        XPosition(5)
    }
}

#[derive(Debug)]
pub struct YPosition(u8);

impl YPosition {
    fn bottom() -> YPosition {
        YPosition(0)
    }

    fn top() -> YPosition {
        YPosition(9)
    }

    fn down(&mut self) {
        self.0 -= 1
    }
}

fn random_obstacle() -> Position {
    Position {
        x: XPosition(random::<u8>() % 10),
        y: YPosition::top(),
    }
}
