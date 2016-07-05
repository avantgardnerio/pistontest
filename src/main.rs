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

const GREEN: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SPEED: f64 = 200.0;
const STAR_COUNT: usize = 100;
const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 768.0;
const OPENGL: OpenGL = OpenGL::V3_2;

pub struct Sprite<'a> {
	position: Vec2d,
	texture: &'a Texture,
	image: &'a Image
}

impl <'a>Sprite<'a> {
    pub fn draw(
        &self,
        draw_state: &DrawState,
        c: Context,
        g: &mut GlGraphics
    ) {
    	let rect = self.image.rectangle.unwrap();
		let transform = c.transform
			.trans(self.position[0], self.position[1])
			.trans(-rect[2]as f64/2.0, -rect[3]as f64/2.0);
        self.image.draw(self.texture, draw_state, transform, g);
    }
}

pub struct Assets {
	spaceship_texture: Texture,
	spaceship_image: Image,
	beam_texture: Texture,
	beam_image: Image,
	lutetia_texture: Texture,
	lutetia_image: Image
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
			spaceship_image: Image::new().rect([0.0, 0.0, 90.0, 128.0]),
			spaceship_texture: Texture::from_path(Path::new("assets/spaceship.png")).unwrap(),

			beam_texture: Texture::from_path(Path::new("assets/beam.png")).unwrap(),
			beam_image: Image::new().rect([0.0, 0.0, 49.0, 98.0]),

			lutetia_texture: Texture::from_path(Path::new("assets/lutetia.jpg")).unwrap(),
			lutetia_image: Image::new().rect([0.0, 0.0, 107., 128.0])
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
					image: &assets.spaceship_image,
					texture: &assets.spaceship_texture,
					position: [WIDTH / 2.0, HEIGHT - 64.0],
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
            
		    for s in state.stars.iter() {
	            let transform = c.transform.trans(s[0], s[1]);
		        let square = rectangle::square(0.0, 0.0, 1.0);
		        rectangle(RED, square, transform, gl);
		    }
		    
		    for b in state.beams.iter() {
		    	b.draw(draw_state, c, gl);
		    }

		    for b in state.asteriods.iter() { 
		    	b.draw(draw_state, c, gl);
		    }

            state.spaceship.draw(draw_state, c, gl);
        });
    }
    
    fn fire(&mut self) {
		let beam = Sprite {
			image: &self.state.assets.beam_image,
			texture: &self.state.assets.beam_texture,
			position: self.state.spaceship.position
		};
    	self.state.beams.push(beam);
    }

    fn update(&mut self, args: &UpdateArgs) {
    	let state = &mut self.state;
    	if state.keys.contains(&Key::Left) {
    		state.spaceship.position[0] -= SPEED * args.dt;
    	}
    	if state.keys.contains(&Key::Right) {
    		state.spaceship.position[0] += SPEED * args.dt;
    	}

		for b in state.beams.iter_mut() { b.position[1] -= SPEED * args.dt; }
	    state.beams.retain(|b| b.position[1] > 0.0);

	    for s in state.stars.iter_mut() {
	    	s[1] += SPEED * args.dt;
	    	if s[1] > HEIGHT {
		    	s[0] = random::<f64>() * WIDTH;
		    	s[1] = 0.0; 
	    	}
	    }
	    
		for b in state.asteriods.iter_mut() { 
			b.position[1] += SPEED * args.dt; 
		}
	    state.asteriods.retain(|b| b.position[1] < HEIGHT);

	    if state.last_asteriod < state.total_time - state.asteriod_interval {
	    	state.last_asteriod = state.total_time;
	    	let asteriod = Sprite {
				image: &self.state.assets.lutetia_image,
				texture: &self.state.assets.lutetia_texture,
	    		position: [random::<f64>() * WIDTH, 0.0]
	    	};
	    	state.asteriods.push(asteriod);
	    }

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