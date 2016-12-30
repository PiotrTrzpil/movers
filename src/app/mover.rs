extern crate std;
extern crate piston_window;
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

pub struct Selection {
    start_pos: Vector2<f64>,
    end_pos: Vector2<f64>
}
impl Selection {
    pub fn new(start_pos: Vector2<f64>) -> Selection {
        Selection {
            start_pos: start_pos,
            end_pos: start_pos
        }
    }

    fn mymin(a: f64, b: f64) -> f64 {
        let ord = a.partial_cmp(&b).unwrap_or(Ordering::Less);
        if ord == Ordering::Less {
            a
        } else {
            b
        }
    }
    pub fn as_myrect(&self) -> MyRectangle {
        let pos_x = Selection::mymin(self.end_pos.x, self.start_pos.x);
        let pos_y = Selection::mymin(self.end_pos.y, self.start_pos.y);

        MyRectangle {
            width: (self.end_pos.x - self.start_pos.x).abs(),
            height: (self.end_pos.y - self.start_pos.y).abs(),
            position: Vector2::new(pos_x, pos_y),
        }
    }

    fn as_rectangle(&self) -> [f64; 4] {
        [self.start_pos.x, self.start_pos.y, self.end_pos.x - self.start_pos.x, self.end_pos.y - self.start_pos.y]
    }

    pub fn render(&self, window: &mut PistonWindow, e: &piston_window::Event) {
        let grid = deform::DeformGrid::new(
            self.as_rectangle(),
            1, 1
        );

        use graphics::*;
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        window.draw_2d(e, |c, gl| {

            grid.draw_vertical_lines(
                &Line::new(RED, 0.5),
                &c.draw_state,
                c.transform,
                gl
            );
            grid.draw_horizontal_lines(
                &Line::new(RED, 0.5),
                &c.draw_state,
                c.transform,
                gl
            );
        });
    }

    pub fn update_pos(&mut self, last_mouse_pos: Vector2<f64>) {
        self.end_pos = last_mouse_pos;
    }
}


pub struct Mover {
    rotation: f64,
    pub target: Option<Vector2<f64>>,
    pub position: Vector2<f64>
}

impl Mover {

    pub fn new(position: Vector2<f64>) -> Mover {
        Mover {
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
