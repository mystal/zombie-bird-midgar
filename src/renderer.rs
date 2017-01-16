use std::fs::File;
use std::rc::Rc;

use bmfont::{BMFont, OrdinateOrientation};
use cgmath;
use midgar::{Midgar, MagnifySamplerFilter, Surface, Texture2d};
use midgar::graphics::animation::{Animation, PlayMode};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::sprite::{Sprite, SpriteRenderer};
use midgar::graphics::texture_region::TextureRegion;
use midgar::graphics::texture_region::TextureRegionHolder;

use scroll_handler::Pipe;
use world::{GameState, GameWorld};


const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


pub struct GameRenderer {
    sprite_renderer: SpriteRenderer,
    shape_renderer: ShapeRenderer,

    texture: Rc<Texture2d>,
    // logoTexture: Rc<Texture2d>,
    text_texture: Rc<Texture2d>,
    shadow_texture: Rc<Texture2d>,

    text_font: BMFont,
    shadow_font: BMFont,

    bird: TextureRegion,
    bird_up: TextureRegion,
    bird_down: TextureRegion,
    bird_animation: Animation,

    // TODO: Make these TextureRegions since we don't need to store state with them.
    bg: Sprite,
    grass: Sprite,
    skull_up: Sprite,
    skull_down: Sprite,
    bar: TextureRegion,

    // TextureRegion logo, zbLogo;
    // TextureRegion playButtonUp, playButtonDown;
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        let (screen_width, screen_height) = midgar.graphics().screen_size();
        let game_width = 136.0f32;
        let game_height = screen_height as f32 / (screen_width as f32 / game_width);
        let mid_point_y = (game_height / 2.0) as u32;

        let texture = midgar.graphics().load_texture("assets/texture.png", true);
        let texture = Rc::new(texture);

        let text_texture = midgar.graphics().load_texture("assets/text.png", false);
        let text_texture = Rc::new(text_texture);
        let shadow_texture = midgar.graphics().load_texture("assets/shadow.png", false);
        let shadow_texture = Rc::new(shadow_texture);

        let text_font = {
            let file = File::open("assets/text.fnt").unwrap();
            BMFont::new(file, OrdinateOrientation::TopToBottom).unwrap()
        };
        let shadow_font = {
            let file = File::open("assets/shadow.fnt").unwrap();
            BMFont::new(file, OrdinateOrientation::TopToBottom).unwrap()
        };

        // Load bird.
        let mut bird = TextureRegion::with_sub_field(texture.clone(), (153, 116), (17, 12));
        bird.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bird.set_alpha(true);
        let mut bird_up = TextureRegion::with_sub_field(texture.clone(), (170, 116), (17, 12));
        bird_up.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bird_up.set_alpha(true);
        let mut bird_down = TextureRegion::with_sub_field(texture.clone(), (136, 116), (17, 12));
        bird_down.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bird_down.set_alpha(true);

        let mut bird_animation = Animation::new(0.06, &[bird_down.clone(), bird.clone(), bird_up.clone()]).unwrap();
        bird_animation.play_mode = PlayMode::LoopPingPong;

        // Load background.
        let mut bg = Sprite::with_sub_field(texture.clone(), (0, 85), (136, 43));
        bg.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        bg.set_position(cgmath::vec2(0.0, mid_point_y as f32 - 66.0));

