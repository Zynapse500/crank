use ::collision::{Collide, Impact};


pub trait Body<T>: Collide<T> {}

impl<T> Body<T> for T
    where T: Collide<T>
{}


pub trait PhysicsObject {
    type CollisionBody: Clone;

    fn tick(&mut self, dt: f32, obstacles: &[Box<&Body<Self::CollisionBody>>]) {
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


    fn update_position(&mut self, dt: f32, obstacles: &[Box<&Body<Self::CollisionBody>>]) {
        let velocity = self.get_velocity();
        let delta = [
            velocity[0] * dt,
            velocity[1] * dt,
        ];

        let position = self.get_position();

        self.set_position([
            position[0] + delta[0],
            position[1] + delta[1]
        ]);

        'collision: loop {

            let mut this_collider = self.get_collider().clone();
            for obstacle in obstacles {
                if let Some(overlap) = obstacle.overlap(&this_collider) {
                    let position = self.get_position();
                    let normal = ::vec2_normalize(overlap.resolve);

                    // Move out of overlap
                    self.set_position([
                        position[0] - overlap.resolve[0] - 0.00 * normal[0],
                        position[1] - overlap.resolve[1] - 0.00 * normal[1],
                    ]);


                    let impact = Impact {
                        time: 0.0,
                        normal: [-normal[0], -normal[1]],
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

        self.set_velocity([
            dot * impact.normal[1],
            dot * impact.normal[0]
        ]);
    }
}
