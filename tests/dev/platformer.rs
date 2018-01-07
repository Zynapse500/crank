#[allow(unused_macros)]
macro_rules! print_deb {
    ($var:expr) => {println!("{}: {:?}", stringify!($var), $var)};
}

const TILE_SIZE: f32 = 128.0;
const WORLD_SIZE: [usize; 2] = [128, 128];


use super::frame_counter::FrameCounter;

use crank;

use crank::{WindowHandle, UpdateInfo, Renderer};
use crank::{RenderBatch, CenteredView, Texture, TextureFilter};
use crank::{RenderShape, Rectangle};

use crank::Image;

use crank::KeyCode;

use crank::{Collide};
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
    view: CenteredView,

    world: World,
    tile_dictionary: Vec<Tile>,
    player: Player,
}


impl Platformer {
    fn tick(&mut self, dt: f32) {
        let mut direction = Vec2f::new(0.0, 0.0);
        if self.window.key_down(KeyCode::D) { direction.x += 1.0; }
        if self.window.key_down(KeyCode::A) { direction.x -= 1.0; }
        if self.window.key_down(KeyCode::W) { direction.y += 1.0; }
        if self.window.key_down(KeyCode::S) { direction.y -= 1.0; }

        if direction.x != 0.0 || direction.y != 0.0 {
            direction = direction.normalized();
            let delta = 60.0 * dt * direction;
            self.player.apply_force(delta.into());
        }


        let velocity = self.player.get_velocity();
        let bounds = Rectangle{
            center: self.player.get_position(),
            size: [
                velocity[0].abs() + 10.0,
                velocity[1].abs() + 10.0,
            ]
        };

        let world_obstacles = self.world.get_obstacles(bounds);
        let mut obstacles: Vec<Box<&Body<<Player as PhysicsObject>::CollisionBody>>> = Vec::new();
        for rect in world_obstacles.iter() {
            obstacles.push(Box::new(rect));
        }

        self.player.update(dt);
        self.player.tick(dt, obstacles.as_slice());


        // Move camera
        let player_position = self.player.get_position();

        self.view.center[0] += (player_position[0] - self.view.center[0]) * dt * 4.0;
        self.view.center[1] += (player_position[1] - self.view.center[1]) * dt * 4.0;
    }

    fn draw(&mut self) {
        self.batch.clear();
        self.batch.set_view(self.view);

        let batch = &mut self.batch;
        self.world.draw(batch);

        self.player.draw(batch);


        /*let velocity = self.player.get_velocity();
        let bounds = Rectangle{
            center: self.player.get_position(),
            size: [
                velocity[0].abs() + 10.0,
                velocity[1].abs() + 10.0,
            ]
        };

        let obstacles = self.world.get_obstacles(bounds);
        for obstacle in obstacles {
            batch.set_color([0.0, 1.0, 1.0, 1.0]);
            batch.fill_rectangle(&Rectangle {
                size: crank::vec2_sub(obstacle.size, [0.3; 2]),
                .. obstacle
            });
        }*/
    }

    fn create_world(dictionary: &Vec<Tile>) -> World {
        World::random(WORLD_SIZE[0], WORLD_SIZE[1], dictionary)
    }
}


