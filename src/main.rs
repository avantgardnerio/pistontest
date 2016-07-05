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

mod sprite;
use sprite::{Sprite, Drawable};

const GREEN: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SPEED: f64 = 200.0;
const STAR_COUNT: usize = 100;
const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 768.0;
const OPENGL: OpenGL = OpenGL::V3_2;


pub struct Assets {
	spaceship: Drawable,
	beam: Drawable,
	lutetia: Drawable
}

pub struct AppState<'a> {
	assets: &'a Assets,
    spaceship: Sprite<'a>,
    keys: HashSet<Key>,
    stars: [Vec2d; STAR_COUNT],
    beams: Vec<Sprite<'a>>,
    asteriods: Vec<Sprite<'a>>,
    asteriod_interval: f64,
    total_time: f64,
    last_asteriod: f64
}

pub struct App<'a> {
    gl: GlGraphics,
    state: AppState<'a>
}

impl Assets {
	pub fn new() -> Self {
		Assets {
			spaceship: Drawable::new(Texture::from_path(Path::new("assets/spaceship.png")).unwrap()),
			beam: Drawable::new(Texture::from_path(Path::new("assets/beam.png")).unwrap()),
			lutetia: Drawable::new(Texture::from_path(Path::new("assets/lutetia.jpg")).unwrap())
		}
	}
}

impl <'a>App<'a> {
    pub fn new(assets: &'a Assets) -> Self {		
        let mut this = App {
			gl: GlGraphics::new(OPENGL),
			state: AppState {
				assets: assets,
				spaceship: Sprite {
					drawable: &assets.spaceship,
					position: [WIDTH / 2.0, HEIGHT - assets.spaceship.get_width()/2.0],
				},
				keys: HashSet::new(),
				stars: [[0.0,0.0]; STAR_COUNT],
				beams: Vec::new(),
				asteriods: Vec::new(),
				asteriod_interval: 3.0,
				total_time: 0.0,
				last_asteriod: 0.0
			}
		};

		for s in this.state.stars.iter_mut() {
			s[0] = random::<f64>() * WIDTH;
			s[1] = random::<f64>() * HEIGHT; 
		}

        return this;
    }

    fn render(&mut self, args: &RenderArgs) {
        let draw_state = &DrawState::default();
    	let state = &self.state;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            
            // Stars
		    for s in state.stars.iter() {
	            let transform = c.transform.trans(s[0], s[1]);
		        let square = rectangle::square(0.0, 0.0, 1.0);
		        rectangle(RED, square, transform, gl);
		    }

			// Beams		    
		    for b in state.beams.iter() {
		    	b.draw(draw_state, c, gl);
		    }

			// Asteriods
		    for b in state.asteriods.iter() { 
		    	b.draw(draw_state, c, gl);
		    }

			// Ship
            state.spaceship.draw(draw_state, c, gl);
        });
    }
    
    fn fire(&mut self) {
		let beam = Sprite {
			drawable: &self.state.assets.beam,
			position: self.state.spaceship.position
		};
    	self.state.beams.push(beam);
    }

    fn update(&mut self, args: &UpdateArgs) {
    	let state = &mut self.state;
    	
    	// Move the ship
    	if state.keys.contains(&Key::Left) {
    		state.spaceship.position[0] -= SPEED * args.dt;
    	}
    	if state.keys.contains(&Key::Right) {
    		state.spaceship.position[0] += SPEED * args.dt;
    	}

		// Move laser beams
		for b in state.beams.iter_mut() { 
			b.position[1] -= SPEED * args.dt; 
		}
	    state.beams.retain(|b| b.position[1] > 0.0);

		// Move the stars
	    for s in state.stars.iter_mut() {
	    	s[1] += SPEED * args.dt;
	    	if s[1] > HEIGHT {
		    	s[0] = random::<f64>() * WIDTH;
		    	s[1] = 0.0; 
	    	}
	    }

		// Move the asteriods	    
		for b in state.asteriods.iter_mut() { 
			b.position[1] += SPEED * args.dt; 
		}
	    state.asteriods.retain(|b| b.position[1] < HEIGHT);
	    if state.last_asteriod < state.total_time - state.asteriod_interval {
	    	state.last_asteriod = state.total_time;
	    	let asteriod = Sprite {
				drawable: &self.state.assets.lutetia,
	    		position: [random::<f64>() * WIDTH, 0.0]
	    	};
	    	state.asteriods.push(asteriod);
	    }

		// Move on
	    state.total_time += args.dt;
    }
}

fn main() {
    let mut window: Window = WindowSettings::new(
        "Asteriods!",
        [1024, 768]
    )
    .opengl(OPENGL)
    .exit_on_esc(true)
    .build()
    .unwrap();
    
    let assets = Assets::new();
    let mut app: App = App::new(&assets);
    
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