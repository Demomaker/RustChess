mod screen;
mod text;
mod displayable;
mod game;
mod chess;
mod game_state;

use screen::Screen;
use game::Game;

fn main() {
    // Create instances without using mut unnecessarily
    let screen = Screen::new();
    let mut game = Game::new(screen);

    game.run();
}
