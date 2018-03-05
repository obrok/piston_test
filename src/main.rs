extern crate piston_test;
extern crate piston_window;

use piston_test::game::Game;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .expect("Failed to initialize window");
    let mut game = Game::new();

    while let Some(event) = window.next() {
        if let Some(UpdateArgs { dt }) = event.update_args() {
            game.step(dt);
            println!("{:?}", game);
        }

        window.draw_2d(&event, |context, graphics| {
            piston_window::clear([1.0, 1.0, 1.0, 1.0], graphics);
            piston_window::rectangle(
                [0.5, 0.5, 0.0, 1.0],
                [0.0, 0.5, 100.0, 100.5],
                context.transform,
                graphics,
            );
        });
    }
}
