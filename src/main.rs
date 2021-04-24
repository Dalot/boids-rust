extern crate rltk;
use rand::Rng;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use systems::{BoidSystem, MovementSys};
mod components;
use components::*;
mod systems;

const WIDTH: f64 = 80.0;
const HEIGHT: f64 = 50.0;
const SCALE: f64 = 50000.0;

#[derive(Default, Debug)]
pub struct DeltaTime(f32);

#[derive(Default, Debug)]
pub struct Flock {
    pub positions: Vec<Position>,
}

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
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Velocity>();
    gs.ecs.register::<Boid>();
    gs.ecs.insert(DeltaTime(0.0));
    gs.ecs.insert(Flock {
        positions: Vec::new(),
    });

    let mut rng = rand::thread_rng();
    for _ in 0..3 {
        let pos = Position::new(
            rng.gen_range(5.0..WIDTH - 5.0),
            rng.gen_range(5.0..HEIGHT - 5.0),
        );
        {
            let mut flock = gs.ecs.write_resource::<Flock>();
            flock.positions.push(pos);
        }

        gs.ecs
            .create_entity()
            .with(Renderable {
                glyph: rltk::to_cp437('_'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(pos)
            .with(Velocity::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ))
            .with(Boid::new())
            .build();
    }

    rltk::main_loop(context, gs)
}