impl crank::Game for Platformer {
    fn setup(window: WindowHandle) -> Self {

        let tile_dictionary = vec![
            Tile::new(TileId::Grass, false, (1, 1), Image::decode(include_bytes!("res/Grass.png")).unwrap()),
            Tile::new(TileId::Sand, false, (1, 1), Image::decode(include_bytes!("res/Sand.png")).unwrap()),

            Tile::new(TileId::Water, true, (4, 4), Image::decode(include_bytes!("res/Water.png")).unwrap()),
        ];


        let platformer = Platformer {
            running: true,
            window,

            frame_counter: FrameCounter::new(),
            time_accumulator: 0.0,

            batch: RenderBatch::new(),

            view: CenteredView::default(),
            world: Platformer::create_world(&tile_dictionary),
            tile_dictionary,
            player: Player::new([WORLD_SIZE[0] as f32 / 2.0, WORLD_SIZE[1] as f32 / 2.0]),
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
            self.window.set_title(&format!("FPS: {}   ---   Tiles: {}", fps, WORLD_SIZE[0] * WORLD_SIZE[1]));
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
                self.player = Player::new([WORLD_SIZE[0] as f32 / 2.0, WORLD_SIZE[1] as f32 / 2.0]);
            }

            KeyCode::M => {
                self.world = Platformer::create_world(&self.tile_dictionary);
            }

            _ => ()
        }
    }

    fn size_changed(&mut self, width: u32, height: u32) {
        self.view.size[0] = width as f32 / TILE_SIZE;
        self.view.size[1] = height as f32 / TILE_SIZE;
    }
}


const CHUNK_SIZE: [usize; 2] = [32; 2];



struct World {
    chunk_indices: HashMap<[i32; 2], usize>,
    chunks: Vec<(Rectangle, Chunk)>,
}


struct Chunk {
    tiles: HashMap<[i32; 2], (Rectangle, Tile)>,
}

impl World {
    pub fn random(width: usize, height: usize, dictionary: &Vec<Tile>) -> World {
        let len = dictionary.len();
        let tile_indices = rand::thread_rng().gen_iter()
            .take(width * height).map(|i: usize| i % len).collect::<Vec<usize>>();

        World::from_indices(width, height, tile_indices, dictionary)
    }


    pub fn from_indices(width: usize, height: usize, tile_indices: Vec<usize>, dictionary: &Vec<Tile>) -> World {
        let mut world = World {
            chunk_indices: HashMap::new(),
            chunks: Vec::new()
        };

        let chunks_x = (width as f64 / CHUNK_SIZE[0] as f64).ceil() as usize;
        let chunks_y = (height as f64 / CHUNK_SIZE[1] as f64).ceil() as usize;

        for x in 0..chunks_x {
            for y in 0..chunks_y {
                let w = if x < chunks_x - 1 {CHUNK_SIZE[0]} else {width - x * CHUNK_SIZE[0]};
                let h = if y < chunks_y - 1 {CHUNK_SIZE[1]} else {height - y * CHUNK_SIZE[1]};

                let chunk = Chunk::new();

                let rect = Rectangle {
                    center: [
                        (x * w) as f32 + w as f32 / 2.0,
                        (y * h) as f32 + h as f32 / 2.0,
                    ],
                    size: [w as f32 + 1.0, h as f32 + 1.0],
                };

                world.chunk_indices.insert([x as i32, y as i32], world.chunks.len());
                world.chunks.push((rect, chunk));
            }
        }


        let get_index = |x: usize, y: usize| -> usize {
            tile_indices[x + width * (height - y - 1)]
        };

        let get_neighbours = |x: usize, y: usize| {
            let mut neighbours = [None; 8];

            let x = x as i32;
            let y = y as i32;

            let mut i = 0;
            for ny in (y - 1..y + 2).rev() {
                for nx in x - 1..x + 2 {
                    if nx == x && ny == y { continue; }

                    if 0 <= nx && nx < width as i32 &&
                        0 <= ny && ny < height as i32 {
                        neighbours[i] = Some(dictionary[get_index(nx as usize, ny as usize)].id());
                    }

                    i += 1;
                }
            }

            neighbours
        };

        for x in 0..width {
            for y in 0..height {
                let index = get_index(x, y);
                let tile: Tile = dictionary[index];

                let tx = x as i32;
                let ty = y as i32;

                match tile {
                    t @ Tile::Open { .. } => { world.set_tile(tx, ty, t); }

                    t @ Tile::Open4x4 { .. } => {
                        let texture = t.texture_from_neighbours(&get_neighbours(x, y));
                        world.set_tile(tx, ty, Tile::Open { id: t.id(), texture });
                    }

                    t @ Tile::Solid { .. } => { world.set_tile(tx, ty, t); }

                    t @ Tile::Solid4x4 { .. } => {
                        let texture = t.texture_from_neighbours(&get_neighbours(x, y));
                        world.set_tile(tx, ty, Tile::Solid { id: t.id(), texture });
                    }
                }
            }
        }

        world
    }


