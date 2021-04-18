use specs::prelude::*;

use crate::components::{Position, Velocity};
use crate::{DeltaTime, VEL_STEP};

pub struct Sys;
impl<'a> System<'a> for Sys {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        let delta = delta.0;
        for (pos, vel) in (&mut pos, &vel).join() {
            update_position(pos, vel, delta);
        }
    }
}

fn update_position(pos: &mut Position, vel: &Velocity, delta: f32) {
    pos.x += vel.x * VEL_STEP as f32 * delta;
    pos.y += vel.y * VEL_STEP as f32 * delta;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_update() {
        let mut pos = Position::new(1.0, 1.0);
        let vel = Velocity::new(1.0, 1.0);
        let delta = 0.001;
        update_position(&mut pos, &vel, delta); 
        assert_eq!(pos.x as i32, 2.0 as i32);
        assert_eq!(pos.y as i32, 2.0 as i32);
    }
}