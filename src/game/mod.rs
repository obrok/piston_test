#[derive(Debug)]
pub struct Game {
    player: Position,
    obstacles: Vec<Position>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Position {
                x: XPosition::middle(),
                y: YPosition::bottom(),
            },
            obstacles: vec![],
        }
    }
}

#[derive(Debug)]
struct Position {
    x: XPosition,
    y: YPosition,
}

#[derive(Debug)]
struct XPosition(u8);

impl XPosition {
    pub fn middle() -> XPosition {
        XPosition(5)
    }
}

#[derive(Debug)]
struct YPosition(u8);

impl YPosition {
    pub fn bottom() -> YPosition {
        YPosition(0)
    }
}
