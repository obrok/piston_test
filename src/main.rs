extern crate piston_test;
extern crate piston_window;

use piston_test::game::{Game, InProgressGame, LostGame};
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Crappy game", [1024, 1024])
        .exit_on_esc(true)
        .build()
        .expect("Failed to initialize window");
    let mut game = Game::new();

    while let Some(event) = window.next() {
        if let Some(UpdateArgs { dt }) = event.update_args() {
            game = game.step(dt);
        }

        if let Some(render_args) = event.render_args() {
            if let Some(ref game) = game.in_progress() {
                window.draw_2d(&event, |context, graphics| {
                    piston_window::clear([0.0; 4], graphics);
                    for position in game.obstacles() {
                        piston_window::rectangle(
                            [1.0, 0.0, 0.0, 1.0],
                            rectangle(&render_args, &game, position),
                            context.transform,
                            graphics,
                        );
                    }

                    piston_window::rectangle(
                        [0.0, 1.0, 0.0, 1.0],
                        rectangle(&render_args, &game, game.player()),
                        context.transform,
                        graphics,
                    );
                });
            }
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Left {} => game = game.left(),
                Key::Right {} => game = game.right(),
                _ => (),
            }
        }

        if let Game::Lost(LostGame { score }) = game {
            println!("You scored {}", score);
            window.set_should_close(true);
        }
    }
}

fn rectangle(render_args: &RenderArgs, game: &InProgressGame, (x, y): (u8, u8)) -> [f64; 4] {
    let width = render_args.width as f64;
    let height = render_args.height as f64;
    let grid_x_step = width / (game.grid_width() as f64);
    let grid_y_step = height / (game.grid_height() as f64);
    let x = x as f64;
    let y = y as f64;

    [
        grid_x_step * x,
        grid_y_step * (game.grid_height() as f64 - y - 1.0),
        grid_x_step,
        grid_y_step,
    ]
}
