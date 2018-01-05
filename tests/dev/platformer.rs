macro_rules! print_deb {
    ($var:expr) => {println!("{}: {:?}", stringify!($var), $var)};
}

const TILE_SIZE: f32 = 32.0;


use super::frame_counter::FrameCounter;

use crank;

use crank::{WindowHandle, UpdateInfo, Renderer};
use crank::{RenderBatch, CenteredView};
use crank::{RenderShape, Rectangle, Line};
use crank::KeyCode;
use crank::{Collide, Overlap, RayCast, Intersection};

use crank::{Vec2f};

use rand;

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

        if self.window.key_down(KeyCode::Space) { self.player.jump(); }

        self.player.tick(dt, &self.world.get_obstacles());

        /*
                let mut i = 0;
                while let Some(overlap) = self.player.overlap(&self.world) {
                    self.player.translate(overlap.resolve);
                    let force = vec2_mul(
                        vec2_normalize(vec2_abs(overlap.resolve)),
                        vec2_scale(-1.0, self.player.get_velocity())
                    );
                    self.player.apply_force(force);

                    if force[1] > 0.0 {
                        self.player.set_grounded(true);
                    }

                    i += 1;
                    if i > 100 {
                        break;
                    }
                }*/
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
}


impl crank::Game for Platformer {
    fn setup(window: WindowHandle) -> Self {
        let platformer = Platformer {
            running: true,
            window,

            frame_counter: FrameCounter::new(),
            time_accumulator: 0.0,

            batch: RenderBatch::new(),

            world: World::random(25, 25),
            player: Player::new([12.5, 30.0]),
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
                self.player = Player::new([12.5, 30.0]);
            }

            KeyCode::M => {
                self.world = World::random(25, 25);
            }

            _ => ()
        }
    }
}


struct World {
    tiles: HashMap<[u32; 2], Tile>
}

impl World {
    pub fn random(width: usize, height: usize) -> World {
        let mut world = World {
            tiles: HashMap::new()
        };

        for x in 0..width {
            for y in 0..height {
                let tile = if rand::random::<bool>() { Tile::Air } else {
                    Tile::Solid(Rectangle::new([x as f32, y as f32], [1.0; 2]))
                };
                world.tiles.insert([x as u32, y as u32], tile);
            }
        }

        world
    }

    pub fn draw(&self, batch: &mut RenderBatch) {
        for (&position, &tile) in self.tiles.iter() {
            if let Tile::Solid(rect) = tile {
                batch.set_color([0.0, 0.4, 0.0, 1.0]);
                batch.fill_rectangle(&rect);
            }
        }
    }

    pub fn get_obstacles<'a>(&self) -> Vec<Rectangle> {
        let mut rectangles = Vec::new();

        for (&position, &tile) in self.tiles.iter() {
            if let Tile::Solid(rect) = tile {
                rectangles.push(rect.clone());
            }
        }

        rectangles
    }
}


impl Collide<Rectangle> for World {
    fn intersects(&self, other: &Rectangle) -> bool {
        for (&position, &tile) in self.tiles.iter() {
            if let Tile::Solid(rect) = tile {
                if rect.intersects(other) {
                    return true;
                }
            }
        }

        false
    }

    fn overlap(&self, other: &Rectangle) -> Option<Overlap> {
        let mut total_overlap: Option<Overlap> = None;

        for (&position, &tile) in self.tiles.iter() {
            if let Tile::Solid(rect) = tile {
                if let Some(overlap) = rect.overlap(other) {
                    if let Some(ref mut total) = total_overlap {
                        total.depth += overlap.depth;
                        for i in 0..2 {
                            total.resolve[i] = (total.resolve[i] + overlap.resolve[i]) / 2.0;
                        }
                    } else {
                        total_overlap = Some(overlap);
                    }
                }
            }
        }

        total_overlap
    }
}


#[derive(Copy, Clone)]
enum Tile {
    Air,
    Solid(Rectangle),
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

    pub fn apply_force(&mut self, force: [f32; 2]) {
        self.velocity[0] += force[0];
        self.velocity[1] += force[1];
    }

    pub fn tick(&mut self, dt: f32, obstacles: &[Rectangle]) {
        self.velocity[0] -= 20.0 * self.velocity[0] * dt;
        self.velocity[1] -= 1.0 * self.velocity[1] * dt;

        self.velocity[1] -= 20.0 * dt;


        let size = self.rect.size;
        let padded = obstacles.iter().map(|r| {
            Rectangle {
                size: crank::vec2_add(size, r.size),
                ..*r
            }
        }).collect::<Vec<Rectangle>>();


        self.grounded = false;
        let mut remaining_time = dt;
        while remaining_time > 0.0 {
            let delta = Vec2f {
                x: self.velocity[0] * remaining_time,
                y: self.velocity[1] * remaining_time,
            };

            let sweep_start = Vec2f::from(self.rect.center);
            let sweep_end = sweep_start + delta;

            let line = Line {
                start: sweep_start.into(),
                end: sweep_end.into()
            };

            let broad_phase = line.bounding_box();

            let mut first: Option<Intersection> = None;
            for rect in padded.iter() {
                if rect.intersects(&broad_phase) {
                    if let Some(impact) = rect.line_intersection(&line) {
                        if let Some(ref mut f) = first {
                            if impact.time < f.time {
                                *f = impact;
                            }
                        } else {
                            first = Some(impact);
                        }
                    }
                }
            }

            if let Some(impact) = first {
                let time_left = 1.0 - impact.time;
                remaining_time *= time_left;

                let dot = impact.normal[1] * self.velocity[0] + impact.normal[0] * self.velocity[1];

                self.velocity[0] = dot * impact.normal[1];
                self.velocity[1] = dot * impact.normal[0];

                self.rect.center[0] = impact.point[0];
                self.rect.center[1] = impact.point[1];

                if impact.normal[1] == 1.0 {
                    self.grounded = true;
                }
            } else {
                self.rect.center[0] = sweep_end[0];
                self.rect.center[1] = sweep_end[1];
                remaining_time = 0.0;
            }
        }
    }

    pub fn jump(&mut self) {
        if self.grounded {
            self.grounded = false;
            self.apply_force([0.0, 14.0]);
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


impl<C: Collide<Rectangle>> Collide<C> for Player {
    fn intersects(&self, other: &C) -> bool {
        other.intersects(&self.rect)
    }

    fn overlap(&self, other: &C) -> Option<Overlap> {
        let overlap = other.overlap(&self.rect);
        match overlap {
            Some(overlap) => {
                Some(Overlap {
                    resolve: [-1.0 * overlap.resolve[0], -1.0 * overlap.resolve[1]],
                    ..overlap
                })
            }

            None => None
        }
    }
}
