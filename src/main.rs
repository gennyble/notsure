mod physics;
mod thing;

use physics::{aabb_check, Intersection, LineSegment};
use smitten::{self, Color, SignedDistance, Smitten, Vec2, VirtualKeyCode};
use thing::Thing;

fn main() {
	let mut smitty = Smitten::new((720, 480), "Square", 36);

	let square = Thing::new((0, 0), (1, 1), smitty.make_texture("images/puare.png"));
	let mut us = Thing::new((-3.0, -3.0), (1, 1), Color::rgb(0.1, 0.3, 0.5));
	let speed = 0.075;

	loop {
		let _events = smitty.events();

		// Keyboard Control
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

		// Drawing

		smitty.clear();
		smitty.rect(square.center, square.size, square.draw);
		smitty.rect(us.center, us.size, us.draw);

		smitty.swap();
	}
}

fn sdf_seg(seg: LineSegment, thickness: u32, color: Color) -> SignedDistance {
	SignedDistance::LineSegment {
		start: seg.start,
		end: seg.end,
		thickness,
		color,
	}
}

fn do_full_collision(dynamic: &mut Thing, stuck: &Thing, movement: Vec2) -> bool {
	dynamic.offset(Vec2::new(0.025 * movement.x, 0.025 * movement.y));
	if !aabb_check(stuck, dynamic) {
		return false;
	}

	// We're colliding
	let previous = Thing::new(dynamic.previous_center, dynamic.size, dynamic.draw);

	let tr = LineSegment::new(previous.topright(), dynamic.topright());
	let tl = LineSegment::new(previous.topleft(), dynamic.topleft());
	let br = LineSegment::new(previous.bottomright(), dynamic.bottomright());
	let bl = LineSegment::new(previous.bottomleft(), dynamic.bottomleft());
	let sides = vec![tr, tl, br, bl];

	let mut interdist = vec![];

	// Compute intersections and distance to intersection
	for side in sides {
		for inter in stuck.intersect_segment(&side) {
			if let Intersection::Point(p) = inter.1 {
				let dist = ((p.x - side.start.x) * (p.x - side.start.x)
					+ (p.y - side.start.y) * (p.y - side.start.y))
					.sqrt();

				interdist.push((inter.0, inter.1, dist));
			}
		}
	}

	// Sort by distance
	interdist.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

	if let Some((side, Intersection::Point(mut p), dist)) = interdist.first() {
		match side {
			thing::Side::Top => dynamic.center.y = p.y + dynamic.half_size.y,
			thing::Side::Right => dynamic.center.x = p.x - dynamic.half_size.x,
			thing::Side::Bottom => dynamic.center.y = p.y - dynamic.half_size.y,
			thing::Side::Left => dynamic.center.x = p.x + dynamic.half_size.x,
		}
		println!("{:?}", p);
		//dynamic.put(p);
		return true;
	}
	false
}
