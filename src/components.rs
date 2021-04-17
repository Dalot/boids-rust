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

pub struct  Boid {
    pub position: Position,
    velocity: Velocity,
    last_update: Duration,
    pub color_index: usize,
    color_factor: u8
}

impl Boid {
    pub fn new() -> Self {
        Self {
         position: Position::new(0.0,0.0),
         velocity: Velocity::new(0.0,0.0),
         last_update: SystemTime::now().elapsed().unwrap(),
         color_index: 0,
         color_factor: 3
        }
    }
}
