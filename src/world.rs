use midgar::{Midgar, VirtualKeyCode};
use nalgebra::{self, Isometry2};
use ncollide::query;
use ncollide::shape::Cuboid;

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

    mid_point_y: u32,

    bird: Bird,
    scroller: ScrollHandler,
    ground: Cuboid<nalgebra::Vector2<f32>>,
}

impl GameWorld {
    pub fn new(game_width: f32, game_height: f32) -> Self {
        let mid_point_y = (game_height / 2.0) as u32;

        GameWorld {
            game_state: GameState::Ready,
            score: 0,
            run_time: 0.0,

            mid_point_y: mid_point_y,

            bird: Bird::new(33.0, mid_point_y as f32 + 5.0, 17, 12, game_height),
            scroller: ScrollHandler::new(mid_point_y as f32 - 66.0, game_height),
            ground: Cuboid::new(nalgebra::Vector2::new(136.0 / 2.0, 11.0 / 2.0)),
        }
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        self.run_time += dt;

        match self.game_state {
            GameState::Menu | GameState::Ready => self.update_ready(midgar, dt),
            GameState::Running => self.update_running(midgar, dt),
            GameState::GameOver => self.update_game_over(midgar, dt),
            _ => {},
        }
    }

    fn update_ready(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(&VirtualKeyCode::Space) {
            self.game_state = GameState::Running;
        }

        self.bird.update_ready(midgar, self.run_time);
        self.scroller.update_ready(dt);
    }

    fn update_running(&mut self, midgar: &Midgar, dt: f32) {
        self.bird.update_running(midgar, dt);
        self.scroller.update_running(dt);

        if self.scroller.scored(&self.bird) {
            self.score += 1;
            // println!("Scored! {}", self.score);
        }

        if self.scroller.collides(&self.bird) && self.bird.is_alive() {
            // Clean up on game over
            self.scroller.stop();
            self.bird.die();
            // AssetLoader.dead.play();
        }

        let bird_overlaps_ground = {
            let (bounding_circle, bird_center) = self.bird.bounding_circle();
            let ref bird_center = Isometry2::new(bird_center, nalgebra::zero());
            let ground_center = nalgebra::Vector2::new(136.0 / 2.0, self.mid_point_y as f32 - 71.5);
            let ref ground_center = Isometry2::new(ground_center, nalgebra::zero());
            let distance = query::distance(bird_center, bounding_circle,
                                           ground_center, &self.ground);
            distance == 0.0
        };

        if bird_overlaps_ground {
            self.scroller.stop();
            self.bird.die();
            self.bird.decelerate();
            self.game_state = GameState::GameOver;

            // if (score > AssetLoader.getHighScore()) {
            //     AssetLoader.setHighScore(score);
            //     currentState = GameState.HIGHSCORE;
            // }
        }
    }

    fn update_game_over(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(&VirtualKeyCode::Space) {
            self.restart();
        }
    }

    fn restart(&mut self) {
        self.score = 0;
        self.bird.on_restart(self.mid_point_y as f32 + 5.0);
        self.scroller.on_restart();
        self.game_state = GameState::Ready;
    }

    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    pub fn run_time(&self) -> f32 {
        self.run_time
    }

    pub fn mid_point_y(&self) -> u32 {
        self.mid_point_y
    }

    pub fn bird(&self) -> &Bird {
        &self.bird
    }

    pub fn scroller(&self) -> &ScrollHandler {
        &self.scroller
    }
}
