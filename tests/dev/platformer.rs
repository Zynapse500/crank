#[allow(unused_macros)]
macro_rules! print_deb {
    ($var:expr) => {println!("{}: {:?}", stringify!($var), $var)};
}

const TILE_SIZE: f32 = 64.0;


use super::frame_counter::FrameCounter;

use crank;

use crank::{WindowHandle, UpdateInfo, Renderer};
use crank::{RenderBatch, CenteredView, Texture, TextureFilter};
use crank::{RenderShape, Rectangle};

use crank::Image;

use crank::KeyCode;

use crank::{Collide, Overlap, Impact};
use crank::{PhysicsObject, Body};

use crank::Vec2f;

use rand;
use rand::Rng;

use std::collections::HashMap;

pub fn main() {
    let settings = crank::GameSettings {
        vertical_sync: false,
        clear_color: [0.2; 4],
    };

    crank::run_game::<Platformer>(800, 600, "Platformer", settings).unwrap();
}


pub struct Platformer {
    running: bool,
    window: WindowHandle,
    frame_counter: FrameCounter,

    time_accumulator: f32,

    batch: RenderBatch,

    world: World,
    tile_dictionary: Vec<Tile>,
    player: Player,
}


impl Platformer {
    fn tick(&mut self, dt: f32) {
        let mut direction = Vec2f::new(0.0, 0.0);
        if self.window.key_down(KeyCode::D) { direction.x += 1.0; }
        if self.window.key_down(KeyCode::A) { direction.x -= 1.0; }

        if direction.x != 0.0 || direction.y != 0.0 {
            direction = direction.normalized();
            let delta = 60.0 * dt * direction;
            self.player.apply_force(delta.into());
        }

        // Gravity
        self.player.apply_force([0.0, -30.0 * dt]);

        if self.window.key_down(KeyCode::Space) { self.player.jump(); }

        let mut obstacles: Vec<Box<Body<<Player as PhysicsObject>::CollisionBody>>> = Vec::new();
        for r in self.world.get_obstacles().into_iter() {
            obstacles.push(Box::new(r));
        }

        self.player.update(dt);
        self.player.tick(dt, obstacles.as_slice());
    }

    fn draw(&mut self) {
        self.batch.clear();
        let view = self.calculate_view();
        self.batch.set_view(view);

        let batch = &mut self.batch;
        self.world.draw(batch);

        self.player.draw(batch);
    }


    fn calculate_view(&self) -> CenteredView {
        let w = self.window.get_width() as f32 / TILE_SIZE;
        let h = self.window.get_height() as f32 / TILE_SIZE;

        CenteredView {
            center: self.player.get_position(),
            size: [w, h],
        }
    }

    fn create_world(dictionary: &Vec<Tile>) -> World {
        World::random(25, 6, dictionary)
    }
}


impl crank::Game for Platformer {
    fn setup(window: WindowHandle) -> Self {
        let present_textures_vec: Vec<Texture> = Image::decode(include_bytes!("res/present.png")).unwrap()
            .split_tiles(4, 4).into_iter().map(|image| image.into()).collect();

        let mut present_textures = [Texture::empty(); 16];

        for i in 0..16 {
            present_textures[i] = present_textures_vec[i];
            present_textures[i].set_filter(TextureFilter::Linear);
        }

        let tile_dictionary = vec![
            Tile::Open { id: TileId::Air },
            Tile::Solid4x4 { id: TileId::Present, textures: present_textures }
        ];


        let platformer = Platformer {
            running: true,
            window,

            frame_counter: FrameCounter::new(),
            time_accumulator: 0.0,

            batch: RenderBatch::new(),

            world: Platformer::create_world(&tile_dictionary),
            tile_dictionary,
            player: Player::new([12.5, 8.0]),
        };

        platformer
    }

