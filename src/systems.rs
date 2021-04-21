use crate::components::{Position, Velocity};
use crate::{Boid, DeltaTime, Renderable, VEL_STEP};
use rltk::Rltk;
use specs::prelude::*;
use std::fmt;

pub struct MovementSys;
impl<'a> System<'a> for MovementSys {
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

pub struct BoidSystem<'a> {
    pub ctx: &'a mut Rltk,
}
impl<'a> System<'a> for BoidSystem<'_> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Boid>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (pos, render, boid, mut vel): Self::SystemData) {
        for (pos, render, boid, vel) in (&pos, &render, &boid, &mut vel).join() {
            self.draw_boid(pos, render);
            let boid_body = boid.body(pos.x, pos.y);
            for (x, y) in boid_body.iter() {
                if *x >= 79.0 || *x <= 1.0 {
                    vel.x = -vel.x;
                    vel.y = -vel.y;
                    break;
                }
                if *y >= 49.0 || *y <= 1.0 {
                    vel.y = -vel.y;
                    vel.x = -vel.x;
                    break; 
                }
            }
        }
    }
}

impl<'a> BoidSystem<'a> {
    pub fn draw_boid(&mut self, pos: &Position, render: &Renderable) {
        let base = 4;
        let height = 3;
        // Draw the drone

        // the hat first
        for i in 0..base {
            self.ctx.set(
                pos.x as i32 + i as i32,
                pos.y as i32,
                render.fg,
                render.bg,
                render.glyph,
            );
        }

        // One helice and 2 side of the square
        for i in 0..height {
            self.ctx.set(
                pos.x as i32 + i as i32,
                pos.y as i32 + i as i32,
                render.fg,
                render.bg,
                rltk::to_cp437('/'),
            );
        }

        // The other helice and other 2 sides of the square
        for i in 0..height {
            self.ctx.set(
                pos.x as i32 + height as i32 - i as i32,
                pos.y as i32 + i as i32,
                render.fg,
                render.bg,
                rltk::to_cp437('\\'),
            );
        }
    }
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
