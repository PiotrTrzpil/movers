
use std::cell::*;
use std::rc::*;
use std::mem;
use piston_window::*;
use opengl_graphics::{ GlGraphics };
use cgmath::{ Vector2 };
use cgmath::InnerSpace;

use app::mover::*;
use app::shapes::*;



pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    movers: Vec<Rc<RefCell<Mover>>>,
    last_mouse_pos: Vector2<f64>,
    selection: Option<Rc<RefCell<Mover>>>,
    selection_shape: Option<Selection>
}

impl Game {

    pub fn new(opengl: OpenGL) -> Game {
        Game {
            gl: GlGraphics::new(opengl),
            movers: vec![],
            last_mouse_pos: Vector2::new(0.0, 0.0),
            selection: None,
            selection_shape: None,
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
    pub fn handle_selection(&mut self) -> bool {
        let mut handled = false;
        for mover_rc in &mut self.movers {
            let cell = &(*mover_rc);
            let mover = &cell.borrow_mut();
            let bounds = MyRectangle {
                position: mover.position - Vector2::new(10.0, 10.0),
                width: 20.0,
                height: 20.0
            };
            if self.last_mouse_pos.is_in_bounds(&bounds) {
                if self.selection.is_some() {
                    let selection = mem::replace(&mut self.selection, None).unwrap();
                    drop(selection);
                }
                println!("Selected at {}, {}", mover.position.x, mover.position.y);
                self.selection = Some(mover_rc.clone());
                handled = true;
            }
        }
        handled
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

    pub fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_, gl| {
            clear(GREEN, gl);
        });

        for mover_rc in &self.movers {
            let mover = &mut (*mover_rc).borrow_mut();
            mover.render(args, &mut self.gl);
        }

        if let Some(ref selection) = self.selection_shape {
            selection.render(args, &mut self.gl);
        };
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for mover_rc in &mut self.movers {
            let mover = &mut (*mover_rc).borrow_mut();
            mover.update(args);
        }
    }
}