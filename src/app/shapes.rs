
use std::cell::*;
use std::rc::*;
use std::mem;
use piston_window::*;
use opengl_graphics::{ GlGraphics };
use cgmath::{ Vector2 };
use cgmath::InnerSpace;
use std::fmt::*;
use std::*;

pub struct MyRectangle {
    pub position: Vector2<f64>,
    pub width: f64,
    pub height: f64
}

impl Display for MyRectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyRectangle: ({},{}), w:{}, h:{}", self.position.x, self.position.y, self.width, self.height)
    }
}

pub trait BoundsCheck {
    fn is_in_bounds(&self, rect: &MyRectangle) -> bool;
}

impl BoundsCheck for Vector2<f64> {
    fn is_in_bounds(&self, rect: &MyRectangle) -> bool {
        self.x >= rect.position.x && self.x <= rect.position.x + rect.width &&
            self.y >= rect.position.y && self.y <= rect.position.y + rect.height
    }
}