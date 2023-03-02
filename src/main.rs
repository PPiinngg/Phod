use std::{
	sync::{Arc, Mutex},
	thread,
	time::Duration,
};

mod audio;
mod ui;

fn main() {
	let thread_ui = thread::spawn(|| {
		ui::ui_main();
	});

	let _thread_audio = thread::spawn(|| {
		audio::audio_main();
	});

	thread_ui.join().expect("ui thread join shat itself");
}
