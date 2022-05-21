use smitten::{Draw, Vec2};

use crate::physics::{self, Intersection, LineSegment};

pub struct Thing {
	pub center: Vec2,
	previous_center: Vec2,

	pub size: Vec2,
	half_size: Vec2,
	pub draw: Draw,
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