    fn update(&mut self, info: UpdateInfo) {
        const UPDATE_RATE: f32 = 1.0 / 240.0;

        self.time_accumulator += info.dt;
        while self.time_accumulator > UPDATE_RATE {
            self.tick(UPDATE_RATE);
            self.time_accumulator -= UPDATE_RATE;
        }

        if let Some(fps) = self.frame_counter.tick() {
            self.window.set_title(&format!("FPS: {}", fps));
        }

        self.draw();
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.submit_batch(&self.batch);
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

impl crank::WindowEventHandler for Platformer {
    fn key_pressed(&mut self, key: KeyCode) {
        match key {
            KeyCode::Escape => self.running = false,

            KeyCode::R => {
                self.player = Player::new([12.5, 8.0]);
            }

            KeyCode::M => {
                self.world = Platformer::create_world(&self.tile_dictionary);
            }

            _ => ()
        }
    }
}


struct World {
    tiles: HashMap<[u32; 2], (Rectangle, Tile)>
}

impl World {
    pub fn random(width: usize, height: usize, dictionary: &Vec<Tile>) -> World {
        let mut world = World {
            tiles: HashMap::new()
        };

        let len = dictionary.len();
        let tile_indices = rand::thread_rng().gen_iter()
            .take(width * height).map(|i: usize| i % len).collect::<Vec<usize>>();

        for x in 0..width {
            for y in 0..height {
                let index = tile_indices[x + width * y];
                let mut tile: Tile = dictionary[index];

                let pos = [x as u32, y as u32];

                let rect = Rectangle {
                    center: [x as f32, y as f32],
                    size: [1.0; 2],
                };

                match tile {
                    t @ Tile::Open { .. } => (),

                    t @ Tile::Solid { .. } => { world.tiles.insert(pos, (rect, t)); }

                    t @ Tile::Solid4x4 { .. } => {
                        let mut neighbours = [TileId::Air; 8];

                        let x = x as i32;
                        let y = y as i32;

                        let mut i = 0;
                        for ny in (y - 1..y + 2).rev() {
                            for nx in x - 1..x + 2 {
                                if nx == x && ny == y { continue; }

                                if 0 <= nx && nx < width as i32 &&
                                    0 <= ny && ny < height as i32 {
                                    neighbours[i] = dictionary[tile_indices[nx as usize + width * ny as usize]].id();
                                }

                                i += 1;
                            }
                        }

                        let texture = t.texture_from_neighbours(&neighbours);
                        world.tiles.insert(pos, (rect, Tile::Solid { id: t.id(), texture }));
                    }
                }
            }
        }

        world
    }

    pub fn draw(&self, batch: &mut RenderBatch) {
        for (&position, tile) in self.tiles.iter() {
            if let &(rect, Tile::Solid { texture, .. }) = tile {
                batch.set_color([1.0, 1.0, 1.0, 1.0]);
                batch.set_texture(Some(texture));
                batch.fill_rectangle(&rect);
            }
        }

        batch.set_texture(None);
    }

    pub fn get_obstacles<'a>(&self) -> Vec<Rectangle> {
        let mut rectangles = Vec::new();

        for (&position, tile) in self.tiles.iter() {
            rectangles.push(tile.0.clone());
        }

        rectangles
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open { id: TileId },
    Solid { id: TileId, texture: Texture },
    Solid4x4 { id: TileId, textures: [Texture; 4 * 4] },
}


#[derive(Copy, Clone, Eq, PartialEq)]
enum TileId {
    Air,
    Present,
}


impl Tile {
    /// Returns the id of a tile
    pub fn id(&self) -> TileId {
        match self {
            &Tile::Open { id, .. } => id,
            &Tile::Solid { id, .. } => id,
            &Tile::Solid4x4 { id, .. } => id,
        }
    }

    /// Returns the right texture index based on the neighbouring tiles
    ///
    /// 'neighbours' layout:
    /// [0, 1, 2]
    /// [3,    4]
    /// [5, 6, 7]
    pub fn texture_from_neighbours(&self, neighbours: &[TileId]) -> Texture {
        match self {
            &Tile::Solid4x4 { id, textures } => {
                let mut correct = Vec::new();
                for i in 0..neighbours.len() {
                    // Ignore diagonals
                    if i == 0 || i == 2 || i == 5 || i == 7 { continue; }
                    if neighbours[i] == id {
                        correct.push(i);
                    }
                }

                if correct == [1, 3, 4, 6] { return textures[5]; }
                if correct == [3, 4, 6] { return textures[1]; }
                if correct == [1, 3, 6] { return textures[6]; }
                if correct == [1, 3, 4] { return textures[9]; }
                if correct == [1, 4, 6] { return textures[4]; }
                if correct == [4, 6] { return textures[0]; }
                if correct == [3, 6] { return textures[2]; }
                if correct == [1, 4] { return textures[8]; }
                if correct == [1, 3] { return textures[10]; }
                if correct == [1, 6] { return textures[7]; }
                if correct == [3, 4] { return textures[13]; }
                if correct == [6] { return textures[3]; }
                if correct == [1] { return textures[11]; }
                if correct == [4] { return textures[12]; }
                if correct == [3] { return textures[14]; }
                return textures[15];
            }

            _ => unimplemented!()
        }
    }
}


struct Player {
    rect: Rectangle,

    velocity: [f32; 2],
    grounded: bool,
}

impl Player {
    pub fn new(position: [f32; 2]) -> Player {
        Player {
            rect: Rectangle::new(position, [0.5, 0.9]),
            velocity: [0.0; 2],
            grounded: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.grounded = false;
    }

    pub fn jump(&mut self) {
        if self.grounded {
            self.grounded = false;
            self.apply_force([0.0, 16.0]);
        }
    }

    pub fn get_position(&self) -> [f32; 2] {
        self.rect.center
    }

    pub fn get_velocity(&self) -> [f32; 2] {
        self.velocity
    }


    pub fn translate(&mut self, amount: [f32; 2]) {
        self.rect.center[0] += amount[0];
        self.rect.center[1] += amount[1];
    }

    pub fn draw(&self, batch: &mut RenderBatch) {
        batch.set_color([0.1, 0.1, 0.8, 1.0]);
        batch.fill_rectangle(&self.rect);
    }
}


impl PhysicsObject for Player {
    type CollisionBody = Rectangle;

    fn get_position(&self) -> [f32; 2] {
        self.rect.center
    }

    fn set_position(&mut self, position: [f32; 2]) {
        self.rect.center = position;
    }

    fn get_velocity(&self) -> [f32; 2] {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: [f32; 2]) {
        self.velocity = velocity;
    }

    fn get_drag(&self) -> [f32; 2] {
        [20.0, 1.0]
    }

    fn set_drag(&mut self, drag: [f32; 2]) {
        unimplemented!()
    }

    fn get_collider<'a>(&'a self) -> &'a Self::CollisionBody {
        &self.rect
    }

    fn handle_impact(&mut self, impact: Impact) {
        let dot = impact.normal[1] * self.velocity[0] + impact.normal[0] * self.velocity[1];

        self.velocity[0] = dot * impact.normal[1];
        self.velocity[1] = dot * impact.normal[0];

        if impact.normal[1] == 1.0 {
            self.grounded = true;
        }
    }
}
