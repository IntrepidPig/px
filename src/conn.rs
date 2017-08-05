use xcb;

use win::{Window};
use event::{Event};

pub struct Connection {
	conn: xcb::Connection,
	screen: i32,
	root: Window,
}

impl Connection {
	pub fn new() -> Result<Connection, ()> {
		let (conn, screen) = match xcb::Connection::connect(None) {
			Ok((conn, screen)) => (conn, screen),
			Err(_) => return Err(())
		};

		let root = conn.get_setup().roots().nth(screen as usize).unwrap().root(); // TODO handle this error

		Ok(Connection { conn: conn, screen: screen, root: Window::new(root) })
	}
	
	fn wait_for_event(&self) -> xcb::GenericEvent {
		self.conn.flush();

		self.conn.wait_for_event().unwrap()
	}

	pub fn map_window(&self, win: Window) {
		xcb::map_window(&self.conn, win.x());
	}
}

pub struct EventLoop<'a> {
	conn: &'a Connection
}

impl<'a> EventLoop<'a> {
	pub fn new(conn: &'a Connection) -> Result<EventLoop<'a>, ()> {
		// Install as window manager
		let mask = vec![
			(
				xcb::CW_EVENT_MASK,
				xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY | xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT
			)
		];

		match xcb::change_window_attributes_checked(&conn.conn, conn.root.x(), &mask).request_check() { //TODO use new fancy unwrap_or_else
			Ok(_) => {
				Ok(EventLoop { conn: conn })
			},
			Err(_) => {
				Err(())
			}
		}
	}

	fn cast_event(&self, e: xcb::GenericEvent) -> Result<Event, ()> {
		use event::*;
		match e.response_type() {
			xcb::MAP_REQUEST => {
				let xevent: &xcb::MapRequestEvent = unsafe { xcb::cast_event(&e) };
				return Ok(Event::MapReqEvent(MapReq {
					window: Window::new(xevent.window()),
					parent: Window::new(xevent.parent()),
					sibling: Window::new(xevent.parent()),
				}))
			},
			xcb::CONFIGURE_REQUEST => { 
				let xevent: &xcb::ConfigureRequestEvent = unsafe { xcb::cast_event(&e) };
				return Ok(Event::ConfReqEvent(ConfReq {
					window: Window::new(xevent.window()),
					parent: Window::new(xevent.parent()),
					sibling: Window::new(xevent.parent()),
					//stack_mode: xevent.stack_mode(),
					//sequence: xevent.sequence,
					x: xevent.x(),
					y: xevent.y(),
					width: xevent.width(),
					height: xevent.height(),
					border_width: xevent.border_width(),
					value_mask: xevent.value_mask()
				}))
			},
			_ => Ok(Event::Debug),
		}
	}
}

impl<'a> Iterator for &'a EventLoop<'a> {
	type Item = Event;

	fn next(&mut self) -> Option<Self::Item> {
		Some(self.cast_event(self.conn.wait_for_event()).unwrap())
	}
}