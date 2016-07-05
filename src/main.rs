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

pub struct Sprite<'a> {
	position: Vec2d,
	texture: &'a Texture,
	image: &'a Image
}

pub struct AppState<'a> {
    image : Image,
    spaceship: Sprite<'a>,
    beam : Texture,
    lutetia : Texture,
    keys: HashSet<Key>,
    stars: [Vec2d; STAR_COUNT],
    beams: Vec<Vec2d>,
    asteriods: Vec<Vec2d>,
    asteriod_interval: f64,
    total_time: f64,
    last_asteriod: f64
}

pub struct App<'a> {
    gl: GlGraphics,
    state: AppState<'a>
}

impl <'a>App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        let draw_state = &DrawState::default();
    	let state = &self.state;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            
		    for s in state.stars.iter() {
	            let transform = c.transform.trans(s[0], s[1]);
		        let square = rectangle::square(0.0, 0.0, 1.0);
		        rectangle(RED, square, transform, gl);
		    }
		    
		    for b in state.beams.iter() {
	            let trans = c.transform.trans(b[0], b[1]).trans(-45.0, -64.0);
	            state.image.draw(&state.beam, draw_state, trans, gl);
		    }

		    for b in state.asteriods.iter() {
	            let trans = c.transform.trans(b[0], b[1]).trans(-45.0, -64.0);
	            state.image.draw(&state.lutetia, draw_state, trans, gl);
		    }

            let transform = c.transform.trans(state.spaceship.position[0], state.spaceship.position[1]).trans(-45.0, -64.0);
            state.spaceship.image.draw(state.spaceship.texture, draw_state, transform, gl);
        });
    }
    
    fn fire(&mut self) {
    	self.state.beams.push(self.state.spaceship.position);
    }

    fn update(&mut self, args: &UpdateArgs) {
    	let state = &mut self.state;
    	if state.keys.contains(&Key::Left) {
    		state.spaceship.position[0] -= SPEED * args.dt;
    	}
    	if state.keys.contains(&Key::Right) {
    		state.spaceship.position[0] += SPEED * args.dt;
    	}

		for b in state.beams.iter_mut() { b[1] -= SPEED * args.dt; }
	    state.beams.retain(|b| b[1] > 0.0);

	    for s in state.stars.iter_mut() {
	    	s[1] += SPEED * args.dt;
	    	if s[1] > HEIGHT {
		    	s[0] = random::<f64>() * WIDTH;
		    	s[1] = 0.0; 
	    	}
	    }
	    
		for b in state.asteriods.iter_mut() { b[1] += SPEED * args.dt; }
	    state.asteriods.retain(|b| b[1] < HEIGHT);

	    if state.last_asteriod < state.total_time - state.asteriod_interval {
	    	state.last_asteriod = state.total_time;
	    	let asteriod = [random::<f64>() * WIDTH, 0.0];
	    	state.asteriods.push(asteriod);
	    }

	    state.total_time += args.dt;
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
    
    let spaceship_texture = Texture::from_path(Path::new("assets/spaceship.png")).unwrap();
    let spaceship_image = Image::new().rect([0.0, 0.0, 90.0, 128.0]);
    
    let mut app = App {
		gl: GlGraphics::new(opengl),
		state: AppState {
			image: Image::new().rect([0.0, 0.0, 90.0, 128.0]),
			spaceship: Sprite {
				image: &spaceship_image,
				texture: &spaceship_texture,
				position: [WIDTH / 2.0, HEIGHT - 64.0],
			},
			beam: Texture::from_path(Path::new("assets/beam.png")).unwrap(),
			lutetia: Texture::from_path(Path::new("assets/lutetia.jpg")).unwrap(),
			keys: HashSet::new(),
			stars: [[0.0,0.0]; STAR_COUNT],
			beams: Vec::new(),
			asteriods: Vec::new(),
			asteriod_interval: 3.0,
			total_time: 0.0,
			last_asteriod: 0.0
		}
    };
    for s in app.state.stars.iter_mut() {
    	s[0] = random::<f64>() * WIDTH;
    	s[1] = random::<f64>() * HEIGHT; 
    }

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
        	if key == Key::Space {
        		app.fire();
        	}
        	app.state.keys.insert(key);
        }
        
        if let Some(Button::Keyboard(key)) = e.release_args() {
        	app.state.keys.remove(&key);
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}