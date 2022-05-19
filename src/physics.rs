use smitten::Vec2;

pub trait AxisAlignedBoundingBox {
	fn bottom_left(&self) -> Vec2;
	fn top_right(&self) -> Vec2;
}

pub fn aabb_check<A, B>(a: &A, b: &B) -> bool
where
	A: AxisAlignedBoundingBox,
	B: AxisAlignedBoundingBox,
{
	let a_bl = a.bottom_left();
	let a_tr = a.top_right();

	let b_bl = b.bottom_left();
	let b_tr = b.top_right();

	if a_bl.x < b_tr.x && a_tr.x > b_bl.x {
		// Collide on X axis
		if a_bl.y < b_tr.y && a_tr.y > b_bl.y {
			// Collide on y axis
			true
		} else {
			false
		}
	} else {
		false
	}
}

struct LineSegment {
	start: Vec2,
	end: Vec2,

	// If the line is vertical, this will be f32::NAN
	slope: f32,
}

impl LineSegment {
	pub fn new(start: Vec2, end: Vec2) -> Self {
		LineSegment {
			start,
			end,
			slope: Self::slope(start, end),
		}
	}

	pub fn vertical(&self) -> bool {
		self.start.x == self.end.x
	}

	pub fn horizontal(&self) -> bool {
		self.start.y == self.end.y
	}

	#[inline]
	pub fn slope(start: Vec2, end: Vec2) -> f32 {
		let dx = end.x - start.x;

		if dx == 0.0 {
			return f32::NAN;
		}

		let dy = end.y - start.y;
		dy / dx
	}

	//TODO: gen- uh, what to do about floating point errors?
	// tolerance or something weirder? (compare deltas and do something about the sign?)
	pub fn parallel_to(&self, b: &LineSegment) -> bool {
		(self.vertical() && b.vertical()) || self.slope == b.slope
	}
}

#[cfg(test)]
mod test {
	use smitten::Vec2;

	use crate::physics::aabb_check;

	use super::{AxisAlignedBoundingBox, LineSegment};

	// A more traditional graphics model used here than the one in smitten
	struct Thing {
		center: Vec2,
		half_size: Vec2,
	}

	impl AxisAlignedBoundingBox for Thing {
		fn bottom_left(&self) -> Vec2 {
			self.center - self.half_size
		}

		fn top_right(&self) -> Vec2 {
			self.center + self.half_size
		}
	}

	#[test]
	fn intersection() {
		let a = Thing {
			center: Vec2::new(0.0, 0.0),
			half_size: Vec2::new(2.0, 2.0),
		};

		let b = Thing {
			center: Vec2::new(1.0, 1.0),
			half_size: Vec2::new(2.0, 2.0),
		};

		assert!(aabb_check(&a, &b))
	}

	// What do I name this test? It makes sure that the intersection algorithm
	// doesn't false positive if only X intersects
	#[test]
	fn x_axis_overlap_no_intersect() {
		let a = Thing {
			center: Vec2::new(0.0, 0.0),
			half_size: Vec2::new(2.0, 2.0),
		};

		let b = Thing {
			center: Vec2::new(1.0, 10.0),
			half_size: Vec2::new(2.0, 2.0),
		};

		assert!(!aabb_check(&a, &b))
	}

	#[test]
	fn y_axis_overlap_no_intersect() {
		let a = Thing {
			center: Vec2::new(0.0, 0.0),
			half_size: Vec2::new(2.0, 2.0),
		};

		let b = Thing {
			center: Vec2::new(10.0, 1.0),
			half_size: Vec2::new(2.0, 2.0),
		};

		assert!(!aabb_check(&a, &b))
	}

	#[test]
	fn slope_is_correct() {
		let a = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
		assert_eq!(a.slope, 1.0);

		let b = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, -1.0));
		assert_eq!(b.slope, -1.0);

		let c = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(3.0, 1.0));
		assert_eq!(c.slope, 1.0 / 3.0);
	}

	#[test]
	fn vertical_lines_are_parallel() {
		let a = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 10.0));
		let b = LineSegment::new(Vec2::new(1.0, 0.0), Vec2::new(1.0, 10.0));

		assert!(a.parallel_to(&b))
	}

	#[test]
	fn horizontal_lines_are_parallel() {
		let a = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
		let b = LineSegment::new(Vec2::new(0.0, 1.0), Vec2::new(10.0, 1.0));

		assert!(a.parallel_to(&b))
	}

	#[test]
	fn sloped_lines_are_parallel() {
		// Positive slope
		let a = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
		let b = LineSegment::new(Vec2::new(1.0, 0.0), Vec2::new(3.0, 2.0));

		assert!(a.parallel_to(&b));

		// Negative slope
		let c = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(-2.0, -2.0));
		let d = LineSegment::new(Vec2::new(-1.0, 0.0), Vec2::new(-3.0, -2.0));

		assert!(a.parallel_to(&b))
	}
}