    pub fn draw(&self, batch: &mut RenderBatch) {
        let bounds = batch.get_view_bounds();

        for &(rect, ref chunk) in self.chunks.iter() {
            if bounds.intersects(&rect) {
                chunk.draw(bounds, batch);
            }
        }

        batch.set_texture(None);
    }

    pub fn get_obstacles(&self, bounds: Rectangle) -> Vec<Rectangle> {
        let mut obstacles = Vec::new();

        for &(rect, ref chunk) in self.chunks.iter() {
            if bounds.intersects(&rect) {
                obstacles.extend(chunk.get_obstacles(bounds));
            }
        }

        obstacles
    }


    fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        let chunk_x = (x as f32 / CHUNK_SIZE[0] as f32).floor() as i32;
        let chunk_y = (y as f32 / CHUNK_SIZE[1] as f32).floor() as i32;

        let result = self.chunk_indices.get(&[chunk_x, chunk_y]);
        if let Some(&index) = result {
            let &mut (_, ref mut chunk) = self.chunks.get_mut(index).unwrap();

            chunk.set_tile(x, y, tile);
        }
    }
}


impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            tiles: HashMap::new()
        }
    }


    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        let rect = Rectangle {
            center: [x as f32, y as f32],
            size: [1.0; 2]
        };

        self.tiles.insert([x, y], (rect, tile));
    }


    pub fn draw(&self, bounds: Rectangle, batch: &mut RenderBatch) {
        for (_, tile) in self.tiles.iter() {
            if bounds.intersects(&tile.0) {
                match tile.1 {
                    Tile::Solid { texture, .. } | Tile::Open { texture, .. } => {
                        batch.set_color([1.0, 1.0, 1.0, 1.0]);
                        batch.set_texture(Some(texture));
                        batch.fill_rectangle(&tile.0);
                    }

                    _ => ()
                }
            }
        }
    }


    pub fn get_obstacles(&self, bounds: Rectangle) -> Vec<Rectangle> {
        let mut obstacles = Vec::new();

        for (_, tile) in self.tiles.iter() {
            if tile.1.is_solid() {
                if bounds.intersects(&tile.0) {
                    obstacles.push(tile.0.clone());
                }
            }
        }

        obstacles
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Open { id: TileId, texture: Texture },
    Open4x4 { id: TileId, textures: [Texture; 4 * 4] },

    Solid { id: TileId, texture: Texture },
    Solid4x4 { id: TileId, textures: [Texture; 4 * 4] },
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TileId {
    Grass,
    Sand,
    Water
}


impl Tile {

    /// Create a tile from an id and image
    pub fn new(id: TileId, solid: bool, size: (u32, u32), image: Image) -> Tile {
        match size {
            (1, 1) => {
                // texture.set_filter(TextureFilter::Linear);
                match solid {
                    true => Tile::Solid { id, texture: image.into() },
                    false => Tile::Open { id, texture: image.into() },
                }
            }

            (4, 4) => {
                macro_rules! into_arr {
                    ($n:expr, $v:ident) => {{
                        let mut arr = [Texture::empty(); $n];
                        for i in 0..$n {
                            arr[i] = $v[i];
                        }
                        arr
                    }};
                }

                let mut tiles: Vec<Texture> = image.split_tiles(4, 4).into_iter().map(|image| image.into()).collect();
                for texture in tiles.iter_mut() {
                    texture.set_filter(TextureFilter::Linear);
                }

                let textures = into_arr!(4*4, tiles);

                match solid {
                    true => Tile::Solid4x4 { id, textures },
                    false => Tile::Open4x4 { id, textures },
                }
            }

            _ => unimplemented!()
        }
    }


    /// Returns the id of a tile
    pub fn id(&self) -> TileId {
        match self {
            &Tile::Open { id, .. } => id,
            &Tile::Open4x4 { id, .. } => id,

            &Tile::Solid { id, .. } => id,
            &Tile::Solid4x4 { id, .. } => id,
        }
    }

    /// Returns true if this tile is solid
    pub fn is_solid(&self) -> bool {
        match self {
            &Tile::Solid { .. } | &Tile::Solid4x4 { .. } => true,
            _ => false,
        }
    }

    /// Returns the right texture index based on the neighbouring tiles
        ///
        /// 'neighbours' layout:
        /// [0, 1, 2]
        /// [3,    4]
        /// [5, 6, 7]
    //noinspection RsDanglingElse
    pub fn texture_from_neighbours(&self, neighbours: &[Option<TileId>]) -> Texture {
        let match_axis = |matches: &Vec<usize>| {
            if *matches == [1, 3, 4, 6] { Some((1, 1)) } else

            if *matches == [3, 4, 6] { Some((1, 0)) } else
            if *matches == [1, 3, 6] { Some((2, 1)) } else
            if *matches == [1, 3, 4] { Some((1, 2)) } else
            if *matches == [1, 4, 6] { Some((0, 1)) } else

            if *matches == [4, 6] { Some((0, 0)) } else
            if *matches == [3, 6] { Some((2, 0)) } else
            if *matches == [1, 4] { Some((0, 2)) } else
            if *matches == [1, 3] { Some((2, 2)) } else
            if *matches == [1, 6] { Some((3, 1)) } else
            if *matches == [3, 4] { Some((1, 3)) } else

            if *matches == [6] { Some((3, 0)) } else
            if *matches == [1] { Some((3, 2)) } else
            if *matches == [4] { Some((0, 3)) } else
            if *matches == [3] { Some((2, 3)) } else

            if *matches == [] { Some((3, 3)) } else

            { None }
        };

        match self {
            &Tile::Solid4x4 { id, textures } | &Tile::Open4x4 { id, textures } => {
                let mut correct = Vec::with_capacity(8);
                for i in 0..neighbours.len() {
                    if let Some(other_id) = neighbours[i] {
                        // Ignore diagonals
                        if i == 0 || i == 2 || i == 5 || i == 7 { continue; }
                        if other_id == id {
                            correct.push(i);
                        }
                    }
                }
                if let Some((x, y)) = match_axis(&correct) {
                    return self.texture_from_atlas(x, y);
                } else {
                    return Texture::default();
                }
            }

            _ => unimplemented!()
        }
    }


    fn texture_from_atlas(&self, x: usize, y: usize) -> Texture {
        match self {
            &Tile::Solid4x4 { textures, .. } | &Tile::Open4x4 { textures, .. } => {
                assert!(x < 4 && y < 4);
                return textures[x + y * 4];
            }

            &Tile::Solid { texture, .. } | &Tile::Open { texture, .. } => {
                assert!(x == 0 && y == 0);
                return texture;
            }
        }
    }
}


struct Player {
    rect: Rectangle,

    velocity: [f32; 2],
}

impl Player {
    pub fn new(position: [f32; 2]) -> Player {
        Player {
            rect: Rectangle::new(position, [0.5, 0.5]),
            velocity: [0.0; 2],
        }
    }

    pub fn update(&mut self, dt: f32) {}

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
        [20.0, 20.0]
    }

    fn set_drag(&mut self, drag: [f32; 2]) {
        unimplemented!()
    }

    fn get_collider<'a>(&'a self) -> &'a Self::CollisionBody {
        &self.rect
    }
}
