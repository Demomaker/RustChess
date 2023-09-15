use crate::game_state::GameState;
use crate::screen::Screen;
use crate::text::Text;
use std::rc::Rc;
use rand::Rng;

pub struct Chess {
    screen: Rc<Screen>,
}

impl Chess {
    pub fn new(screen: Rc<Screen>) -> Self {
        Chess { 
            screen
        }
    }

    pub fn init(&mut self) {
    }

    pub fn play(&mut self) -> GameState {
        let hello_world = Text::new("\nHello Chess".to_string());
        self.screen.clear();
        self.screen.show(&hello_world);

        let input = self.screen.input().trim().to_lowercase(); // Normalize and trim input
        

        if input.is_empty() {
            // Player didn't enter anything
            println!("Please enter a valid character or a special command.");
        } else if input == "stop" {
            return GameState::Stop;
        } else if input == "pause" {
            return GameState::Pause;
        } else if input == "replay" {
            return GameState::Init;
        } else if input.len() > 1 {
            // Player entered more than one character
            println!("Please enter only one character or a special command.");
        }

        GameState::Play
    }
}