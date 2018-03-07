extern crate piston_test;
extern crate piston_window;

use piston_test::game::Game;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Crappy game", [1024, 1024])
        .exit_on_esc(true)
        .build()
        .expect("Failed to initialize window");
    let mut game = Game::new();

    while let Some(event) = window.next() {
        if let Some(UpdateArgs { dt }) = event.update_args() {
            game.step(dt);
        }

        if let Some(RenderArgs { width, height, .. }) = event.render_args() {
            window.draw_2d(&event, |context, graphics| {
                let width = width as f64;
                let height = height as f64;
                let grid_x_step = width / (game.grid_width() as f64);
                let grid_y_step = height / (game.grid_height() as f64);

                piston_window::clear([0.0; 4], graphics);
                for (x, y) in game.obstacles() {
                    let x = x as f64;
                    let y = y as f64;

                    piston_window::rectangle(
                        [0.5, 0.5, 0.0, 1.0],
                        [
                            grid_x_step * (game.grid_width() as f64 - x - 1.0),
                            grid_y_step * (game.grid_height() as f64 - y - 1.0),
                            grid_x_step,
                            grid_y_step,
                        ],
                        context.transform,
                        graphics,
                    );
                }
            });
        }
    }
}
