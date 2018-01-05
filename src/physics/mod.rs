
use ::collision::{Collide, Sweep, Impact};

pub trait PhysicsObject {
    type C;

    fn tick(&mut self, dt: f32, obstacles: &[Box<Sweep<C> + Collide<C>>]) {
        self.update_velocity(dt);
        self.update_position(dt, obstacles);
    }

    fn update_velocity(&mut self, dt: f32) {
        let position = self.get_position();
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

    fn get_collider<'a>(&'a self) -> &'a Self::C;

    fn update_position(&mut self, dt: f32, obstacles: &[Box<Sweep<C> + Collide<C>>]) {
        let mut remaining_time = dt;

        while remaining_time > 0.0 {
            let velocity = self.get_velocity();
            let other_delta = [
                -velocity[0] * remaining_time,
                -velocity[1] * remaining_time,
            ];

            let this_collider = self.get_collider();

            let mut first: Option<Impact> = None;
            for obstacle in obstacles {
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

            let position = self.get_position();

            if let Some(impact) = first {
                self.set_position([
                    position[0] - other_delta[0] * impact.time,
                    position[1] - other_delta[1] * impact.time,
                ]);

                self.handle_impact(impact);

                remaining_time *= 1.0 - impact.time;
            } else {
                self.set_position([
                    position[0] - other_delta[0],
                    position[1] - other_delta[1]
                ]);

                remaining_time = 0.0;
            }
        }
    }

    fn handle_impact(&mut self, impact: Impact) {
        let velocity = self.get_velocity();

        let dot = impact.normal[1] * velocity[0] + impact.normal[0] * velocity[1];

        velocity[0] = dot * impact.normal[1];
        velocity[1] = dot * impact.normal[0];
    }
}
