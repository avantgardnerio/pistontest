extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use glutin_window::GlutinWindow as Window;

use graphics::*;
use graphics::rectangle::square;

use opengl_graphics::{GlGraphics, OpenGL, Texture};

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;

use std::collections::HashSet;

use std::path::Path;


const GREEN: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const SPEED: f64 = 200.0;

pub struct App {
    gl: GlGraphics,
    x: f64,
    y: f64,
    image : Image,
    texture : Texture,
    keys: HashSet<Key>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
    	let texture = &self.texture;
    	let image = &self.image;
        let (x, y) = (&self.x, &self.y);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            let transform = c.transform.trans(*x, *y).trans(-45.0, -64.0);
            image.draw(texture, &DrawState::default(), transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
    	if self.keys.contains(&Key::Left) {
    		self.x -= SPEED * args.dt;
    	}
    	if self.keys.contains(&Key::Right) {
    		self.x += SPEED * args.dt;
    	}
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [1024, 768]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
    
    let mut app = App {
		gl: GlGraphics::new(opengl),
		x: 1024.0 / 2.0,
		y: 768.0 - 64.0,
		image: Image::new().rect([0.0, 0.0, 90.0, 128.0]),
		texture: Texture::from_path(Path::new("src/spaceship.png")).unwrap(),
		keys: HashSet::new()
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
        	app.keys.insert(key);
        }
        
        if let Some(Button::Keyboard(key)) = e.release_args() {
        	app.keys.remove(&key);
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}