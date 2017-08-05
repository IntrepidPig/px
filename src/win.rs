use xcb;

#[derive(Debug)]
pub struct Window {
	id: xcb::Window
}

impl Window {
	pub fn new(id: xcb::Window) -> Window {
		Window { id: id }
	}

	pub fn x(&self) -> xcb::Window {
		self.id
	}
}