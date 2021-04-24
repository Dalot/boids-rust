use crate::components::{Position, Velocity};
use crate::{Boid, DeltaTime, Flock, Renderable, HEIGHT, WIDTH};
use rltk::Rltk;
use specs::prelude::*;
use std::f64::consts::PI;

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
    pos.x += vel.x as f64 * delta as f64;
    pos.y += vel.y as f64 * delta as f64;
    dbg!(pos);
}

pub struct BoidSystem<'a> {
    pub ctx: &'a mut Rltk,
}
impl<'a> System<'a> for BoidSystem<'_> {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Boid>,
        WriteStorage<'a, Velocity>,
        Read<'a, Flock>,
    );

    fn run(&mut self, (pos, render, boid, mut vel, flock): Self::SystemData) {
        for (pos, render, boid, vel) in (&pos, &render, &boid, &mut vel).join() {
            self.draw_boid(boid, pos, render);

            if pos.x >= WIDTH || pos.x < 1.0 {
                vel.x = -vel.x;
                vel.y = -vel.y;
                break;
            }
            if pos.y >= HEIGHT || pos.y < 1.0 {
                vel.y = -vel.y;
                vel.x = -vel.x;
                break;
            }

            let positions_copy = flock.positions.clone();
            let neighbours = self.neighbours(pos, positions_copy);
            
        }
    }
}

impl<'a> BoidSystem<'a> {
    pub fn draw_boid(&mut self, boid: &Boid, pos: &Position, render: &Renderable) {
        self.ctx.set(
            pos.x as i32,
            pos.y as i32,
            render.fg,
            render.bg,
            rltk::to_cp437('▲'),
        );
    }

    pub fn neighbours(&self, pos: &Position, mut positions: Vec<Position>) -> Vec<Position> {
        positions.sort_unstable_by(|a, b| pos.distance(a, b));
        positions
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

    #[test]
    fn text_sort_stable() {
        let pos = Position::new(20.0, 20.0);
        let mut positions = vec![
            Position::new(25.0, 25.0),
            Position::new(10.0, 10.0),
            Position::new(15.0, 15.0),
            pos.clone(),
        ];
        positions.sort_unstable_by(|a, b| pos.distance(a, b));
        assert_eq!(
            vec![
                Position::new(20.0, 20.0),
                Position::new(25.0, 25.0),
                Position::new(15.0, 15.0),
                Position::new(10.0, 10.0),
            ],
            positions
        );
    }
}
