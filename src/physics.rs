use std::fmt;

use smitten::Vec2;

const TOLERANCE: f32 = 0.00001;

pub trait AxisAlignedBoundingBox {
	fn bottom_left(&self) -> Vec2;
	fn top_right(&self) -> Vec2;
	fn previous_bottom_left(&self) -> Vec2;
	fn previous_top_rght(&self) -> Vec2;
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

#[derive(Copy, Clone, Debug)]
pub struct LineSegment {
	start: Vec2,
	end: Vec2,

	// If the line is vertical, this will be f32::NAN
	slope: f32,
}

impl LineSegment {
	pub fn new<S: Into<Vec2>, E: Into<Vec2>>(start: S, end: E) -> Self {
		let start = start.into();
		let end = end.into();

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

	pub fn swap_points(&mut self) {
		//TODO: gen- Listen to clippy the almighty
		let tmp = self.start;
		self.start = self.end;
		self.end = tmp;
	}

	#[inline]
	fn slope(start: Vec2, end: Vec2) -> f32 {
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

	pub fn intersects_with(&self, b: &LineSegment) -> bool {
		self.bounds().bounding_box_collides_with(&b.bounds())
			&& self.touches_or_crosses(b)
			&& b.touches_or_crosses(self)
	}

	fn bounding_box_collides_with(&self, b: &LineSegment) -> bool {
		self.start.x <= b.end.x
			&& self.end.x >= b.start.x
			&& self.start.y <= b.end.y
			&& self.end.y >= b.start.y
	}

	//TODO: gen- Please don't reuse the segment like this oh my god
	fn bounds(&self) -> LineSegment {
		let mut start = Vec2::ZERO;
		let mut end = Vec2::ZERO;

		if self.start.x < self.end.x {
			start.x = self.start.x;
			end.x = self.end.x;
		} else {
			start.x = self.end.x;
			end.x = self.start.x;
		}

		if self.start.y < self.end.y {
			start.y = self.start.y;
			end.y = self.end.y;
		} else {
			start.y = self.end.y;
			end.y = self.start.y;
		}

		Self {
			start,
			end,
			slope: 0.0,
		}
	}

	fn point_cross_product(a: Vec2, b: Vec2) -> f32 {
		a.x * b.y - b.x * a.y
	}

	fn has_point(&self, mut point: Vec2) -> bool {
		let shifted_endpoint = self.end - self.start;
		point -= self.start;
		Self::point_cross_product(shifted_endpoint, point).abs() < TOLERANCE
	}

	fn left_of_point(&self, mut p: Vec2) -> bool {
		let tmp = self.end - self.start;
		p -= self.start;
		Self::point_cross_product(tmp, p) < 0.0
	}

	fn touches_or_crosses(&self, b: &LineSegment) -> bool {
		self.has_point(b.start)
			|| self.has_point(b.end)
			|| (self.left_of_point(b.start) ^ self.left_of_point(b.end))
	}

	// Comments copied directly from Martin Thoma's code in which this function is based
	// Look for `getIntersection` on this page:
	// https://martin-thoma.com/how-to-check-if-two-line-segments-intersect
	pub fn calculate_intersection_point(&self, b: &LineSegment) -> Intersection {
		let mut a = *self;
		let mut b = *b;

		if a.vertical() {
			// Case (A)
			// As a is a perfect vertical line, it cannot be represented
			// nicely in a mathematical way. But we directly know that

			let mut start = Vec2::new(a.start.x, 0.0);
			let mut end = start;

			if b.vertical() {
				// Case (AA): Both lines vertical and, since we know they
				// collide, we know all X are the same

				// Normalize A and B so that their start is before their end
				if a.start.y > a.end.y {
					a.swap_points();
				}
				if b.start.y > b.end.y {
					b.swap_points();
				}

				// Make sure A is lower than B
				if a.start.y > b.start.y {
					let tmp = a;
					a = b;
					b = tmp;
				}

				// Now we know that the y-value of a["first"] is the
				// lowest of all 4 y values
				// this means, we are either in case (AAA):
				//   a: x--------------x
				//   b:    x---------------x
				// or in case (AAB)
				//   a: x--------------x
				//   b:    x-------x
				// in both cases:
				// get the relavant y intervall
				start.y = b.start.y;
				end.y = a.end.y.min(b.end.y);

				Intersection::Line(LineSegment::new(start, end))
			} else {
				// Case (AB)
				// we can mathematically represent line b as
				//     y = m*x + t <=> t = y - m*x
				// m = (y1-y2)/(x1-x2)

				//TODO: gen- Is this slope equation not backwards??
				let m = (b.start.y - b.end.y) / (b.start.x - b.end.x);
				let t = b.start.y - m * b.start.x;

				start.y = m * start.x + t;

				Intersection::Point(start)
			}
		} else if b.vertical() {
			// Case (B)
			// essentially the same as Case (AB), but with
			// a and b switched

			let mut start = Vec2::new(a.start.x, 0.0);
			let mut end = start;

			let tmp = a;
			a = b;
			b = tmp;

			let m = (b.start.y - b.end.y) / (b.start.x - b.end.x);
			let t = b.start.y - m * b.start.x;

			start.y = m * start.x + t;

			Intersection::Point(start)
		} else {
			// Case (C)
			// Both lines can be represented mathematically
			let ma = (a.start.y - a.end.y) / (a.start.x - a.end.x);
			let mb = (b.start.y - b.end.y) / (b.start.x - b.end.x);
			let ta = a.start.y - ma * a.start.x;
			let tb = b.start.y - mb * b.start.x;

			//TODO: gen- Should we have a tolerance on this because of FPE?
			if ma == mb {
				// Normalize A and B so that their start is before their end
				if a.start.y > a.end.y {
					a.swap_points();
				}
				if b.start.y > b.end.y {
					b.swap_points();
				}

				// Make sure A is lower than B
				if a.start.y > b.start.y {
					let tmp = a;
					a = b;
					b = tmp;
				}

				let start = Vec2::new(b.start.x, ma * b.start.x + ta);
				let end = Vec2::new(a.end.x.min(b.end.x), ma * a.end.x.min(b.end.x) + ta);

				Intersection::Line(LineSegment::new(start, end))
			} else {
				let x1 = (tb - ta) / (ma - mb);
				Intersection::Point(Vec2::new(x1, ma * x1 + ta))
			}
		}
	}
}

impl fmt::Display for LineSegment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"(({},{}) | ({},{}))",
			self.start.x, self.start.y, self.end.x, self.end.y
		)
	}
}

