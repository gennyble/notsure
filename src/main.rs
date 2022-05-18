use smitten::{self, Smitten, VirtualKeyCode};

fn main() {
	let mut smitty = Smitten::new((720, 480), "Square", 36);
	let cheerpuff = smitty.make_texture("images/cheerpuff.png");

	loop {
		let _events = smitty.events();
		if smitty.is_key_down(VirtualKeyCode::Escape) {
			break;
		}

		smitty.clear();
		smitty.rect((0, 0), (1, 1), cheerpuff);
		smitty.swap();
	}
}
