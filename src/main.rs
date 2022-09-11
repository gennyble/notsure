mod physics;
mod thing;

use physics::{aabb_check, Intersection, LineSegment};
use smitten::{self, Color, Key, SignedDistance, Smitten, Vec2};
use thing::Thing;

fn main() {
	let mut smitty = Smitten::new((720, 480), "Square", 36);
	smitty.texture_coloring(false);

	let sq = smitty.make_texture("images/puare.png");

	let mut us = Thing::new((-3.0, -3.0), (1, 1), Color::rgb(0.1, 0.3, 0.5));
	let speed = 0.075;

	let mut grid = Grid::new((5, 5), (1, 1), 1.0);
	let gridlines = grid.gridlines();

	loop {
		let _events = smitty.events();

		// Keyboard Control
		if smitty.is_key_down(Key::Escape) {
			break;
		}

		if smitty.is_key_down(Key::W) {
			us.offset((0.0, speed));
		} else if smitty.is_key_down(Key::S) {
			us.offset((0.0, -speed));
		}

		if smitty.is_key_down(Key::A) {
			us.offset((-speed, 0.0));
		} else if smitty.is_key_down(Key::D) {
			us.offset((speed, 0.0));
		}

		if smitty.is_key_down(Key::E) {
			if let Some(coords) = grid.get_coords(us.center) {
				grid.set_tile(coords, Some(Tile::Solid));
			}
		}

		// Drawing
		smitty.clear();

		for line in &gridlines {
			smitty.sdf(*line);
		}

		for (tl, p) in grid.tiles_and_position() {
			match tl {
				Some(Tile::Solid) => smitty.rect(p, (grid.side_length, grid.side_length), sq),
				_ => (),
			}
		}

		if let Some(p) = grid.coordinate_center(2, 2) {
			smitty.sdf(SignedDistance::Circle {
				center: p,
				radius: 2,
				color: Color::YELLOW,
			})
		}

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

#[derive(Copy, Clone, Debug)]
struct Size {
	width: u32,
	height: u32,
}

impl Size {
	pub fn stride(&self) -> usize {
		self.width as usize * self.height as usize
	}

	pub fn half(&self) -> Vec2 {
		Vec2::new(self.width as f32 / 2.0, self.height as f32 / 2.0)
	}
}

impl From<(u32, u32)> for Size {
	fn from(t: (u32, u32)) -> Self {
		Size {
			width: t.0,
			height: t.1,
		}
	}
}

impl From<Size> for Vec2 {
	fn from(s: Size) -> Self {
		Vec2 {
			x: s.width as f32,
			y: s.width as f32,
		}
	}
}

struct Grid {
	size: Size,
	position: Vec2,

	side_length: f32,
	grid: Vec<Option<Tile>>,
}

impl Grid {
	pub fn new<S: Into<Size>, P: Into<Vec2>>(size: S, position: P, side_length: f32) -> Self {
		let size = size.into();

		Self {
			size,
			position: position.into(),
			side_length,
			grid: vec![None; size.stride()],
		}
	}

	pub fn gridlines(&self) -> Vec<SignedDistance> {
		let mut lines = vec![];

		let half_size = self.size.half();

		let x_start = self.position.x - half_size.x;
		let x_end = self.position.x + half_size.x;

		let y_start = self.position.y - half_size.y;
		let y_end = self.position.y + half_size.y;

		for x in 0..=self.size.width {
			let x_position = (x_start + x as f32) * self.side_length;

			lines.push(SignedDistance::LineSegment {
				start: Vec2::new(x_position, y_start),
				end: Vec2::new(x_position, y_end),
				thickness: 2,
				color: Color::rgb(0.5, 0.3, 0.0),
			});
		}

		for y in 0..=self.size.height {
			let y_position = (y_start + y as f32) * self.side_length;

			lines.push(SignedDistance::LineSegment {
				start: Vec2::new(x_start, y_position),
				end: Vec2::new(x_end, y_position),
				thickness: 2,
				color: Color::rgb(0.5, 0.3, 0.0),
			});
		}

		lines
	}

	pub fn coordinate_center(&self, x: u32, y: u32) -> Option<Vec2> {
		if x >= self.size.width && y >= self.size.height {
			return None;
		}

		let half_size = self.size.half();
		Some(Vec2 {
			x: (x as f32 - half_size.x) * self.side_length
				+ (self.side_length / 2.0)
				+ self.position.x,
			y: (y as f32 - half_size.y) * self.side_length
				+ (self.side_length / 2.0)
				+ self.position.y,
		})
	}

	//TODO: gen- function name?
	pub fn get_coords(&self, loc: Vec2) -> Option<Size> {
		let offset_to_center = loc - self.position;

		//TODO: gen- Why uh, why is it half_size PLUS offset?
		let half_size = self.size.half();
		let coords = (half_size + offset_to_center) / self.side_length;

		if coords.x < 0.0
			|| coords.y < 0.0
			|| coords.x > self.size.width as f32
			|| coords.y > self.size.height as f32
		{
			None
		} else {
			Some(Size {
				width: coords.x as u32,
				height: coords.y as u32,
			})
		}
	}

	pub fn set_tile(&mut self, coords: Size, tile: Option<Tile>) {
		//TODO: gen- check coordinates valid
		self.grid[coords.height as usize * self.size.width as usize + coords.width as usize] = tile;
	}

	pub fn tiles_and_position(&self) -> Vec<(Option<&Tile>, Vec2)> {
		let mut ret = vec![];

		for (idx, tile) in self.grid.iter().enumerate() {
			let y = idx / self.size.width as usize;
			let x = idx % self.size.width as usize;

			ret.push((
				tile.as_ref(),
				self.coordinate_center(x as u32, y as u32).unwrap(),
			))
		}

		ret
	}
}

#[derive(Clone, Copy, Debug)]
enum Tile {
	Solid,
}
