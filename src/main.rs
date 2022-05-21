mod physics;

use physics::{AxisAlignedBoundingBox, Intersection, LineSegment};
use smitten::{self, Color, Draw, SignedDistance, Smitten, Vec2, VirtualKeyCode};

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

	pub fn offset<T: Into<Vec2>>(&mut self, offset: T) {
		self.previous_center = self.center;
		self.center += offset.into();
	}

	pub fn edge_intersections(&self, b: &Thing) -> Vec<bool> {
		// Top, right, bottom, left
		let mut ret = vec![false; 4];

		let b_edges = vec![b.top(), b.right(), b.bottom(), b.left()];

		ret[0] = b_edges.iter().any(|l| self.top().intersects_with(l));
		ret[1] = b_edges.iter().any(|l| self.right().intersects_with(l));
		ret[2] = b_edges.iter().any(|l| self.bottom().intersects_with(l));
		ret[3] = b_edges.iter().any(|l| self.left().intersects_with(l));

		ret
	}

	pub fn intersect_segment(&self, seg: &LineSegment) -> Vec<Intersection> {
		let b_edges = vec![self.top(), self.right(), self.bottom(), self.left()];

		b_edges
			.into_iter()
			.filter_map(|e| {
				if e.intersects_with(seg) {
					let intersect = e.calculate_intersection_point(seg);

					if let Intersection::Point(p) = intersect {
						println!("Intersect! Point {},{}", p.x, p.y);
					} else {
						println!("Intersect! Line!");
					}

					Some(intersect)
				} else {
					None
				}
			})
			.collect()
	}

	fn top(&self) -> LineSegment {
		LineSegment::new(
			(
				self.center.x - self.half_size.x,
				self.center.y + self.half_size.y,
			),
			(
				self.center.x + self.half_size.x,
				self.center.y + self.half_size.y,
			),
		)
	}

	fn right(&self) -> LineSegment {
		LineSegment::new(
			(
				self.center.x - self.half_size.x,
				self.center.y + self.half_size.y,
			),
			(
				self.center.x - self.half_size.x,
				self.center.y - self.half_size.y,
			),
		)
	}

	fn bottom(&self) -> LineSegment {
		LineSegment::new(
			(
				self.center.x - self.half_size.x,
				self.center.y - self.half_size.y,
			),
			(
				self.center.x + self.half_size.x,
				self.center.y - self.half_size.y,
			),
		)
	}

	fn left(&self) -> LineSegment {
		LineSegment::new(
			(
				self.center.x + self.half_size.x,
				self.center.y + self.half_size.y,
			),
			(
				self.center.x + self.half_size.x,
				self.center.y - self.half_size.y,
			),
		)
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
	let square2 = Thing::new((5, 5), (1, 1), Color::rgb(0.1, 0.6, 0.6));
	let square3 = Thing::new((-5, 5), (1, 1), Color::rgb(0.1, 0.6, 0.6));
	let square4 = Thing::new((-5, -5), (1, 1), Color::rgb(0.1, 0.6, 0.6));
	let square5 = Thing::new((5, -5), (1, 1), Color::rgb(0.1, 0.6, 0.6));
	let mut us = Thing::new((-3.0, -3.0), (1, 1), Color::rgb(0.1, 0.3, 0.5));
	let mut intersecting = false;
	let speed = 0.075;

	let mut sdf = vec![];
	let mut point_a = None;
	let mut point_b = None;
	let mut down_last = false;

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
		smitty.rect(square2.center, square2.size, square2.draw);
		smitty.rect(square3.center, square3.size, square3.draw);
		smitty.rect(square4.center, square4.size, square4.draw);
		smitty.rect(square5.center, square5.size, square5.draw);
		smitty.rect(us.center, us.size, us.draw);

		{
			let mut top = us.center;
			top.y += us.half_size.y;

			let mut right = us.center;
			right.x += us.half_size.x;

			let mut bottom = us.center;
			bottom.y -= us.half_size.y;

			let mut left = us.center;
			left.x -= us.half_size.x;

			let vertical = Vec2::new(3.0 / 36.0, us.size.y);
			let horizontal = Vec2::new(us.size.x, vertical.x);

			let edges = us
				.edge_intersections(&square)
				.into_iter()
				.zip(us.edge_intersections(&square2).into_iter())
				.zip(us.edge_intersections(&square3).into_iter())
				.zip(us.edge_intersections(&square4).into_iter())
				.zip(us.edge_intersections(&square5).into_iter())
				.map(|((((a, f), b), c), d)| {
					if a || b || c || d || f {
						Color::rgb(1.0, 0.0, 0.0)
					} else {
						Color::rgb(0.8, 0.8, 0.8)
					}
				})
				.collect::<Vec<Color>>();

			smitty.rect(top, horizontal, edges[0]);
			smitty.rect(left, vertical, edges[1]);
			smitty.rect(bottom, horizontal, edges[2]);
			smitty.rect(right, vertical, edges[3]);
		}

		if let Some(p) = point_a {
			smitty.sdf(SignedDistance::Circle {
				center: p,
				radius: 0.2,
				color: Color::rgb(0.6, 0.6, 0.2),
			});
		}

		if let Some(p) = point_b {
			smitty.sdf(SignedDistance::Circle {
				center: p,
				radius: 0.2,
				color: Color::rgb(0.2, 0.6, 0.2),
			});
		}

		if !down_last && smitty.is_key_down(VirtualKeyCode::T) {
			down_last = true;
			if let (Some(start), Some(end)) = (point_a, point_b) {
				let end = us.center;

				let sdfseg = SignedDistance::LineSegment {
					start,
					end,
					thickness: 2,
					color: Color::rgb(0.6, 0.1, 0.8),
				};
				sdf.push(sdfseg);

				let seg = LineSegment::new(start, end);
				let is = square.intersect_segment(&seg);

				let colors = vec![
					Color::rgb(1.0, 0.0, 0.0),
					Color::rgb(0.0, 1.0, 0.0),
					Color::rgb(0.0, 0.0, 1.0),
					Color::rgb(1.0, 1.0, 0.0),
					Color::rgb(1.0, 0.0, 1.0),
					Color::rgb(0.0, 1.0, 1.0),
				];

				for (i, &c) in is.iter().zip(colors.iter()) {
					if let Intersection::Point(v) = i {
						println!(
							"INTERSECTION POINT: ({},{}) color ({}, {}, {})",
							v.x, v.y, c.r, c.g, c.b
						);

						sdf.push(SignedDistance::Circle {
							center: *v,
							radius: 0.075,
							color: c,
						});
					}
				}
			}
		}

		if smitty.is_key_down(VirtualKeyCode::R) {
			down_last = false;
		}

		for sdf in &sdf {
			smitty.sdf(*sdf);
		}

		smitty.swap();
	}
}
