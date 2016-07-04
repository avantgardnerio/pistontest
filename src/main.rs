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

use std::path::Path;


const GREEN: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    image : Image,
    texture : Texture
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
    	let rotation = self.rotation;
    	let texture = &self.texture;
    	let image = &self.image;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            let transform = c.transform.trans(x, y).rot_rad(rotation).trans(-64.0, -64.0);
            image.draw(texture, &DrawState::default(), transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [200, 200]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
    
    let mut app = App {
		gl: GlGraphics::new(opengl),
		rotation: 0.0,
		image: Image::new().rect(square(0.0, 0.0, 200.0)),
		texture: Texture::from_path(Path::new("src/spaceship.png")).unwrap()
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}