extern crate rltk;

use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};
use systems::Sys;
mod components;
use components::*;
mod systems;

#[derive(Default, Debug)]
pub struct DeltaTime(f32);
struct State {
    ecs: World,
}
impl State {
    fn run_systems(&mut self) {
        let mut sys = Sys{};
        sys.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
        let now = std::time::Instant::now();
        {
            let mut delta = self.ecs.write_resource::<DeltaTime>();
            *delta = DeltaTime(0.0);
        }  

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x as i32, pos.y as i32, render.fg, render.bg, render.glyph);
        }

        {
            let mut delta = self.ecs.write_resource::<DeltaTime>();
            *delta = DeltaTime(now.elapsed().as_secs_f32() * 1000.0 + delta.0);
            dbg!(delta.0);
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
    gs.ecs.insert(DeltaTime(0.0));

    gs.ecs
        .create_entity()
        .with(Position { x: 40.0, y: 25.0 })
        .with(Velocity { x:1.0, y: 1.0 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    rltk::main_loop(context, gs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_boid() {
        assert_eq!(Boid::new().color_index, 0);
    }
}
