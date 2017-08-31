use cgmath::{self, Vector2};
use ears::AudioController;
use midgar::{Midgar, KeyCode};
use nalgebra;
use ncollide::shape::Ball;

use sounds::Sounds;
//use units::WorldPosition;


pub const BIRD_RADIUS: f32 = 6.5;
pub const BIRD_GRAVITY: f32 = -460.0;


pub struct Bird {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,

    rotation: f32,
    width: u32,
    height: u32,

    original_y: f32,

    is_alive: bool,

    bounding_circle: Ball<f32>,
    game_height: f32,
}

impl Bird {
    pub fn new(x: f32, y: f32, width: u32, height: u32, game_height: f32) -> Self {
        Bird {
            position: cgmath::vec2(x, y),
            velocity: cgmath::vec2(0.0, 0.0),
            acceleration: cgmath::vec2(0.0, BIRD_GRAVITY),

            rotation: 0.0,
            width: width,
            height: height,

            original_y: y,

            is_alive: true,

            bounding_circle: Ball::new(BIRD_RADIUS),
            game_height: game_height,
        }
    }

    pub fn update_ready(&mut self, midgar: &Midgar, run_time: f32, sounds: &mut Sounds) {
        self.position.y = 2.0 * (7.0 * run_time).sin() + self.original_y;
        if midgar.input().was_key_pressed(KeyCode::Space) {
            self.on_click(sounds);
        }
    }

    pub fn update_running(&mut self, midgar: &Midgar, dt: f32, sounds: &mut Sounds) {
        if midgar.input().was_key_pressed(KeyCode::Space) {
            self.on_click(sounds);
        }

        self.velocity += self.acceleration * dt;

        if self.velocity.y < -200.0 {
            self.velocity.y = -200.0;
        }

        // Ceiling check.
        if self.position.y > self.game_height + BIRD_RADIUS {
            self.position.y = self.game_height + BIRD_RADIUS;
            self.velocity.y = 0.0;
        }

        self.position += self.velocity * dt;

        // Rotate counterclockwise
        if self.velocity.y > 0.0 {
            self.rotation += 600.0 * dt;

            if self.rotation > 20.0 {
                self.rotation = 20.0;
            }
        }

        // Rotate clockwise
        if self.is_falling() || !self.is_alive {
            self.rotation -= 480.0 * dt;
            if self.rotation < -90.0 {
                self.rotation = -90.0;
            }
        }
    }

    fn on_click(&mut self, sounds: &mut Sounds) {
        if self.is_alive {
            self.velocity.y = 140.0;
            sounds.flap.play();
        }
    }

    pub fn die(&mut self) {
        self.is_alive = false;
        self.velocity.y = 0.0;
    }

    pub fn decelerate(&mut self) {
        // We want the bird to stop accelerating downwards once it is dead.
        self.acceleration.y = 0.0;
    }

    pub fn on_restart(&mut self, y: f32) {
        self.rotation = 0.0;
        self.position.y = y;
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.acceleration.x = 0.0;
        self.acceleration.y = BIRD_GRAVITY;
        self.is_alive = true;
    }

    pub fn is_falling(&self) -> bool {
        self.velocity.y < -110.0
    }

    pub fn should_flap(&self) -> bool {
        self.is_alive && self.velocity.y > -70.0
    }

    pub fn position(&self) -> cgmath::Vector2<f32> {
        self.position
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn bounding_circle(&self) -> (&Ball<f32>, nalgebra::Vector2<f32>) {
        let bird_center = nalgebra::Vector2::new(self.position.x + 9.0, self.position.y + 6.0);
        (&self.bounding_circle, bird_center)
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}
