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

#[cfg(test)]
mod test {
	use smitten::Vec2;

	use crate::physics::aabb_check;

	use super::AxisAlignedBoundingBox;

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
}
