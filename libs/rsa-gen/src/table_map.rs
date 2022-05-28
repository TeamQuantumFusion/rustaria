use std::ops::Range;

pub struct TableMap<T> {
	pub data: Vec<T>,
	pub width: u32,
	pub height: u32,
	pub change_history: Option<Vec<u16>>,
}

impl<T> TableMap<T> {
	pub fn from_raw(data: Vec<T>, width: u32, height: u32) -> TableMap<T> {
		if data.len() != (width * height) as usize {
			panic!("Size does not match");
		}

		TableMap {
			data,
			width,
			height,
			change_history: None,
		}
	}

	pub fn insert(&mut self, x: u32, y: u32, value: T) {
		assert!(x < self.width && y < self.height);
		let idx = (x + (y * self.width)) as usize;

		if let Some(history) = &mut self.change_history {
			history[idx] = history[idx].saturating_add(1);
		}

		self.data[(x + (y * self.width)) as usize] = value;
	}

	pub fn get(&self, x: u32, y: u32) -> &T {
		assert!(
			x < self.width && y < self.height,
			"{} < {} | {} < {}",
			x,
			self.width,
			y,
			self.height
		);
		&self.data[(x + (y * self.width)) as usize]
	}

	pub fn get_mut(&mut self, x: u32, y: u32) -> &mut T {
		assert!(x < self.width && y < self.height);
		&mut self.data[(x + (y * self.width)) as usize]
	}

	pub fn inc(&mut self, x: u32, y: u32) {
		assert!(x < self.width && y < self.height);
		let idx = (x + (y * self.width)) as usize;

		if let Some(history) = &mut self.change_history {
			history[idx] = history[idx].saturating_add(1);
		}
	}

	pub fn for_each(&self, mut func: impl FnMut(u32, u32, &T)) {
		for y in 0..self.height {
			for x in 0..self.width {
				func(x, y, self.get(x, y));
			}
		}
	}

	pub fn for_each_mut(&mut self, mut func: impl FnMut(u32, u32, &mut T)) {
		for y in 0..self.height {
			for x in 0..self.width {
				func(x, y, self.get_mut(x, y));
			}
		}
	}

	pub fn slice_mut(&mut self, x_range: Range<u32>, y_range: Range<u32>) -> TableMapMutSlice<T> {
		TableMapMutSlice {
			map: self,
			x_range,
			y_range,
		}
	}

	pub fn slice(&self, x_range: Range<u32>, y_range: Range<u32>) -> TableMapSlice<T> {
		TableMapSlice {
			data: self,
			x_range,
			y_range,
		}
	}
}

pub struct TableMapMutSlice<'a, T> {
	map: &'a mut TableMap<T>,
	pub x_range: Range<u32>,
	pub y_range: Range<u32>,
}

impl<'a, T> TableMapMutSlice<'a, T> {
	pub fn get(&self, x: u32, y: u32) -> &T {
		assert!(self.x_range.contains(&x) && self.y_range.contains(&y));
		self.map.get(x, y)
	}

	pub fn get_mut(&mut self, x: u32, y: u32) -> &mut T {
		assert!(self.x_range.contains(&x) && self.y_range.contains(&y));
		self.map.get_mut(x, y)
	}

	pub fn insert(&mut self, x: u32, y: u32, value: T) {
		assert!(self.x_range.contains(&x) && self.y_range.contains(&y));
		self.map.insert(x, y, value)
	}

	pub fn inc(&mut self, x: u32, y: u32) {
		assert!(self.x_range.contains(&x) && self.y_range.contains(&y));
		self.map.inc(x, y);
	}

	pub fn for_each(&self, mut func: impl FnMut(u32, u32, &T)) {
		for y in self.y_range.clone() {
			for x in self.x_range.clone() {
				func(x, y, self.get(x, y));
			}
		}
	}

	pub fn for_each_mut(&mut self, mut func: impl FnMut(u32, u32, &mut T)) {
		for y in self.y_range.clone() {
			for x in self.x_range.clone() {
				func(x, y, self.get_mut(x, y));
			}
		}
	}
}
pub struct TableMapSlice<'a, T> {
	data: &'a TableMap<T>,
	pub x_range: Range<u32>,
	pub y_range: Range<u32>,
}

impl<'a, T> TableMapSlice<'a, T> {
	pub fn get(&self, x: u32, y: u32) -> &T {
		assert!(self.x_range.contains(&x) && self.y_range.contains(&y));
		self.data.get(x, y)
	}

	pub fn for_each(&self, mut func: impl FnMut(u32, u32, &T)) {
		for y in self.y_range.clone() {
			for x in self.x_range.clone() {
				func(x, y, self.get(x, y));
			}
		}
	}
}

impl<T: Default> TableMap<T> {
	pub fn new_default(width: u32, height: u32, history: bool) -> TableMap<T> {
		let mut data = Vec::with_capacity((width * height) as usize);
		for _ in 0..(width * height) {
			data.push(T::default());
		}

		TableMap {
			data,
			width,
			height,
			change_history: history.then(|| vec![0u16; (width * height) as usize]),
		}
	}
}

impl<T: Clone> TableMap<T> {
	pub fn new_with(width: u32, height: u32, history: bool, default: T) -> TableMap<T> {
		let mut data = Vec::with_capacity((width * height) as usize);
		for _ in 0..(width * height) {
			data.push(default.clone());
		}

		TableMap {
			data,
			width,
			height,
			change_history: history.then(|| vec![0u16; (width * height) as usize]),
		}
	}
}
