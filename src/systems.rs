use specs::prelude::*;

use crate::components::{Position, Velocity};
use crate::DeltaTime;

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
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
            dbg!(pos);
        }
    }
}
