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
pub struct  Boid {}

impl Boid {
    pub fn new() -> Self {
        Self {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;

}