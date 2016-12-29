#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(unused_imports)]

extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate cgmath;

use std::cell::*;
use std::rc::*;
use std::mem;
use piston_window::*;
use opengl_graphics::{ GlGraphics };
use cgmath::{ Vector2 };
use cgmath::InnerSpace;

mod app;
use app::game::*;
use app::mover::*;

fn main() {
    let opengl = OpenGL::V3_2;

    let (width, height) = (300, 300);
    let mut window: PistonWindow =
    WindowSettings::new("Movers", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mover1 = Mover::new(Vector2::new(width as f64 /2.0 - 20.0, height as f64 /2.0 - 20.0));
    let mover2 = Mover::new(Vector2::new(width as f64 /2.0 + 25.0, height as f64 /2.0 + 25.0));
    let mut app = Game::new(opengl);
    app.add_mover(mover1);
    app.add_mover(mover2);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {

        if let Some(button) = e.press_args() {
            app.input_button(button, true);
        };
        if let Some(button) = e.release_args() {
            app.input_button(button, false);
        };

        if let Some(mouse_pos) = e.mouse_cursor_args() {
            app.mouse_cursor(mouse_pos[0], mouse_pos[1]);
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}