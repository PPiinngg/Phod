use std::{sync::{atomic::AtomicBool, Arc}, thread, time::Duration};

use crossbeam_utils::atomic::AtomicCell;

mod audio;
mod ui;

pub struct GlobalState {
	pub button: AtomicCell<f32>,
}
impl GlobalState {
	pub fn new() -> Self {
		Self { button: AtomicCell::new(0.0_f32) }
	}
}

fn main() {
	let state: Arc<GlobalState> = Arc::new(GlobalState::new());

	let state_ui = state.clone();
	let thread_ui = thread::spawn(|| {
		ui::ui_main(state_ui);
	});

	let state_audio = state.clone();
	let thread_audio = thread::spawn(|| {
		audio::audio_main(state_audio);
	});

	thread_ui.join().expect("ui thread join shat itself");
	thread_audio.join().expect("audio thread join shat itself");
}
