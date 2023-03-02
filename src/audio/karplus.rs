struct DelayLine {
	index: usize,
	buffer: Vec<f32>,
}
impl DelayLine {
	pub fn new(max_size: usize) -> Self {
		Self { index: 0usize, buffer: Vec::<f32>::with_capacity(max_size) }
	}
	pub fn read(&mut self) -> f32 {
		self.buffer[self.index]
	}
	pub fn write(&mut self, v: f32) {
		self.buffer[self.index] = v
	}
	pub fn step(&mut self) {
		self.index += 1usize
	}
	pub fn resize(&mut self, size: usize) {
		todo!("DelayLine resize");
	}
}

