use midgar::{Midgar, VirtualKeyCode};

use bird::Bird;
use scroll_handler::ScrollHandler;


#[derive(Clone, Copy)]
pub enum GameState {
    Menu,
    Ready,
    Running,
    GameOver,
    HighScore,
}

pub struct GameWorld {
    game_state: GameState,
    score: u32,
    run_time: f32,

    bird: Bird,
    scroller: ScrollHandler,
    //ground: Ground,
}

impl GameWorld {
    pub fn new(game_width: f32, game_height: f32) -> Self {
        let mid_point_y = (game_height / 2.0) - 5.0;

        GameWorld {
            game_state: GameState::Ready,
            score: 0,
            run_time: 0.0,

            bird: Bird::new(33.0, mid_point_y),
            scroller: ScrollHandler::new(mid_point_y - 66.0),
        }
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        self.run_time += dt;

        match self.game_state {
            GameState::Menu | GameState::Ready => self.update_ready(midgar, dt),
            GameState::Running => self.update_running(midgar, dt),
            _ => {},
        }
    }

    fn update_ready(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(&VirtualKeyCode::Space) {
            self.game_state = GameState::Running;
        }

        self.bird.update_ready(self.run_time);
        self.scroller.update_ready(dt);
    }

    fn update_running(&mut self, midgar: &Midgar, dt: f32) {
        self.bird.update_running(midgar, dt);
        self.scroller.update_running(dt);
    }

    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    pub fn run_time(&self) -> f32 {
        self.run_time
    }

    pub fn bird(&self) -> &Bird {
        &self.bird
    }

    pub fn scroller(&self) -> &ScrollHandler {
        &self.scroller
    }
}
