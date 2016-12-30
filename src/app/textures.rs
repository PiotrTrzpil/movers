#![allow(get_unwrap)]

extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate cgmath;
extern crate find_folder;
extern crate gfx_device_gl;

use std::cell::*;
use std::rc::*;
use std::mem;
use std::path::PathBuf;
use piston_window::*;

use opengl_graphics::{ GlGraphics };
use cgmath::{ Vector2 };
use cgmath::InnerSpace;
use piston_window::*;
use app::mover::*;
use app::shapes::*;
use std::collections::HashMap;

pub struct Textures {
    assets_path: PathBuf,
    textures: HashMap<String, Texture<gfx_device_gl::Resources>>
}

impl Textures {

    pub fn new() -> Textures {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        Textures {
            assets_path: assets,
            textures: HashMap::new()
        }
    }

    pub fn load(&self, window: &mut PistonWindow, name: &str) -> Texture<gfx_device_gl::Resources> {
        let texture = self.assets_path.join(name);
        Texture::from_path(
            &mut window.factory,
            &texture,
            Flip::None,
            &TextureSettings::new()
        ).unwrap()
    }

    pub fn load_into_map(&mut self, window: &mut PistonWindow, name: String) {
        let tex = self.load(window, name.as_ref());
        self.textures.insert(name, tex);
    }

    pub fn get(& self, name: &str) -> & Texture<gfx_device_gl::Resources> {
        self.textures.get(name).unwrap()
    }
}