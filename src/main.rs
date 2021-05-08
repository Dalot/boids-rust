// NOTE: This is how I usually structure my use statements, but I'm not sure if there's a standard
// way or not.
use std::time::Instant;

use rand::Rng;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use systems::{BoidSystem, MovementSys};

mod components;
use components::*;
mod systems;
mod constants;
use crate::constants::*;

#[derive(Default, Debug)]
pub struct DeltaTime(f32);

struct State {
    ecs: World,
    previous_instant: Instant,
}
impl State {
    fn run_systems(&mut self, ctx: &mut Rltk) {
        let mut sys = MovementSys {};
        sys.run_now(&self.ecs);
        let mut sys = BoidSystem { ctx };
        sys.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let now = Instant::now();
        {
            let mut delta = self.ecs.write_resource::<DeltaTime>();
            delta.0 = (now - self.previous_instant).as_secs_f32();
        }
        self.previous_instant = now;

        ctx.cls();
        self.run_systems(ctx);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(WIDTH as u32, HEIGHT as u32)?
        .with_title("Boids Simulation")
        .build()?;
    let mut gs = State {
        ecs: World::new(),
        previous_instant: Instant::now(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Velocity>();
    gs.ecs.register::<Boid>();
    gs.ecs.insert(DeltaTime(0.0));

    let mut rng = rand::thread_rng();
    for _ in 0..150 {
        let pos = Position::new(rng.gen_range(0.0..WIDTH), rng.gen_range(0.0..HEIGHT));

        gs.ecs
            .create_entity()
            .with(Renderable {
                glyph: rltk::to_cp437('_'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(pos)
            .with(Velocity::new(rng.gen_range(-3.0..3.0), rng.gen_range(-3.0..3.0)))
            .with(Boid{})
            .build();
    }

    rltk::main_loop(context, gs)
}
