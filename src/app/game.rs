

extern crate gfx_device_gl;
extern crate piston_window;
extern crate futures;
extern crate tokio_timer;
extern crate tokio_core;
extern crate futures_cpupool;

use self::futures_cpupool::*;
use self::futures::*;
use self::futures::future::*;
use self::tokio_timer::*;
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
use std::time::*;
use self::tokio_core::reactor::*;
use std::thread;
use std::sync::mpsc;
use app::ObjectId;

pub struct DefaultInputMode {
    last_mouse_pos: Vector2<f64>,
}
struct SelectionInputMode {
    selection_shape: Selection,
}

pub trait Command {

}

struct MoveCommand {
    objectId: ObjectId,
    target: Vector2<f64>
}

impl MoveCommand {

}

impl Command for MoveCommand {

}

pub trait InputMode {
    //fn new() -> Self;
    fn mouse_cursor(&mut self, _: f64, _:f64) {

    }
    fn input_button(&mut self, _: &mut GameLogic, _: Button, _: bool) -> Option<Box<InputMode>>;


    fn render(&mut self, _: &mut PistonWindow, _: &piston_window::Event, _: &Textures) {
    }
}

impl DefaultInputMode {
    fn new() -> DefaultInputMode {
        DefaultInputMode {
            last_mouse_pos: Vector2::new(0.0, 0.0)
        }
    }
    fn start_selection(&mut self, game: &mut GameLogic) -> SelectionInputMode {
        SelectionInputMode::new(game.last_mouse_pos)
    }
}

impl InputMode for DefaultInputMode {

    fn mouse_cursor(&mut self, x: f64, y:f64) {
        self.last_mouse_pos = Vector2::new(x, y);
    }
    fn input_button(&mut self, game: &mut GameLogic, button: Button, pressed: bool) -> Option<Box<InputMode>> {
        if pressed {
            match button {
                Button::Mouse(MouseButton::Right) => {
                    if let Some(ref mut selection_rc) = game.selection {
                        let cell = &(*selection_rc);
                        let mut selection: RefMut<Mover> = cell.borrow_mut();
                        game.processor.send_command(Box::new(MoveCommand{objectId: selection.id, target: game.last_mouse_pos}));
                        selection.target = Some(game.last_mouse_pos);
                    };
                    None
                }
                Button::Mouse(MouseButton::Left) => {
                    Some(Box::new(self.start_selection(game)))
                }
                _ => None
            }
        } else {
            None
        }
    }
}

impl SelectionInputMode {
    fn new(last_mouse_pos: Vector2<f64>) -> SelectionInputMode {
        SelectionInputMode {
            selection_shape: Selection::new(last_mouse_pos)
        }
    }
    pub fn end_selection(&mut self, game: &mut GameLogic) -> DefaultInputMode {
        {
            let bounds = self.selection_shape.as_myrect();
            println!("Bounds is: {}", bounds);
            for mover_rc in &mut game.movers {
                let cell = &(*mover_rc);
                let mover = &cell.borrow_mut();
                if mover.position.is_in_bounds(&bounds) {
                    if game.selection.is_some() {
                        let selection = mem::replace(&mut game.selection, None).unwrap();
                        drop(selection);
                    }
                    println!("Selected at {}, {}", mover.position.x, mover.position.y);
                    game.selection = Some(mover_rc.clone());
                }
            }
        }
        DefaultInputMode::new()
    }
}

impl InputMode for SelectionInputMode {

    fn render(&mut self, window: &mut PistonWindow, e: &piston_window::Event, _: &Textures) {
        self.selection_shape.render(window, e);
    }

    fn mouse_cursor(&mut self, x: f64, y:f64) {
        self.selection_shape.update_pos(Vector2::new(x, y));
    }

    fn input_button(&mut self, game: &mut GameLogic, button: Button, pressed: bool) -> Option<Box<InputMode>> {
        if pressed {
            None
        } else {
            match button {
                Button::Mouse(MouseButton::Left) => {
                    Some(Box::new(self.end_selection(game)))
                }
                Button::Mouse(MouseButton::Right) => {
                    None
                }
                _ => None
            }
        }
    }
}


