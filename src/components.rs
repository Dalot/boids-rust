use crate::SCALE;
use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;
use std::hash::{Hash, Hasher};

#[derive(Component, Clone, Debug)]
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

#[derive(Component, Clone, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, pos_a: &Position, pos_b: &Position) -> std::cmp::Ordering {

        let res_a = ((pos_a.x - self.x).powi(2) + (pos_a.y - self.y).powi(2)).sqrt();
        let res_b = ((pos_b.x - self.x).powi(2) + (pos_b.y - self.y).powi(2)).sqrt();

        res_a.partial_cmp(&res_b).unwrap()
    }

    // since rust does not support overloading methos, there is an opportunity here for possible refactor 
    pub fn distance_to(&self, pos_a: &Position) -> f64 {
        ((pos_a.x - self.x).powi(2) + (pos_a.y - self.y).powi(2)).sqrt()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_string().hash(state);
        self.y.to_string().hash(state);
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
