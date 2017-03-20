use midgar::{App, Midgar, KeyCode};

use renderer::GameRenderer;
use world::GameWorld;


pub struct GameApp {
    world: GameWorld,
    renderer: GameRenderer,

    time_to_fps: f64,
}

impl App for GameApp {
    fn create(midgar: &Midgar) -> Self {
        // TODO: Query screen size and store info for renderer/world to use.
        // TODO: Keep separate world/screen (pixel) coordinates.

        let (screen_width, screen_height) = midgar.graphics().screen_size();
        let game_width = 136.0f32;
        let game_height = screen_height as f32 / (screen_width as f32 / game_width);
        println!("Screen: {:?}, Game: {:?}", (screen_width, screen_height), (game_width, game_height));

        GameApp {
            world: GameWorld::new(game_width, game_height),
            renderer: GameRenderer::new(midgar),

            time_to_fps: 1.0,
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(&KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        let dt = midgar.time().delta_time();

        // TODO: Process input?

        // Update game world.
        self.world.update(midgar, dt as f32);

        // Render game world.
        self.renderer.render(midgar, dt as f32, &self.world);

        // Print FPS every second.
        self.time_to_fps -= dt;
        if self.time_to_fps <= 0.0 {
            println!("FPS: {:.2}, Frame time: {:.2} ms", midgar.fps(), midgar.frame_time() * 1000.0);
            self.time_to_fps = 1.0;
        }
    }

    fn resize(&mut self, size: (u32, u32), midgar: &Midgar) {
        //self.renderer.resize(size);
    }
}
