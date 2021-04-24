use crate::SCALE;
use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;
use std::time::{Duration, SystemTime};
#[derive(Component)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Velocity {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: x * SCALE,
            y: y * SCALE,
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, posA: &Position, posB: &Position) -> std::cmp::Ordering {

        let resA = ((posA.x - self.x).powi(2) + (posA.y - self.y).powi(2)).sqrt();
        let resB = ((posB.x - self.x).powi(2) + (posB.y - self.y).powi(2)).sqrt();

        resA.partial_cmp(&resB).unwrap()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Boid {}

impl Boid {
    pub fn new() -> Self {
        Self {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
