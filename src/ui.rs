use std::{
	ffi::{CStr, CString},
	sync::atomic::AtomicBool,
};

use crossbeam_utils::atomic::AtomicCell;
use raylib::prelude::*;

macro_rules! str2cstr {
	($s:expr) => {
		CString::new($s)
			.expect("Failed to create CString from rust string slice")
			.as_c_str()
	};
}

pub fn ui_main(buttonval: &AtomicCell<f32>) {
	let (mut rl, thread) = raylib::init()
		.size(640, 480)
		.title(format!("Phod [v{}b]", env!("CARGO_PKG_VERSION")).as_str())
		.build();

	while !rl.window_should_close() {
		let mut d = rl.begin_drawing(&thread);
		d.gui_enable();

		d.clear_background(Color::WHITE);

		if d.gui_button(
			Rectangle {
				x: 10.0f32,
				y: 10.0f32,
				width: 30.0f32,
				height: 15.0f32,
			},
			Some(str2cstr!("trig")),
		) {
			buttonval.store(0.5_f32);
		} else {
			buttonval.store(0.0_f32);
		}
	}
}
