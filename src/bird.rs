use cgmath::{self, Vector2};
use midgar::{Midgar, VirtualKeyCode};

//use units::WorldPosition;


pub struct Bird {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,

    rotation: f32,
    // width: u32,
    // height: u32,

    original_y: f32,

    is_alive: bool,

    // bounding_circle: Circle,
}

impl Bird {
    pub fn new(x: f32, y: f32) -> Self {
        Bird {
            position: cgmath::vec2(x, y),
            velocity: cgmath::vec2(0.0, 0.0),
            acceleration: cgmath::vec2(0.0, -460.0),

            rotation: 0.0,

            original_y: y,

            is_alive: true,
        }
    }

    pub fn update_ready(&mut self, run_time: f32) {
        self.position.y = 2.0 * (7.0 * run_time).sin() + self.original_y;
    }

    pub fn update_running(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(&VirtualKeyCode::Space) {
            self.on_click();
        }
 
        self.velocity += self.acceleration * dt;

        if self.velocity.y < -200.0 {
            self.velocity.y = -200.0;
        }

        // CEILING CHECK
        // if self.position.y < -13.0 {
        //     self.position.y = -13.0;
        //     self.velocity.y = 0.0;
        // }

        self.position += self.velocity * dt;

        // Set the circle's center to be (9, 6) with respect to the bird.
        // Set the circle's radius to be 6.5f;
        // boundingCircle.set(position.x + 9, position.y + 6, 6.5f);

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

    fn on_click(&mut self) {
        if self.is_alive {
            //AssetLoader.flap.play();
            self.velocity.y = 140.0;
        }
    }

    pub fn is_falling(&self) -> bool {
        self.velocity.y < -110.0
    }

    pub fn position(&self) -> cgmath::Vector2<f32> {
        self.position
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }
}
