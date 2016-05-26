use cgmath;
use midgar::{Midgar, MagnifySamplerFilter, Surface};
use midgar::sprite::{Sprite, SpriteRenderer};

use world::{GameState, GameWorld};


const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


pub struct GameRenderer {
    sprite_renderer: SpriteRenderer,
    //shape_renderer: glutils::ShapeRenderer,

    projection: cgmath::Matrix4<f32>,

    bird_sprite: Sprite,
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        let texture = midgar.graphics().load_texture("assets/texture.png");
        // TODO: Load sprites.
        let mut bird_sprite = Sprite::with_sub_field(texture, (153, 116), (17, 12));
        bird_sprite.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bird_sprite.set_alpha(true);

        let (screen_width, screen_height) = midgar.graphics().screen_size();
        let game_width = 136.0f32;
        let game_height = screen_height as f32 / (screen_width as f32 / game_width);

        GameRenderer {
            sprite_renderer: SpriteRenderer::new(&midgar.graphics().display),
            //shape_renderer: glutils::ShapeRenderer::new(&midgar.graphics().display),
            projection: cgmath::ortho(0.0, game_width, 0.0, game_height, -1.0, 1.0),

            bird_sprite: bird_sprite,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld) {
        // Get framebuffer target.
        let mut target = midgar.graphics().display.draw();
        target.clear_color(CLEAR_COLOR[0], CLEAR_COLOR[1], CLEAR_COLOR[2], CLEAR_COLOR[3]);

        // TODO: Draw background shapes in background.

        // TODO: Draw grass and pipes.

        // TODO: Draw world.
        match world.game_state() {
            GameState::Running => {
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
        self.sprite_renderer.draw_sprite(&self.bird_sprite, &self.projection, target);
    }

    fn draw_grass(&mut self, world: &GameWorld) {
        // batcher.draw(grass, frontGrass.getX(), frontGrass.getY(),
        //         frontGrass.getWidth(), frontGrass.getHeight());
        // batcher.draw(grass, backGrass.getX(), backGrass.getY(),
        //         backGrass.getWidth(), backGrass.getHeight());
    }

    fn draw_pipes(&mut self, world: &GameWorld) {
    }
}
