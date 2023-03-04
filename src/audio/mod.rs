use std::sync::Arc;

use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Data,
};
use crossbeam_utils::atomic::AtomicCell;

use crate::GlobalState;

use self::karplus::DelayModule;
mod dsp;
mod karplus;

fn render(output: &mut [f32], channels: usize, next_sample: &mut dyn FnMut() -> f32) {
	for frame in output.chunks_mut(channels) {
		let value = next_sample();
		for sample in frame.iter_mut() {
			*sample = value;
		}
	}
}

pub fn audio_main(state: Arc<GlobalState>) {
	////////////////////////////////////////////////////////
	// INIT CPAL CONFIG ////////////////////////////////////
	////////////////////////////////////////////////////////
	let host = cpal::default_host();
	let device = host
		.default_output_device()
		.expect("No output device available");

	let mut supported_configs_range = device
		.supported_output_configs()
		.expect("Error while querying configs");
	let supported_config = supported_configs_range
		.next()
		.expect("No supported config?!")
		.with_max_sample_rate();
	let config = supported_config.config();

	////////////////////////////////////////////////////////
	// INIT KARPLUS GENERATOR //////////////////////////////
	////////////////////////////////////////////////////////
	let test_delay = DelayModule::new(10_000_usize);

	////////////////////////////////////////////////////////
	// INIT OUTPUT STREAM //////////////////////////////////
	////////////////////////////////////////////////////////
	let mut render_closure = move || {
		return state.button.load();
	};
	let _stream = device.build_output_stream(
		&config,
		move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
			render(data, config.channels as usize, &mut render_closure)
		},
		move |err| {
			// react to errors here.
		},
		None, // None=blocking, Some(Duration)=timeout
	);
}
