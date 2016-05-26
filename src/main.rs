extern crate cgmath;
extern crate midgar;

mod app;
mod bird;
mod renderer;
mod world;


const SCREEN_SIZE: (u32, u32) = (272, 408);


fn main() {
    let config = midgar::MidgarAppConfig::new()
        .with_screen_size(SCREEN_SIZE);
    let app: midgar::MidgarApp<app::GameApp> = midgar::MidgarApp::new(config);
    app.run();
}
