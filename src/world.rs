use bird::Bird;


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
    //ground: Ground,
}

impl GameWorld {
    pub fn new(game_width: f32, game_height: f32) -> Self {
        GameWorld {
            game_state: GameState::Ready,
            score: 0,
            run_time: 0.0,

            bird: Bird::new(33.0, (game_height / 2.0) - 5.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.run_time += dt;

        match self.game_state {
            GameState::Menu | GameState::Ready => self.update_ready(dt),
            GameState::Running => self.update_running(dt),
            _ => {},
        }
    }

    fn update_ready(&mut self, dt: f32) {
        self.bird.update_ready(self.run_time);
    }

    fn update_running(&mut self, dt: f32) {
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
}
