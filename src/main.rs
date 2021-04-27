extern crate rltk;
use rand::Rng;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use systems::{BoidSystem, MovementSys};
mod components;
use components::*;
mod systems;

const WIDTH: f64 = 150.0;
const HEIGHT: f64 = 100.0;
const SCALE: f64 = 1.0;
const SEPARATION_FACTOR: f64 = 3.0;
const COHERENCE_FACTOR: f64 = 7.0;
const MAX_PROXIMAL_BOIDS: u32 = 9;

#[derive(Default, Debug)]
pub struct DeltaTime(f32);

struct State {
    ecs: World,
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
    let context = RltkBuilder::simple(WIDTH as u32, HEIGHT as u32)?
        .with_title("Boids Simulation")
        .build()?;
    let mut gs = State { ecs: World::new() };
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
            .with(Velocity::new(2.0, 2.0))
            .with(Boid::new())
            .build();
    }

    rltk::main_loop(context, gs)
}