#[derive(Clone, Debug)]
pub enum Intersection {
	Line(LineSegment),
	Point(Vec2),
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

		// We don't need these two yet
		fn previous_bottom_left(&self) -> Vec2 {
			todo!()
		}

		fn previous_top_rght(&self) -> Vec2 {
			todo!()
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

	#[test]
	fn segment_has_point() {
		let a = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));

		assert!(a.has_point(Vec2::new(1.0, 1.0)))
	}
}

/// These are the test cases from the below link where the intersection code was
/// ported from
/// https://martin-thoma.com/how-to-check-if-two-line-segments-intersect
#[cfg(test)]
mod martin {
	use smitten::Vec2;

	use super::LineSegment;

	macro_rules! make_case {
		(($a1x:literal, $a1y:literal), ($a2x:literal, $a2y:literal), ($b1x:literal, $b1y:literal), ($b2x:literal, $b2y:literal), ($ix:literal, $iy:literal)) => {
			Case {
				a: LineSegment {
					start: Vec2 {
						x: $a1x as f32,
						y: $a1y as f32,
					},
					end: Vec2 {
						x: $a2x as f32,
						y: $a2y as f32,
					},
					slope: 0.0,
				},
				b: LineSegment {
					start: Vec2 {
						x: $b1x as f32,
						y: $b1y as f32,
					},
					end: Vec2 {
						x: $b2x as f32,
						y: $b2y as f32,
					},
					slope: 0.0,
				},
				intersection: Some(Vec2 {
					x: $ix as f32,
					y: $iy as f32,
				}),
			}
		};
		(($a1x:literal, $a1y:literal), ($a2x:literal, $a2y:literal), ($b1x:literal, $b1y:literal), ($b2x:literal, $b2y:literal)) => {
			Case {
				a: LineSegment {
					start: Vec2 {
						x: $a1x as f32,
						y: $a1y as f32,
					},
					end: Vec2 {
						x: $a2x as f32,
						y: $a2y as f32,
					},
					slope: 0.0,
				},
				b: LineSegment {
					start: Vec2 {
						x: $b1x as f32,
						y: $b1y as f32,
					},
					end: Vec2 {
						x: $b2x as f32,
						y: $b2y as f32,
					},
					slope: 0.0,
				},
				intersection: None,
			}
		};
	}

