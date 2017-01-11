extern crate std;
extern crate piston_window;
extern crate uuid;

use std::cell::*;
use std::rc::*;
use std::mem;
use piston_window::*;
use opengl_graphics::{ GlGraphics };
use cgmath::{ Vector2 };
use cgmath::InnerSpace;
use app::shapes::*;
use std::cmp::*;
use piston_window::*;
use app::textures::*;
use app::uuid::Uuid;
use app::ObjectId;



pub struct Mover {
    pub id: ObjectId,
    rotation: f64,
    pub target: Option<Vector2<f64>>,
    pub position: Vector2<f64>
}

impl Mover {

    pub fn new(position: Vector2<f64>) -> Mover {
        Mover {
            id: Uuid::new_v4(),
            rotation: 10.0,
            target: None,
            position: position
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        let speed = 80.0;
        if let Some(target) = self.target {
            let pos_diff = (target - self.position).normalize() * speed * args.dt;
            let new_position = self.position + pos_diff;

            if (new_position - target).magnitude() < 1.0 {
                self.target = None;
                self.position = target;
            } else {
                self.position = new_position;
            }

            self.position = new_position;
        };
    }

    pub fn render(&self, window: &mut PistonWindow, e: &piston_window::Event, textures: &Textures) {
        use graphics::*;

        let (x, y) = (self.position.x,
                      self.position.y);

        window.draw_2d(e, |c, gl| {
            let trans = c.transform.trans(x, y).scale(0.3, 0.3).trans(-40.0, -50.0);
            image(textures.get(&"robot.png".to_string()), trans, gl);
        });
    }
}

pub struct Static {
    pub position: Vector2<f64>
}
impl Static {
    pub fn new(position: Vector2<f64>) -> Static {
        Static {
            position: position
        }
    }

    pub fn render(&self, window: &mut PistonWindow, e: &piston_window::Event, textures: &Textures) {
        use graphics::*;

        let (x, y) = (self.position.x,
                      self.position.y);

        window.draw_2d(e, |c, gl| {
            let trans = c.transform.trans(x, y).scale(0.3, 0.3).trans(-40.0, -50.0);
            image(textures.get(&"storage.png".to_string()), trans, gl);
        });
    }
}
