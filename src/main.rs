extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use glutin_window::GlutinWindow as Window;

use graphics::*;

use opengl_graphics::{GlGraphics, OpenGL, Texture};

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;

use std::collections::HashSet;

use std::path::Path;

use rand::random;

use graphics::math::Vec2d;

use std::cell::Cell;

const GREEN: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SPEED: f64 = 200.0;
const STAR_COUNT: usize = 100;
const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 768.0;

pub struct App {
    gl: GlGraphics,
    x: f64,
    y: f64,
    image : Image,
    spaceship : Texture,
    beam : Texture,
    keys: HashSet<Key>,
    stars: [Vec2d; STAR_COUNT],
    beams: Vec<Vec2d>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
    	let spaceship = &self.spaceship;
    	let image = &self.image;
        let (x, y) = (&self.x, &self.y);
        let draw_state = &DrawState::default();
        let stars = &self.stars;
        let beam = &self.beam;
        let beams = &self.beams;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            
		    for s in stars.iter() {
	            let transform = c.transform.trans(s[0], s[1]);
		        let square = rectangle::square(0.0, 0.0, 1.0);
		        rectangle(RED, square, transform, gl);
		    }
		    
		    for b in beams {
	            let trans = c.transform.trans(b[0], b[1]).trans(-45.0, -64.0);
	            image.draw(beam, draw_state, trans, gl);
		    }

            let transform = c.transform.trans(*x, *y).trans(-45.0, -64.0);
            image.draw(spaceship, draw_state, transform, gl);
        });
    }
    
    fn fire(&mut self) {
    	let beam = [self.x, self.y];
    	self.beams.push(beam);
    }

    fn update(&mut self, args: &UpdateArgs) {
    	if self.keys.contains(&Key::Left) {
    		self.x -= SPEED * args.dt;
    	}
    	if self.keys.contains(&Key::Right) {
    		self.x += SPEED * args.dt;
    	}

		for b in self.beams.iter_mut() { b[1] -= SPEED * args.dt; }
	    self.beams.retain(|b| b[1] > 0.0);

	    for s in self.stars.iter_mut() {
	    	s[1] += SPEED * args.dt;
	    	if s[1] > HEIGHT {
		    	s[0] = random::<f64>() * WIDTH;
		    	s[1] = 0.0; 
	    	}
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
		x: WIDTH / 2.0,
		y: HEIGHT - 64.0,
		image: Image::new().rect([0.0, 0.0, 90.0, 128.0]),
		spaceship: Texture::from_path(Path::new("assets/spaceship.png")).unwrap(),
		beam: Texture::from_path(Path::new("assets/beam.png")).unwrap(),
		keys: HashSet::new(),
		stars: [[0.0,0.0]; STAR_COUNT],
		beams: Vec::new()
    };
    for s in app.stars.iter_mut() {
    	s[0] = random::<f64>() * WIDTH;
    	s[1] = random::<f64>() * HEIGHT; 
    }

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
        	if key == Key::Space {
        		app.fire();
        	}
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