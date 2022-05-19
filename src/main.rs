mod physics;

use smitten::{self, Color, Draw, Smitten, Vec2, VirtualKeyCode};

pub struct Thing {
	center: Vec2,
	previous_center: Vec2,

	size: Vec2,
	half_size: Vec2,
	draw: Draw,
}

impl Thing {
	pub fn new<C: Into<Vec2>, S: Into<Vec2>, D: Into<Draw>>(center: C, size: S, draw: D) -> Self {
		let size = size.into();
		let center = center.into();

		Self {
			center,
			previous_center: center,

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

	fn previous_bottom_left(&self) -> Vec2 {
		self.previous_center - self.half_size
	}

	fn previous_top_rght(&self) -> Vec2 {
		self.previous_center + self.half_size
	}
}

fn main() {
	let mut smitty = Smitten::new((720, 480), "Square", 36);

	let square = Thing::new((0, 0), (1, 1), smitty.make_texture("images/puare.png"));
	let mut us = Thing::new((-3.0, -3.0), (1, 1), Color::rgb(0.1, 0.3, 0.5));
	let mut intersecting = false;
	let speed = 0.075;

	let mut point_a = None;
	let mut point_b = None;

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

		if smitty.is_key_down(VirtualKeyCode::Key1) {
			point_a = Some(us.center);
		}

		if smitty.is_key_down(VirtualKeyCode::Key2) {
			point_b = Some(us.center);
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

		if let Some(p) = point_a {
			smitty.rect(p, (0.25, 0.25), Color::rgb(0.6, 0.6, 0.2))
		}

		if let Some(p) = point_b {
			smitty.rect(p, (0.25, 0.25), Color::rgb(0.2, 0.6, 0.2))
		}

		smitty.swap();
	}
}
