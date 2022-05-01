pub struct TableMap<T> {
	pub data: Vec<T>,
	pub width: u32,
	pub height: u32,
}

impl<T: Default> TableMap<T> {
	pub fn new(width: u32, height: u32) -> TableMap<T> {
		let mut data = Vec::with_capacity((width * height) as usize);
		for _ in 0..(width * height) {
			data.push(T::default());
		}

		TableMap {
			data,
			width,
			height,
		}
	}

	pub fn from_raw(data: Vec<T>, width: u32, height: u32) -> TableMap<T> {
		if data.len() != (width * height) as usize {
			panic!("Size does not match");
		}

		TableMap {
			data,
			width,
			height,
		}
	}

	pub fn insert(&mut self, x: u32, y: u32, value: T) {
        assert!(x < self.width && y < self.height);
		self.data[(x + (y * self.width)) as usize] = value;
	}

	pub fn get(&self, x: u32, y: u32) -> &T {
        assert!(x < self.width && y < self.height);
		&self.data[(x + (y * self.width)) as usize]
	}

	pub fn get_mut(&mut self, x: u32, y: u32) -> &mut T {
        assert!(x < self.width && y < self.height);
        &mut self.data[(x + (y * self.width)) as usize]
	}

	pub fn for_each(&self, mut func: impl FnMut(u32, u32, &T)) {
		for y in 0..self.height {
			for x in 0..self.width {
				func(x, y, self.get(x, y));
			}
		}
	}
}
