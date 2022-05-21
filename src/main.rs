mod physics;
mod thing;

use physics::{Intersection, LineSegment};
use smitten::{self, Color, SignedDistance, Smitten, VirtualKeyCode};
use thing::Thing;

fn main() {
	let mut smitty = Smitten::new((720, 480), "Square", 36);

	let square = Thing::new((0, 0), (1, 1), smitty.make_texture("images/puare.png"));
	let mut us = Thing::new((-3.0, -3.0), (1, 1), Color::rgb(0.1, 0.3, 0.5));
	let mut intersecting = false;
	let speed = 0.075;

	let mut point_a = None;

	loop {
		let _events = smitty.events();

		if smitty.is_key_down(VirtualKeyCode::Escape) {
			break;
		}

		if smitty.is_key_down(VirtualKeyCode::W) {
			us.offset((0.0, speed));
		} else if smitty.is_key_down(VirtualKeyCode::S) {
			us.offset((0.0, -speed));
		}

		if smitty.is_key_down(VirtualKeyCode::A) {
			us.offset((-speed, 0.0));
		} else if smitty.is_key_down(VirtualKeyCode::D) {
			us.offset((speed, 0.0));
		}

		if smitty.is_key_down(VirtualKeyCode::Key1) {
			point_a = Some(us.center);
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
			smitty.sdf(SignedDistance::Circle {
				center: p,
				radius: 0.2,
				color: Color::rgb(0.6, 0.6, 0.2),
			});
		}

		if let Some(start) = point_a {
			let end = us.center;

			smitty.sdf(SignedDistance::LineSegment {
				start,
				end,
				thickness: 2,
				color: Color::rgb(0.6, 0.1, 0.8),
			});

			let seg = LineSegment::new(start, end);
			let is = square.intersect_segment(&seg);

			let colors = vec![
				Color::RED,
				Color::GREEN,
				Color::BLUE,
				Color::YELLOW,
				Color::FUCHSIA,
				Color::AQUA,
			];

			for (i, &c) in is.iter().zip(colors.iter()) {
				if let Intersection::Point(v) = i {
					smitty.sdf(SignedDistance::Circle {
						center: *v,
						radius: 0.075,
						color: c,
					});
				}
			}
		}

		smitty.swap();
	}
}
