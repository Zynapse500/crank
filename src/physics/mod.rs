use ::collision::{Collide, Sweep, Impact};
use ::shape::{Rectangle, Bounded};


pub trait Body<T>: Sweep<T> + Collide<T> + Collide<Rectangle> {}

impl<T> Body<T> for T
    where T: Sweep<T> + Collide<T> + Collide<Rectangle>
{}


pub trait PhysicsObject {
    type CollisionBody: Bounded + Clone;

    fn tick(&mut self, dt: f32, obstacles: &[Box<Body<Self::CollisionBody>>]) {
        self.update_velocity(dt);
        self.update_position(dt, obstacles);
    }

    fn update_velocity(&mut self, dt: f32) {
        let velocity = self.get_velocity();
        let drag = self.get_drag();

        self.apply_force([
            -(drag[0] * velocity[0] * dt),
            -(drag[1] * velocity[1] * dt)
        ]);
    }

    fn get_position(&self) -> [f32; 2];
    fn set_position(&mut self, position: [f32; 2]);

    fn get_velocity(&self) -> [f32; 2];
    fn set_velocity(&mut self, velocity: [f32; 2]);

    fn get_drag(&self) -> [f32; 2];
    fn set_drag(&mut self, drag: [f32; 2]);

    fn apply_force(&mut self, force: [f32; 2]) {
        let velocity = self.get_velocity();

        self.set_velocity([
            velocity[0] + force[0],
            velocity[1] + force[1],
        ]);
    }

    fn get_collider<'a>(&'a self) -> &'a Self::CollisionBody;


    fn update_position(&mut self, dt: f32, obstacles: &[Box<Body<Self::CollisionBody>>]) {
        let mut remaining_time = dt;

        while remaining_time > 0.0 {
            let velocity = self.get_velocity();
            let other_delta = [
                -velocity[0] * remaining_time,
                -velocity[1] * remaining_time,
            ];


            let mut first: Option<Impact> = None;
            {
                let this_collider = self.get_collider();
                let bounding_box: Rectangle = this_collider.bounding_box();

                let broad_phase = Rectangle {
                    center: [
                        bounding_box.center[0] - other_delta[0] / 2.0,
                        bounding_box.center[1] - other_delta[1] / 2.0
                    ],
                    size: [
                        bounding_box.size[0] + other_delta[0].abs(),
                        bounding_box.size[1] + other_delta[1].abs()
                    ],
                };

                for obstacle in obstacles {
                    if obstacle.intersects(&broad_phase) {
                        if let Some(impact) = obstacle.sweep(other_delta, this_collider) {
                            if let Some(ref mut f) = first {
                                if impact.time < f.time {
                                    *f = impact;
                                }
                            } else {
                                first = Some(impact.inverse());
                            }
                        }
                    }
                }
            }

            let position = self.get_position();

            if let Some(impact) = first {
                remaining_time *= 1.0 - impact.time;

                self.set_position([
                    position[0] - other_delta[0] * impact.time,
                    position[1] - other_delta[1] * impact.time,
                ]);

                self.handle_impact(impact);
            } else {
                self.set_position([
                    position[0] - other_delta[0],
                    position[1] - other_delta[1]
                ]);

                remaining_time = 0.0;
            }

            // TODO: Fix collisions failing on y ~= 4.5

            let this_collider = self.get_collider().clone();
            for obstacle in obstacles {
                if let Some(overlap) = obstacle.overlap(&this_collider) {
                    println!("Overlap!");

                    let position = self.get_position();

                    self.set_position([
                        position[0] - overlap.resolve[0] * 1.001,
                        position[1] - overlap.resolve[1] * 1.001,
                    ]);
                }
            }
        }
    }

    fn handle_impact(&mut self, impact: Impact) {
        let velocity = self.get_velocity();

        let dot = impact.normal[1] * velocity[0] + impact.normal[0] * velocity[1];

        self.set_velocity([
            dot * impact.normal[1],
            dot * impact.normal[0]
        ]);
    }
}
