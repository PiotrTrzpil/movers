

extern crate gfx_device_gl;
extern crate piston_window;

use std::cell::*;
use std::rc::*;
use std::mem;
use piston_window::*;
use opengl_graphics::{ GlGraphics };
use cgmath::{ Vector2 };
use cgmath::InnerSpace;
use piston_window::*;
use app::mover::*;
use app::shapes::*;
use app::textures::*;
use app::selection::*;

pub struct Game {
    movers: Vec<Rc<RefCell<Mover>>>,
    last_mouse_pos: Vector2<f64>,
    selection: Option<Rc<RefCell<Mover>>>,
    selection_shape: Option<Selection>,
    textures: Textures,
    texture: Texture<gfx_device_gl::Resources>
}

impl Game {

    pub fn new(textures: Textures, tex: Texture<gfx_device_gl::Resources>) -> Game {
        Game {
            movers: vec![],
            last_mouse_pos: Vector2::new(0.0, 0.0),
            selection: None,
            selection_shape: None,
            textures: textures,
            texture: tex
        }
    }

    pub fn add_mover(&mut self, mover: Mover) {
        self.movers.push(Rc::new(RefCell::new(mover)));
    }

    pub fn start_selection(&mut self) {
        println!("start sel ");
        self.selection_shape = Some(Selection::new(self.last_mouse_pos))
    }

    pub fn end_selection(&mut self) {
        println!("end sel ");
        {
            let bounds = self.selection_shape.as_ref().unwrap().as_myrect();
            println!("Bounds is: {}", bounds);
            for mover_rc in &mut self.movers {
                let cell = &(*mover_rc);
                let mover = &cell.borrow_mut();
                if mover.position.is_in_bounds(&bounds) {
                    if self.selection.is_some() {
                        let selection = mem::replace(&mut self.selection, None).unwrap();
                        drop(selection);
                    }
                    println!("Selected at {}, {}", mover.position.x, mover.position.y);
                    self.selection = Some(mover_rc.clone());
                }
            }
        }
        self.selection_shape = None;
    }

    pub fn input_button(&mut self, button: Button, pressed: bool) {
        if pressed {
            match button {
                Button::Mouse(MouseButton::Right) => {
                    if let Some(ref mut selection_rc) = self.selection {
                        let cell = &(*selection_rc);
                        let mut selection = cell.borrow_mut();
                        selection.target = Some(self.last_mouse_pos);
                    };
                }
                Button::Mouse(MouseButton::Left) => {
                    self.start_selection();
                }
                _ => ()
            }
        } else {
            match button {
                Button::Mouse(MouseButton::Left) => {
                    self.end_selection();
                }
                Button::Mouse(MouseButton::Right) => {

                }
                _ => ()
            }
        }
    }

    pub fn mouse_cursor(&mut self, x: f64, y:f64) {
        self.last_mouse_pos = Vector2::new(x, y);
        if let Some(ref mut selection) = self.selection_shape {
            selection.update_pos(Vector2::new(x, y));
        };
    }

    pub fn render(&mut self, window: &mut PistonWindow, e: &piston_window::Event, _: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        window.draw_2d(e, |c, gl| {
            clear(GREEN, gl);
            for x in 0..8 {
                for y in 0..5 {
                    let trans: &math::Matrix2d = &c.transform;
                    let transform2 = trans.trans(150.0 * x as f64, 150.0 * y as f64);//.scale(0.333, 0.333);
                    image(&self.texture, transform2, gl);
                }
            }
        });

        for mover_rc in &self.movers {
            let mover = &mut (*mover_rc).borrow_mut();
            mover.render(window, e, &self.textures);
        }

        if let Some(ref selection) = self.selection_shape {
            selection.render(window, e);
        };
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for mover_rc in &mut self.movers {
            let mover = &mut (*mover_rc).borrow_mut();
            mover.update(args);
        }
    }
}