use win::Window;

#[derive(Debug)]
pub enum Event {
	ConfReqEvent(ConfReq),
	MapReqEvent(MapReq),
	Debug,
}

#[derive(Debug)]
pub struct ConfReq { // Info about a configuration request
	pub window: Window,
	pub parent: Window,
	pub sibling: Window,
	pub x: i16,
	pub y: i16,
	pub width: u16,
	pub height: u16,
	pub border_width: u16,
	pub value_mask: u16,
}

#[derive(Debug)]
pub struct MapReq {
	pub window: Window,
	pub parent: Window,
	pub sibling: Window,
}