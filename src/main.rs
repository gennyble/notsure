mod physics;

use smitten::{self, Color, Draw, Smitten, Vec2, VirtualKeyCode};

struct Thing {
	center: Vec2,
	size: Vec2,
	half_size: Vec2,
	draw: Draw,
}

impl Thing {
	pub fn new<C: Into<Vec2>, S: Into<Vec2>, D: Into<Draw>>(center: C, size: S, draw: D) -> Self {
		let size = size.into();

		Self {
			center: center.into(),
			size,
			half_size: size / 2,
			draw: draw.into(),
		}
	}
}

impl physics::AxisAlignedBoundingBox for Thing {
	fn bottom_left(&self) -> Vec2 {
		self.center - self.half_size
	}

	fn top_right(&self) -> Vec2 {
		self.center + self.half_size
	}
}

fn main() {
	let mut smitty = Smitten::new((720, 480), "Square", 36);

	let square = Thing::new((0, 0), (1, 1), smitty.make_texture("images/puare.png"));
	let mut us = Thing::new((-3.0, -3.0), (1, 1), Color::rgb(0.1, 0.3, 0.5));
	let mut intersecting = false;
	let speed = 0.075;

	loop {
		let _events = smitty.events();

		if smitty.is_key_down(VirtualKeyCode::Escape) {
			break;
		}

		if smitty.is_key_down(VirtualKeyCode::W) {
			us.center.y += speed;
		} else if smitty.is_key_down(VirtualKeyCode::S) {
			us.center.y -= speed;
		}

		if smitty.is_key_down(VirtualKeyCode::A) {
			us.center.x -= speed;
		} else if smitty.is_key_down(VirtualKeyCode::D) {
			us.center.x += speed;
		}

		match physics::aabb_check(&us, &square) {
			true if !intersecting => {
				us.draw = Color::rgb(0.5, 0.1, 0.2).into();
				intersecting = true;
			}
			false if intersecting => {
				us.draw = Color::rgb(0.1, 0.3, 0.5).into();
				intersecting = false;
			}
			_ => (),
		}

		smitty.clear();
		smitty.rect(square.center, square.size, square.draw);
		smitty.rect(us.center, us.size, us.draw);
		smitty.swap();
	}
}
