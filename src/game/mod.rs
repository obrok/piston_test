extern crate rand;
extern crate std;

use self::rand::random;

const X_SIZE: u8 = 10;
const Y_SIZE: u8 = 10;
const DIFFICULTY_GRADIENT: f64 = 1.25;

#[derive(Debug)]
pub enum Game {
    InProgress(InProgressGame),
    Lost(LostGame),
}

impl Game {
    pub fn new() -> Game {
        Game::InProgress(InProgressGame::new())
    }

    pub fn step(self, dt: f64) -> Game {
        self.flat_map(|game| game.step(dt))
    }

    pub fn left(self) -> Game {
        self.flat_map(|game| game.left())
    }

    pub fn right(self) -> Game {
        self.flat_map(|game| game.right())
    }

    pub fn in_progress(&self) -> Option<&InProgressGame> {
        match self {
            &Game::Lost(_) => None,
            &Game::InProgress(ref game) => Some(game),
        }
    }

    fn flat_map<F>(self, f: F) -> Game
    where
        F: Fn(InProgressGame) -> Game,
    {
        match self {
            Game::Lost(game) => Game::Lost(game),
            Game::InProgress(game) => f(game),
        }
    }
}

#[derive(Debug)]
pub struct InProgressGame {
    player: Position,
    obstacles: Vec<Position>,
    time: f64,
}

#[derive(Debug)]
pub struct LostGame {
    score: u64,
}

impl InProgressGame {
    pub fn new() -> InProgressGame {
        InProgressGame {
            player: Position {
                x: XPosition::middle(),
                y: YPosition::bottom(),
            },
            obstacles: vec![],
            time: 0.0,
        }
    }

    pub fn step(mut self, dt: f64) -> Game {
        let previous_step = self.time.powf(DIFFICULTY_GRADIENT).trunc();
        let next_step = (self.time + dt).powf(DIFFICULTY_GRADIENT).trunc();
        self.time += dt;

        if next_step > previous_step {
            self.obstacles = self.obstacles.iter().filter_map(Position::down).collect();
            self.obstacles.push(random_obstacle());
            InProgressGame::check_lost(self)
        } else {
            Game::InProgress(self)
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

    pub fn player(&self) -> (u8, u8) {
        let Position {
            x: XPosition(x),
            y: YPosition(y),
        } = self.player;
        (x, y)
    }

    pub fn left(self) -> Game {
        if let Some(player) = self.player.left() {
            InProgressGame::check_lost(InProgressGame {
                player: player,
                ..self
            })
        } else {
            Game::InProgress(self)
        }
    }

    pub fn right(self) -> Game {
        if let Some(player) = self.player.right() {
            InProgressGame::check_lost(InProgressGame {
                player: player,
                ..self
            })
        } else {
            Game::InProgress(self)
        }
    }

    fn check_lost(self) -> Game {
        if self.obstacles.iter().any(|o| o == &self.player) {
            Game::Lost(LostGame { score: 0 })
        } else {
            Game::InProgress(self)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Position {
    x: XPosition,
    y: YPosition,
}

impl Position {
    fn down(&self) -> Option<Position> {
        self.y.down().map(|y| Position { x: self.x, y })
    }

    fn left(&self) -> Option<Position> {
        self.x.left().map(|x| Position { x, y: self.y })
    }

    fn right(&self) -> Option<Position> {
        self.x.right().map(|x| Position { x, y: self.y })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XPosition(u8);

impl XPosition {
    fn middle() -> XPosition {
        XPosition(X_SIZE / 2)
    }

    fn left(&self) -> Option<XPosition> {
        if self.0 <= 0 {
            None
        } else {
            Some(XPosition(self.0 - 1))
        }
    }

    fn right(&self) -> Option<XPosition> {
        if self.0 >= X_SIZE - 1 {
            None
        } else {
            Some(XPosition(self.0 + 1))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
