use std::rc::Rc;

use cgmath;
use midgar::{Midgar, MagnifySamplerFilter, Surface, Texture2d};
use midgar::sprite::{Sprite, SpriteRenderer};

use scroll_handler::Pipe;
use world::{GameState, GameWorld};


const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


pub struct GameRenderer {
    sprite_renderer: SpriteRenderer,
    //shape_renderer: glutils::ShapeRenderer,

    projection: cgmath::Matrix4<f32>,

    texture: Rc<Texture2d>,
    // logoTexture: Rc<Texture2d>,

    // TODO: Make these TextureRegions since we don't need to store state with them.
    bird: Sprite,
    // Animation birdAnimation;
    // TextureRegion birdDown, birdUp;

    bg: Sprite,
    grass: Sprite,
    skull_up: Sprite,
    skull_down: Sprite,
    bar: Sprite,

    // TextureRegion logo, zbLogo;
    // TextureRegion playButtonUp, playButtonDown;
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        let (screen_width, screen_height) = midgar.graphics().screen_size();
        let game_width = 136.0f32;
        let game_height = screen_height as f32 / (screen_width as f32 / game_width);
        let mid_point_y = (game_height / 2.0) - 5.0;

        let texture = midgar.graphics().load_texture("assets/texture.png");
        let texture = Rc::new(texture);

        // Load bird.
        let mut bird = Sprite::with_sub_field(texture.clone(), (153, 116), (17, 12));
        bird.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bird.set_alpha(true);
        // Load background.
        let mut bg = Sprite::with_sub_field(texture.clone(), (0, 85), (136, 43));
        bg.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bg.set_position(cgmath::vec2(0.0, mid_point_y - 63.0));
        // Load grass.
        let mut grass = Sprite::with_sub_field(texture.clone(), (0, 74), (143, 11));
        grass.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        // Load pipe.
        let mut skull_up = Sprite::with_sub_field(texture.clone(), (192, 114), (24, 14));
        skull_up.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        let mut skull_down = Sprite::with_sub_field(texture.clone(), (192, 114), (24, 14));
        skull_down.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        skull_down.set_flip_y(true);
        let mut bar = Sprite::with_sub_field(texture.clone(), (136, 109), (22, 3));
        bar.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        // TODO: Load other sprites.

        GameRenderer {
            sprite_renderer: SpriteRenderer::new(&midgar.graphics().display),
            //shape_renderer: glutils::ShapeRenderer::new(&midgar.graphics().display),
            projection: cgmath::ortho(0.0, game_width, 0.0, game_height, -1.0, 1.0),

            texture: texture,

            bird: bird,
            bg: bg,
            grass: grass,
            skull_up: skull_up,
            skull_down: skull_down,
            bar: bar,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld) {
        // Get framebuffer target.
        let mut target = midgar.graphics().display.draw();
        target.clear_color(CLEAR_COLOR[0], CLEAR_COLOR[1], CLEAR_COLOR[2], CLEAR_COLOR[3]);

        // TODO: Draw background shapes.

        // Draw world background.
        self.sprite_renderer.draw_sprite(&self.bg, &self.projection, &mut target);

        // Draw grass and pipes.
        self.draw_grass(world, &mut target);
        self.draw_pipes(world, &mut target);
        self.draw_skulls(world, &mut target);

        // TODO: Draw world.
        match world.game_state() {
            GameState::Running => {
                self.draw_bird(world, &mut target);
            },
            GameState::Ready => {
                self.draw_bird(world, &mut target);
            },
            GameState::Menu => {
            },
            GameState::GameOver => {
            },
            GameState::HighScore => {
            },
        }

        target.finish().unwrap();
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.projection = cgmath::ortho(0.0, size.0 as f32, 0.0, size.1 as f32, -1.0, 1.0);
    }

    fn draw_bird<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        self.bird.set_position(world.bird().position());
        self.bird.set_rotation(world.bird().rotation());
        self.sprite_renderer.draw_sprite(&self.bird, &self.projection, target);
    }

    fn draw_grass<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        self.grass.set_position(world.scroller().front_grass().position());
        self.sprite_renderer.draw_sprite(&self.grass, &self.projection, target);
        self.grass.set_position(world.scroller().back_grass().position());
        self.sprite_renderer.draw_sprite(&self.grass, &self.projection, target);
    }

    fn draw_skulls<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        let mut draw_skull = |pipe: &Pipe| {
            let position = pipe.position();
            let height = pipe.height();

            self.skull_up.set_position(position + cgmath::vec2(1.0, height as f32));
            self.sprite_renderer.draw_sprite(&self.skull_up, &self.projection, target);
            self.skull_down.set_position(position + cgmath::vec2(1.0, 14.0 + 45.0 + height as f32));
            self.sprite_renderer.draw_sprite(&self.skull_down, &self.projection, target);
        };

        draw_skull(world.scroller().pipe1());
        draw_skull(world.scroller().pipe2());
        draw_skull(world.scroller().pipe3());
    }

    fn draw_pipes<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        // batcher.draw(bar, pipe1.getX(), pipe1.getY(), pipe1.getWidth(),
        //         pipe1.getHeight());
        // batcher.draw(bar, pipe1.getX(), pipe1.getY() + pipe1.getHeight() + 45,
        //         pipe1.getWidth(), midPointY + 66 - (pipe1.getHeight() + 45));

        // batcher.draw(bar, pipe2.getX(), pipe2.getY(), pipe2.getWidth(),
        //         pipe2.getHeight());
        // batcher.draw(bar, pipe2.getX(), pipe2.getY() + pipe2.getHeight() + 45,
        //         pipe2.getWidth(), midPointY + 66 - (pipe2.getHeight() + 45));

        // batcher.draw(bar, pipe3.getX(), pipe3.getY(), pipe3.getWidth(),
        //         pipe3.getHeight());
        // batcher.draw(bar, pipe3.getX(), pipe3.getY() + pipe3.getHeight() + 45,
        //         pipe3.getWidth(), midPointY + 66 - (pipe3.getHeight() + 45));
    }
}
