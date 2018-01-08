use ::collision::{Collide, Impact, Sweep};
use ::{FloatType, Vector2};

pub trait Body<T>: Collide<T> + Sweep<T> {}

impl<T> Body<T> for T where T: Collide<T> + Sweep<T> {}

pub trait PhysicsObject {
    type CollisionBody: Clone;

    fn tick(&mut self, dt: FloatType, obstacles: &[Box<&Body<Self::CollisionBody>>]) {
        self.update_velocity(dt);
        self.update_position(dt, obstacles);
    }

    fn update_velocity(&mut self, dt: FloatType) {
        let velocity = self.get_velocity();
        let drag = self.get_drag();

        self.apply_force(drag * velocity * -dt);
    }

    fn get_position(&self) -> Vector2;
    fn set_position(&mut self, position: Vector2);

    fn get_velocity(&self) -> Vector2;
    fn set_velocity(&mut self, velocity: Vector2);

    fn get_drag(&self) -> Vector2;
    fn set_drag(&mut self, drag: Vector2);

    fn apply_force(&mut self, force: Vector2) {
        let velocity = self.get_velocity();

        self.set_velocity(velocity + force);
    }

    fn get_collider<'a>(&'a self) -> &'a Self::CollisionBody;


    fn update_position(&mut self, dt: FloatType, obstacles: &[Box<&Body<Self::CollisionBody>>]) {

        // self.set_position(position + delta);
        let mut remaining_time = 1.0;

        while remaining_time > 0.0 {
            let mut delta = self.get_velocity() * dt * remaining_time;

            let this_collider = self.get_collider().clone();

            let mut first: Option<Impact> = None;

            for obstacle in obstacles {
                if let Some(impact) = obstacle.sweep(-delta, &this_collider) {
                    if let Some(ref mut first) = first {
                        if impact.time < first.time {
                            *first = impact.inverse();
                        }
                    } else {
                        first = Some(impact.inverse());
                    }
                }
            }


            if let Some(impact) = first {
                delta *= impact.time;
                remaining_time *= 1.0 - impact.time;

                self.handle_impact(impact);
            } else {
                remaining_time = 0.0;
            }

            let position = self.get_position();
            self.set_position(position + delta);

            // Avoid overlaps
            for obstacle in obstacles {
                if let Some(overlap) = obstacle.overlap(&this_collider) {
                    let position = self.get_position();
                    let normal = overlap.resolve.normal();

                    // Move out of overlap
                    self.set_position( position - overlap.resolve - 0.01 * normal);

                    let impact = Impact {
                        time: 0.0,
                        normal: -normal
                    };

                    self.handle_impact(impact);
                    break;
                }
            }
        }
    }

    fn handle_impact(&mut self, impact: Impact) {
        let velocity = self.get_velocity();
        let rotated_normal: Vector2 = [impact.normal.y, impact.normal.x].into();

        let dot = velocity.dot(rotated_normal);

        self.set_velocity(dot * rotated_normal);
    }
}
