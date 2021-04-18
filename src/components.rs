use specs_derive::Component;
use rltk::{RGB};
use std::time::{Duration, SystemTime};
use specs::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }    
}

#[derive(Component, Debug)]
pub struct  Position {
    pub x: f32,
    pub y: f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }    
}

#[derive(Component)]
pub struct  Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct  Boid {
    pub position: Position,
    pub velocity: Velocity,
}

impl Boid {
    pub fn new(position: Position, velocity: Velocity) -> Self {
        Self {
            position,
            velocity,
        }
    }

    pub fn default() -> Self {
        Self {
            position: Position::new(0.0, 0.0),
            velocity: Velocity::new(0.0, 0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boid_default() {
        let boid = Boid::default();
        assert_eq!(boid.position.x as i32, 0.0 as i32);
    }
}