use std::rc::Rc;

use cgmath;
use midgar::{Midgar, MagnifySamplerFilter, Surface, Texture2d};
use midgar::sprite::{Sprite, SpriteRenderer};

use world::{GameState, GameWorld};


const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


pub struct GameRenderer {
    sprite_renderer: SpriteRenderer,
    //shape_renderer: glutils::ShapeRenderer,

    projection: cgmath::Matrix4<f32>,

    texture: Rc<Texture2d>,
    // logoTexture: Rc<Texture2d>,

    // TODO: Make these TextureRegions since we don't need to store state with them.
    bird_sprite: Sprite,
    // public static TextureRegion logo, zbLogo;
    bg: Sprite,
    grass: Sprite,

    // public static Animation birdAnimation;
    // public static TextureRegion bird, birdDown, birdUp;
    // public static TextureRegion skullUp, skullDown, bar;
    // public static TextureRegion playButtonUp, playButtonDown;
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
        let mut bird_sprite = Sprite::with_sub_field(texture.clone(), (153, 116), (17, 12));
        bird_sprite.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bird_sprite.set_alpha(true);
        // Load background.
        let mut bg = Sprite::with_sub_field(texture.clone(), (0, 85), (136, 43));
        bg.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bg.set_position(cgmath::vec2(0.0, mid_point_y - 63.0));
        // Load grass.
        let mut grass = Sprite::with_sub_field(texture.clone(), (0, 74), (143, 11));
        grass.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        // TODO: Load other sprites.

        GameRenderer {
            sprite_renderer: SpriteRenderer::new(&midgar.graphics().display),
            //shape_renderer: glutils::ShapeRenderer::new(&midgar.graphics().display),
            projection: cgmath::ortho(0.0, game_width, 0.0, game_height, -1.0, 1.0),

            texture: texture,

            bird_sprite: bird_sprite,
            bg: bg,
            grass: grass,
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
        self.bird_sprite.set_position(world.bird().position());
        self.bird_sprite.set_rotation(world.bird().rotation());
        self.sprite_renderer.draw_sprite(&self.bird_sprite, &self.projection, target);
    }

    fn draw_grass<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        self.grass.set_position(world.scroller().front_grass().position());
        self.sprite_renderer.draw_sprite(&self.grass, &self.projection, target);
        self.grass.set_position(world.scroller().back_grass().position());
        self.sprite_renderer.draw_sprite(&self.grass, &self.projection, target);
    }

    fn draw_pipes<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
    }
}
