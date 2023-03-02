struct DelayLine {
	index: usize,
	buffer: Vec<f32>,
}
impl DelayLine {
	pub fn new(max_size: usize) -> Self {
		let mut buffer = Vec::<f32>::with_capacity(max_size);
		buffer.resize(max_size, 0.0f32);
		Self { index: 0usize, buffer: buffer }
	}
	pub fn read(&mut self) -> f32 {
		self.buffer[self.index]
	}
	pub fn write(&mut self, sample: f32) {
		self.buffer[self.index] = sample;
	}
	pub fn step(&mut self) {
		self.index += 1usize;
	}
	pub fn resize(&mut self, size: usize) {
		todo!("DelayLine resize");
	}
}

pub struct DelayModule {
	feedback: f32,
	delay_line: DelayLine,
}
impl DelayModule {
	pub fn new(max_size: usize) -> Self {
		Self { feedback: 0.0f32,
		delay_line: DelayLine::new(max_size) }
	}
	pub fn tick(&mut self, sample: f32) -> f32 {
		let new_sample: f32 = (self.delay_line.read() * self.feedback) + sample;
		self.delay_line.write(new_sample);
		self.delay_line.read()
	}
	pub fn resize(&mut self, size: usize) {
		self.delay_line.resize(size);
	}
}