        // Load grass.
        let mut grass = Sprite::with_sub_field(texture.clone(), (0, 74), (143, 11));
        grass.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));

        // Load pipe.
        let mut skull_up = Sprite::with_sub_field(texture.clone(), (192, 114), (24, 14));
        skull_up.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        let mut skull_down = Sprite::with_sub_field(texture.clone(), (192, 114), (24, 14));
        skull_down.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        skull_down.set_flip_y(true);
        let mut bar = TextureRegion::with_sub_field(texture.clone(), (136, 109), (22, 3));
        bar.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
        // TODO: Load other sprites.

        let projection = cgmath::ortho(0.0, game_width, 0.0, game_height, -1.0, 1.0);

        GameRenderer {
            sprite_renderer: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape_renderer: ShapeRenderer::new(midgar.graphics().display(), projection),

            texture: texture,
            text_texture: text_texture,
            shadow_texture: shadow_texture,

            text_font: text_font,
            shadow_font: shadow_font,

            bird: bird,
            bird_up: bird_up,
            bird_down: bird_down,
            bird_animation: bird_animation,

            bg: bg,
            grass: grass,
            skull_up: skull_up,
            skull_down: skull_down,
            bar: bar,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld) {
        // Get framebuffer target.
        let mut target = midgar.graphics().display().draw();
        target.clear_color(CLEAR_COLOR[0], CLEAR_COLOR[1], CLEAR_COLOR[2], CLEAR_COLOR[3]);

        // Draw Background color
        let color = [55.0 / 255.0, 80.0 / 255.0, 100.0 / 255.0];
        self.shape_renderer.draw_filled_rect(0.0, world.mid_point_y() as f32 - 23.0, 136.0, world.mid_point_y() as f32 + 23.0,
                                             color, &mut target);

        // Draw Dirt
        let color = [147.0 / 255.0, 80.0 / 255.0, 27.0 / 255.0];
        self.shape_renderer.draw_filled_rect(0.0, 0.0, 136.0, 52.0, color, &mut target);

        // Draw world background.
        self.sprite_renderer.draw_sprite(&self.bg, &mut target);

        // Draw grass and pipes.
        self.draw_grass(world, &mut target);
        self.draw_pipes(world, &mut target);
        self.draw_skulls(world, &mut target);

        // Draw world.
        match world.game_state() {
            GameState::Running => {
                self.draw_bird(world, &mut target);
                self.draw_score(world, &mut target);
            },
            GameState::Ready => {
                self.draw_bird(world, &mut target);
                self.draw_score(world, &mut target);
            },
            GameState::Menu => {
            },
            GameState::GameOver => {
                self.draw_bird(world, &mut target);
                self.draw_score(world, &mut target);
            },
            GameState::HighScore => {
                self.draw_bird(world, &mut target);
                self.draw_score(world, &mut target);
            },
        }

        target.finish().unwrap();
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        let game_width = 136.0f32;
        let game_height = size.1 as f32 / (size.0 as f32 / game_width);
        let projection = cgmath::ortho(0.0, game_width, 0.0, game_height, -1.0, 1.0);

        self.sprite_renderer.set_projection_matrix(projection);
        self.shape_renderer.set_projection_matrix(projection);
    }

    fn draw_bird<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        let position = world.bird().position();
        let rotation = world.bird().rotation();
        let texture = if world.bird().should_flap() {
            self.bird_animation.current_key_frame(world.run_time())
        } else {
            &self.bird
        };

        self.sprite_renderer.draw_region_with_rotation(texture, position.x, position.y, rotation,
                                                       texture.size().x as f32, texture.size().y as f32,
                                                       target);
    }

    fn draw_grass<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        self.grass.set_position(world.scroller().front_grass().position());
        self.sprite_renderer.draw_sprite(&self.grass, target);
        self.grass.set_position(world.scroller().back_grass().position());
        self.sprite_renderer.draw_sprite(&self.grass, target);
    }

    fn draw_skulls<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        let mut draw_skull = |pipe: &Pipe| {
            let position = pipe.position();
            let height = pipe.height();

            self.skull_up.set_position(position + cgmath::vec2(-1.0, height as f32 - 14.0));
            self.sprite_renderer.draw_sprite(&self.skull_up, target);
            self.skull_down.set_position(position + cgmath::vec2(-1.0, 45.0 + height as f32));
            self.sprite_renderer.draw_sprite(&self.skull_down, target);
        };

        draw_skull(world.scroller().pipe1());
        draw_skull(world.scroller().pipe2());
        draw_skull(world.scroller().pipe3());
    }

    fn draw_pipes<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        let mut draw_pipe = |pipe: &Pipe| {
            let position = pipe.position();
            let width = pipe.width();
            let height = pipe.height();

            self.sprite_renderer.draw_region(&self.bar, position.x, position.y,
                                             width as f32, pipe.lower_bar_height(), target);
            self.sprite_renderer.draw_region(&self.bar, position.x, position.y + height as f32 + 45.0,
                                             width as f32, pipe.upper_bar_height(),
                                             target);
        };

        draw_pipe(world.scroller().pipe1());
        draw_pipe(world.scroller().pipe2());
        draw_pipe(world.scroller().pipe3());
    }

    fn draw_score<S: Surface>(&mut self, world: &GameWorld, target: &mut S) {
        let scale = 0.25;

        let score_text: String = world.score().to_string();
        //let score_text = format!("{:02}", world.score());
        let text_length = score_text.len();

        let start_position = (68.0 - (3.0 * text_length as f32), world.mid_point_y() as f32 + 64.0);
        let shadow_positions = self.shadow_font.parse(&score_text).unwrap();
        for pos in shadow_positions {
            let offset = (pos.page_rect.x as u32, pos.page_rect.y as u32);
            let size = (pos.page_rect.width, pos.page_rect.height);
            let mut region = TextureRegion::with_sub_field(self.shadow_texture.clone(), offset, size);
            region.set_flip_y(true);
            region.set_scale(scale);
            region.set_alpha(true);
            region.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
            let position = (start_position.0 + pos.screen_rect.x as f32 * scale,
                            start_position.1 + pos.screen_rect.y as f32 * scale);

            self.sprite_renderer.draw_region(&region, position.0 as f32, position.1 as f32,
                                             region.size().x as f32, region.size().y as f32,
                                             &self.projection, target);
        }

        let start_position = (68.0 - (3.0 * text_length as f32), world.mid_point_y() as f32 + 65.0);
        let text_positions = self.text_font.parse(&score_text).unwrap();
        for pos in text_positions {
            let offset = (pos.page_rect.x as u32, pos.page_rect.y as u32);
            let size = (pos.page_rect.width, pos.page_rect.height);
            let mut region = TextureRegion::with_sub_field(self.text_texture.clone(), offset, size);
            region.set_flip_y(true);
            region.set_scale(scale);
            region.set_alpha(true);
            region.set_magnify_filter(Some(MagnifySamplerFilter::Nearest));
            let position = (start_position.0 + pos.screen_rect.x as f32 * scale,
                            start_position.1 + pos.screen_rect.y as f32 * scale);

            self.sprite_renderer.draw_region(&region, position.0 as f32, position.1 as f32,
                                             region.size().x as f32, region.size().y as f32,
                                             &self.projection, target);
        }
    }
}
