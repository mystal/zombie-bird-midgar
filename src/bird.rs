use cgmath::{self, Vector2};

//use units::WorldPosition;


pub struct Bird {
    position: Vector2<f32>,
    // velocity: Vector2,
    // acceleration: Vector2,

    // rotation: f32,
    // width: u32,
    // height: u32,

    original_y: f32,

    // is_alive: bool,

    // bounding_circle: Circle,
}

impl Bird {
    pub fn new(x: f32, y: f32) -> Self {
        Bird {
            position: cgmath::vec2(x, y),
            original_y: y,
        }
    }

    pub fn update_ready(&mut self, run_time: f32) {
        self.position.y = 2.0 * (7.0 * run_time).sin() + self.original_y;
    }

    pub fn position(&self) -> cgmath::Vector2<f32> {
        self.position
    }
}
