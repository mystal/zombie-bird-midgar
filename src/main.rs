extern crate bmfont;
extern crate cgmath;
extern crate ears;
extern crate midgar;
extern crate nalgebra;
extern crate ncollide;
extern crate rand;

use config::*;

mod app;
mod bird;
mod config;
mod renderer;
mod scroll_handler;
mod sounds;
mod world;


fn main() {
    let config = midgar::MidgarAppConfig::new()
        .with_title("Zombie Bird")
        .with_screen_size((SCREEN_SIZE.0 * DEFAULT_SCALE, SCREEN_SIZE.1 * DEFAULT_SCALE))
        .with_vsync(false)
        .with_fps(240);
    let app: midgar::MidgarApp<app::GameApp> = midgar::MidgarApp::new(config);
    app.run();
}
