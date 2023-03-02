use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Data,
};
mod karplus;
mod dsp;

pub fn audio_main() {
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



	////////////////////////////////////////////////////////
	// INIT OUTPUT STREAM //////////////////////////////////
	////////////////////////////////////////////////////////
	let _stream = device.build_output_stream(
		&config,
		move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
			// react to stream events and read or write stream data here.
		},
		move |err| {
			// react to errors here.
		},
		None, // None=blocking, Some(Duration)=timeout
	);
}
