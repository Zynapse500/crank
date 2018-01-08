use ::collision::{Collide, Impact};
use ::{FloatType, Vector2};


pub trait PhysicsObject {
    type CollisionBody: Clone;

    fn tick(&mut self, dt: FloatType, obstacles: &[Box<&Collide<Self::CollisionBody>>]) {
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


    fn update_position(&mut self, dt: FloatType, obstacles: &[Box<&Collide<Self::CollisionBody>>]) {
        let velocity = self.get_velocity();
        let delta = velocity * dt;

        let position = self.get_position();

        self.set_position(position + delta);

        'collision: loop {
            let this_collider = self.get_collider().clone();
            for obstacle in obstacles {
                if let Some(overlap) = obstacle.overlap(&this_collider) {

                    print_deb!(overlap);

                    let position = self.get_position();
                    let normal = overlap.resolve.normal();

                    // Move out of overlap
                    self.set_position( position - overlap.resolve - 0.01 * normal);

                    let impact = Impact {
                        time: 0.0,
                        normal: -normal
                    };

                    self.handle_impact(impact);
                    continue 'collision;
                }
            }

            break;
        }
    }

    fn handle_impact(&mut self, impact: Impact) {
        let velocity = self.get_velocity();

        let dot = impact.normal[1] * velocity[0] + impact.normal[0] * velocity[1];

        self.set_velocity(dot * impact.normal);
    }
}
