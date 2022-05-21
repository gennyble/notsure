mod physics;
mod thing;

use physics::{aabb_check, Intersection, LineSegment};
use smitten::{self, Color, SignedDistance, Smitten, Vec2, VirtualKeyCode};
use thing::Thing;

fn main() {
	let mut smitty = Smitten::new((720, 480), "Square", 36);

	let square = Thing::new((0, 0), (1, 1), smitty.make_texture("images/puare.png"));
	let mut us = Thing::new((-3.0, -3.0), (1, 1), Color::rgb(0.1, 0.3, 0.5));
	let mut intersecting = false;
	let speed = 0.075;

	let mut point_a = None;

	let mut dople = None;
	let mut active = None;
	let mut active_dir = Vec2::ZERO;
	let mut dofull = false;

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

		if smitty.is_key_down(VirtualKeyCode::Key1) {
			point_a = Some(us.center);
		}

		if smitty.is_key_down(VirtualKeyCode::Key2) {
			let mut clone = us;
			clone.draw = Color::rgba(0.1, 0.3, 0.5, 0.25).into();
			dople = Some(clone);
		}

		if dople.is_some() && smitty.is_key_down(VirtualKeyCode::Key3) {
			let mut clone = dople.unwrap();
			clone.draw = Color::rgb(0.6, 0.3, 0.2).into();
			active = Some(clone);
			active_dir = (us.center - dople.unwrap().center).normalize();
			dofull = true;
		}

		// AABB
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

		// Drawing

		smitty.clear();
		smitty.rect(square.center, square.size, square.draw);
		smitty.rect(us.center, us.size, us.draw);

		if let Some(active) = active.as_mut() {
			//if dofull {
			dofull = !do_full_collision(active, &square, active_dir);
			//}
			smitty.rect(active.center, active.size, active.draw);
		}

		if let Some(dople) = dople {
			smitty.rect(dople.center, dople.size, dople.draw);

			let tr = LineSegment::new(dople.topright(), us.topright());
			let tl = LineSegment::new(dople.topleft(), us.topleft());
			let br = LineSegment::new(dople.bottomright(), us.bottomright());
			let bl = LineSegment::new(dople.bottomleft(), us.bottomleft());

			let sides = vec![tr, tl, br, bl];
			let mut interdist = vec![];

			for side in sides {
				for inter in square.intersect_segment(&side) {
					if let Intersection::Point(p) = inter.1 {
						let dist = ((p.x - side.start.x) * (p.x - side.start.x)
							+ (p.y - side.start.y) * (p.y - side.start.y))
							.sqrt();

						interdist.push((inter, dist));
					}
				}
			}

			interdist.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

			let thickness = 2;
			let color = Color::rgba(0.1, 0.3, 0.5, 0.25);

			smitty.sdf(sdf_seg(tr, thickness, color));
			smitty.sdf(sdf_seg(tl, thickness, color));
			smitty.sdf(sdf_seg(br, thickness, color));
			smitty.sdf(sdf_seg(bl, thickness, color));

			for (idx, (i, dist)) in interdist.iter().enumerate() {
				if let Intersection::Point(p) = i.1 {
					smitty.sdf(SignedDistance::Circle {
						center: p,
						radius: 0.1,
						color: if idx == 0 { Color::GREEN } else { Color::WHITE },
					});
				}
			}
		}

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
				if let Intersection::Point(v) = i.1 {
					smitty.sdf(SignedDistance::Circle {
						center: v,
						radius: 0.075,
						color: c,
					});
				}
			}
		}

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
	println!("In");
	if !aabb_check(stuck, dynamic) {
		return false;
	}
	println!("through");
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