pub struct CommandProcessor {
    remote: Remote
}
impl CommandProcessor {
    pub fn new(rem: Remote) -> CommandProcessor {
        CommandProcessor {
            remote: rem,
        }
    }

    pub fn send_command(&mut self, command: Box<Command>) {
        self.remote.spawn(|_| {
            ok({
                println!("Sent a command");
            })
        });
    }
}
pub struct GameLogic {
    processor: CommandProcessor,
    last_mouse_pos: Vector2<f64>,
    movers: Vec<Rc<RefCell<Mover>>>,
    statics: Vec<Rc<RefCell<Static>>>,
    selection: Option<Rc<RefCell<Mover>>>
}

impl GameLogic {
    pub fn new(rem: Remote) -> GameLogic {
        GameLogic {
            processor: CommandProcessor{remote: rem},
            last_mouse_pos: Vector2::new(0.0, 0.0),
            movers: vec![],
            statics: vec![Rc::new(RefCell::new(Static::new(Vector2::new(100.0, 100.0))))],
            selection: None
        }
    }
    pub fn add_mover(&mut self, mover: Mover) {
        self.movers.push(Rc::new(RefCell::new(mover)));
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for mover_rc in &mut self.movers {
            let mover = &mut (*mover_rc).borrow_mut();
            mover.update(args);
        }
    }
    pub fn render(&mut self, window: &mut PistonWindow, e: &piston_window::Event, textures: &Textures) {
        if let Some(ref selected_rc) = self.selection {
            let mover = &mut (*selected_rc).borrow_mut();
            use graphics::*;
            let x = mover.position.x;
            let y = mover.position.y;

            const GREEN:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            window.draw_2d(e, |c, gl| {
                Ellipse::new_border(GREEN, 1.0).draw([x - 15.0, y + 20.0, 35.0, 25.0], &c.draw_state, c.transform, gl);
            });
        }
        for mover_rc in &self.movers {
            let mover = &mut (*mover_rc).borrow_mut();
            mover.render(window, e, textures);
        }
        for static_rc in &self.statics {
            let _static = &mut (*static_rc).borrow_mut();
            _static.render(window, e, textures);
        }

    }
}
pub struct Game {
    textures: Textures,
    texture: Texture<gfx_device_gl::Resources>,
    input_mode: Box<InputMode>,
    game_logic: GameLogic
}

impl Game {

    pub fn new(textures: Textures, tex: Texture<gfx_device_gl::Resources>) -> Game {
        let (tx, rx) = mpsc::channel();

        let th = thread::spawn(move || {
            let mut core: Core = Core::new().unwrap();
            let remote = core.remote();
            tx.send(remote).unwrap();
            let _ = core.run(futures::empty::<String, String>());
        });

        let rem: Remote = rx.recv().unwrap();

        rem.spawn(|_| {
            ok({
                println!("Started logic event loop.");
            })
        });

        Game {
            textures: textures,
            texture: tex,
            input_mode: Box::new(DefaultInputMode::new()),
            game_logic: GameLogic::new(rem)
        }
    }

    pub fn add_mover(&mut self, mover: Mover) {
        self.game_logic.add_mover(mover);
    }

    pub fn input_button(&mut self, button: Button, pressed: bool) {
        {
            let new_mode_opt = self.input_mode.input_button(&mut self.game_logic, button, pressed);
            if let Some(new_mode) = new_mode_opt {
                let _ = mem::replace(&mut self.input_mode, new_mode);
            }
        }
    }

    pub fn mouse_cursor(&mut self, x: f64, y:f64) {
        self.game_logic.last_mouse_pos = Vector2::new(x, y);
        (*self.input_mode).mouse_cursor(x, y);
    }

    pub fn render(&mut self, window: &mut PistonWindow, e: &piston_window::Event, _: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        window.draw_2d(e, |c, gl| {
            clear(GREEN, gl);
            for x in 0..8 {
                for y in 0..5 {
                    let trans: &math::Matrix2d = &c.transform;
                    let transform2 = trans.trans(150.0 * x as f64, 150.0 * y as f64);
                    image(&self.texture, transform2, gl);
                }
            }
        });

        self.game_logic.render(window, e, &self.textures);
        (*self.input_mode).render(window, e, &self.textures);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.game_logic.update(args);
    }
}