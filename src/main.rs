extern crate rltk;

use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};
use systems::{MovementSys, BoidSystem};
mod components;
use components::*;
mod systems;

const VEL_STEP: f32 = 1000.0;

#[derive(Default, Debug)]
pub struct DeltaTime(f32);
struct State {
    ecs: World,
}
impl State {
    fn run_systems(&mut self, ctx: &mut Rltk) {
        let mut sys = MovementSys {};
        sys.run_now(&self.ecs);
        let mut sys = BoidSystem {
            ctx
        };
        sys.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems(ctx);
        let now = std::time::Instant::now();
        {
            let mut delta = self.ecs.write_resource::<DeltaTime>();
            *delta = DeltaTime(0.0);
        }
        {
            let mut delta = self.ecs.write_resource::<DeltaTime>();
            *delta = DeltaTime(now.elapsed().as_micros() as f32 / 1000000.0 + delta.0);
        }
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Velocity>();
    gs.ecs.register::<Boid>();
    gs.ecs.insert(DeltaTime(0.0));

    gs.ecs
        .create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('_'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Position::new(38.0, 25.0))
        .with(Velocity::new(10.0, 10.0))
        .with(Boid::new())
        .build();

    rltk::main_loop(context, gs)
}
