use std::{sync::atomic::AtomicBool, thread, time::Duration};

use crossbeam_utils::atomic::AtomicCell;

mod audio;
mod ui;

// struct 

fn main() {
	let buttonpressed = AtomicCell::new(0.0_f32);

	let thread_ui = thread::spawn(|| {
		ui::ui_main(&buttonpressed);
	});

	let thread_audio = thread::spawn(|| {
		audio::audio_main(&buttonpressed);
	});

	thread_ui.join().expect("ui thread join shat itself");
	thread_audio.join().expect("audio thread join shat itself");
}
