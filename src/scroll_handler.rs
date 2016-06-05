use cgmath::{self, Vector2};


// ScrollHandler will use the constants below to determine
// how fast we need to scroll and also determine
// the size of the gap between each pair of pipes.
const SCROLL_SPEED: f32 = -59.0;
const PIPE_GAP: f32 = 49.0;


pub struct ScrollHandler {
    front_grass: Grass,
    back_grass: Grass,
    // pipe1: Pipe,
    // pipe2: Pipe,
    // pipe3: Pipe,
}

impl ScrollHandler {
    // Constructor receives a float that tells us where we need to create our
    // Grass and Pipe objects.
    pub fn new(y_pos: f32) -> Self {
        let front_grass = Grass::new(0.0, y_pos, 143, 11, SCROLL_SPEED);
        let back_grass = Grass::new(front_grass.get_tail_x(), y_pos, 143, 11, SCROLL_SPEED);

        // let pipe1: Pipe::new(210, 0, 22, 60, SCROLL_SPEED, y_pos);
        // let pipe2: Pipe::new(pipe1.get_tail_x() + PIPE_GAP, 0, 22, 70, SCROLL_SPEED, y_pos);
        // let pipe3: Pipe::new(pipe2.get_tail_x() + PIPE_GAP, 0, 22, 60, SCROLL_SPEED, y_pos);

        ScrollHandler {
            front_grass: front_grass,
            back_grass: back_grass,

            // pipe1: pipe1,
            // pipe2: pipe2,
            // pipe3: pipe3,
        }
    }

    pub fn update_ready(&mut self, dt: f32) {
        self.update_grass(dt);
    }

    pub fn update_running(&mut self, dt: f32) {
        self.update_grass(dt);
        // self.update_pipes(dt);
    }

    fn update_grass(&mut self, dt: f32) {
        self.front_grass.update(dt);
        self.back_grass.update(dt);

        // Check if either grass has scrolled offscreen and reset accordingly.
        if self.front_grass.is_scrolled_left() {
            self.front_grass.reset(self.back_grass.get_tail_x());
        } else if self.back_grass.is_scrolled_left() {
            self.back_grass.reset(self.front_grass.get_tail_x());
        }
    }

    // fn update_pipes(&mut self, dt: f32) {
    //     self.pipe1.update(dt);
    //     self.pipe2.update(dt);
    //     self.pipe3.update(dt);

    //     // Check if any pipe has scrolled offscreen and reset accordingly.
    //     if self.pipe1.is_scrolled_left() {
    //         self.pipe1.reset(self.pipe3.get_tail_x() + PIPE_GAP);
    //     } else if self.pipe2.is_scrolled_left() {
    //         self.pipe2.reset(self.pipe1.get_tail_x() + PIPE_GAP);
    //     } else if self.pipe3.is_scrolled_left() {
    //         self.pipe3.reset(self.pipe2.get_tail_x() + PIPE_GAP);
    //     }
    // }

    pub fn stop(&mut self) {
        self.front_grass.stop();
        self.back_grass.stop();
        // self.pipe1.stop();
        // self.pipe2.stop();
        // self.pipe3.stop();
    }

    pub fn on_restart(&mut self) {
        self.front_grass.on_restart(0.0, SCROLL_SPEED);
        self.back_grass.on_restart(self.front_grass.get_tail_x(), SCROLL_SPEED);
        // self.pipe1.on_restart(210, SCROLL_SPEED);
        // self.pipe2.on_restart(pipe1.get_tail_x() + PIPE_GAP, SCROLL_SPEED);
        // self.pipe3.on_restart(pipe2.get_tail_x() + PIPE_GAP, SCROLL_SPEED);
    }

    pub fn front_grass(&self) -> &Grass {
        &self.front_grass
    }

    pub fn back_grass(&self) -> &Grass {
        &self.back_grass
    }

    // pub fn pipe1(&self) -> &Pipe {
    //     &self.pipe1
    // }

    // pub fn pipe2(&self) -> &Pipe {
    //     &self.pipe2
    // }

    // pub fn pipe3(&self) -> &Pipe {
    //     &self.pipe3
    // }
}


pub struct Grass {
    scrollable: Scrollable,
}

impl Grass {
    fn new(x: f32, y: f32, width: u32, height: u32, scroll_speed: f32) -> Self {
        Grass {
            scrollable: Scrollable::new(x, y, width, height, scroll_speed),
        }
    }

    fn update(&mut self, dt: f32) {
        self.scrollable.update(dt);
    }

    fn on_restart(&mut self, new_x: f32, scroll_speed: f32) {
        self.scrollable.on_restart(new_x, scroll_speed);
    }

    fn reset(&mut self, new_x: f32) {
        self.scrollable.reset(new_x);
    }

    fn stop(&mut self) {
        self.scrollable.stop();
    }

    pub fn is_scrolled_left(&self) -> bool {
        self.scrollable.is_scrolled_left()
    }

    pub fn get_tail_x(&self) -> f32 {
        self.scrollable.get_tail_x()
    }

    pub fn position(&self) -> Vector2<f32> {
        self.scrollable.position()
    }

    pub fn width(&self) -> u32 {
        self.scrollable.width()
    }

    pub fn height(&self) -> u32 {
        self.scrollable.height()
    }
}

// const VERTICAL_GAP: u32 = 45;
// const SKULL_WIDTH: u32 = 24;
// const SKULL_HEIGHT: u32 = 11;

// pub struct Pipe {
//     scrollable: Scrollable,
//     //Rectangle skullUp, skullDown, barUp, barDown;
//     ground_y: f32;
//     is_scored: bool;
// }

struct Scrollable {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    width: u32,
    height: u32,
    is_scrolled_left: bool,
}

impl Scrollable {
    fn new(x: f32, y: f32, width: u32, height: u32, scroll_speed: f32) -> Self {
        Scrollable {
            position: cgmath::vec2(x, y),
            velocity: cgmath::vec2(scroll_speed, 0.0),
            width: width,
            height: height,
            is_scrolled_left: false,
        }
    }

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;

        // If the Scrollable object is no longer visible:
        if self.get_tail_x() < 0.0 {
            self.is_scrolled_left = true;
        }
    }

    fn on_restart(&mut self, new_x: f32, scroll_speed: f32) {
        self.velocity.x = scroll_speed;
        self.reset(new_x);
    }

    // Reset: Should Override in subclass for more specific behavior.
    fn reset(&mut self, new_x: f32) {
        self.position.x = new_x;
        self.is_scrolled_left = false;
    }

    fn stop(&mut self) {
        self.velocity.x = 0.0;
    }

    fn is_scrolled_left(&self) -> bool {
        self.is_scrolled_left
    }

    fn get_tail_x(&self) -> f32 {
        self.position.x + self.width as f32
    }

    fn position(&self) -> Vector2<f32> {
        self.position
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}