	pub struct Case {
		a: LineSegment,
		b: LineSegment,
		intersection: Option<Vec2>,
	}

	impl Case {
		pub fn run_success(&self) -> bool {
			self.a.intersects_with(&self.b)
		}

		pub fn run_fail(&self) -> bool {
			!self.a.intersects_with(&self.b)
		}
	}

	//TODO: gen- Correct these. Some of the intersections here are lines.
	//change the type in Case to Intersection and add another macro rule
	// for six points total.

	// Axis lines. Perpendicular and intersecting in the middle.
	#[rustfmt::skip]
	const T1: Case = make_case!(
		(-5, 0), (5, 0),
		(0, 5), (0, -5),
		(0, 0)
	);

	// Non-perpendicular lines with one of the endpoints on the other
	#[rustfmt::skip]
	const T2: Case = make_case!(
		(0, 0), (2, 2),
		(1, 1), (4, 3),
		(1, 1)
	);

	// Perpendicular lines, one endpoint on a line.
	#[rustfmt::skip]
	const T3: Case = make_case!(
		(-2, 0), (0, 0),
		(-2, -2), (-2, 2),
		(-2, 0)
	);

	// Same as T3, but in the +,+ quadrant and no lines coindicent with the origin
	#[rustfmt::skip]
	const T4: Case = make_case!(
		(4, 0), (4, 8),
		(0, 4), (4, 4),
		(4, 4)
	);

	// Coincident lines. B is inside A
	#[rustfmt::skip]
	const T5: Case = make_case!(
		(0, 0), (10, 10),
		(2, 2), (7, 7),
		(2, 2)
	);

	// Literally the same line
	#[rustfmt::skip]
	const T6: Case = make_case!(
		(6, -3), (-5, -1),
		(6, -3), (-5, -1),
		(6, -3)
	);

	// Parallel, X and Y projections collide but segments do not.
	#[rustfmt::skip]
	const F1: Case = make_case!(
		(2, 2), (10, 10),
		(4, 5), (8, 9)
	);

	// Parallel, Y projections collide. X touches at one point.
	#[rustfmt::skip]
	const F2: Case = make_case!(
		(0, 0), (-7, 7),
		(-7, 1), (-9, 3)
	);

	// Parallel, segments do not overlap
	#[rustfmt::skip]
	const F3: Case = make_case!(
		(0, 0), (0, 2),
		(5, 5), (5, 7)
	);

	// Perpendicular, segments do not overlap
	#[rustfmt::skip]
	const F4: Case = make_case!(
		(0, 0), (0, 2),
		(5, 5), (7, 5)
	);

	// Same line, but different segments.
	#[rustfmt::skip]
	const F5: Case = make_case!(
		(-5, -5), (2, 2),
		(6, 6), (10, 10)
	);

	// Less than a unit not touching
	#[rustfmt::skip]
	const F6: Case = make_case!(
		(5, 0), (1, 5),
		(0, 0), (2, 2)
	);

	// Parallel horiz., one above the other
	#[rustfmt::skip]
	const F7: Case = make_case!(
		(-5, 2), (5, 2),
		(-1, 5), (1, 5)
	);

	#[rustfmt::skip]
	const F8: Case = make_case!(
		(10, 0), (0, 10),
		(5, 2), (5, 4)
	);

	#[rustfmt::skip]
	const F9: Case = make_case!(
		(-10, 0), (0, 10),
		(-5, 2), (-5, 4)
	);

	#[rustfmt::skip]
	const F10: Case = make_case!(
		(10, 0), (10, 10),
		(5, 2), (5, 4)
	);

	#[test]
	fn segments_intersect() {
		assert!(T1.run_success());
		assert!(T2.run_success());
		assert!(T3.run_success());
		assert!(T4.run_success());
		assert!(T5.run_success());
		assert!(T6.run_success());
	}

	#[test]
	fn segments_do_not_intersect() {
		assert!(!F1.run_success());
		assert!(!F2.run_success());
		assert!(!F3.run_success());
		assert!(!F4.run_success());
		assert!(!F5.run_success());
		assert!(!F6.run_success());
		assert!(!F7.run_success());
		assert!(!F8.run_success());
	}

	#[test]
	fn f9() {
		assert!(!F9.run_success());
	}

	#[test]
	fn f10() {
		assert!(!F10.run_success());
	}
}
