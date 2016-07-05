extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use graphics::*;

use opengl_graphics::{GlGraphics, Texture};

use graphics::math::Vec2d;

pub struct Drawable {
	texture: Texture,
	image: Image
}

impl Drawable {
	pub fn new(texture: Texture) -> Self {
		let width = texture.get_width() as f64;
		let height = texture.get_height() as f64;
		Drawable {
			texture: texture,
			image: Image::new().rect([0.0, 0.0, width, height]),
		}
	}
	
	pub fn get_width(&self) -> f64 {
		return self.texture.get_width() as f64;
	}
	
	pub fn get_height(&self) -> f64 {
		return self.texture.get_height() as f64;
	}
}

pub struct Sprite<'a> {
	pub position: Vec2d,
	pub drawable: &'a Drawable
}

impl <'a>Sprite<'a> {
    pub fn draw(&self, draw_state: &DrawState, c: Context, g: &mut GlGraphics) {
		let transform = c.transform
			.trans(self.position[0], self.position[1])
			.trans(-self.drawable.get_width()/2.0, -self.drawable.get_height()/2.0);
        self.drawable.image.draw(&self.drawable.texture, draw_state, transform, g);
    }
}
