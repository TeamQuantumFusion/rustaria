use crate::{TableMap};
use std::ops::{Add, Mul, Sub};

pub fn bilinear<T: Default>(map: &TableMap<T>, x: f32, y: f32) -> [(&T, f32); 4] {
	let c00 = saturating_table_map_get(map, x as u32, y as u32);
	let c01 = saturating_table_map_get(map, x as u32, y as u32 + 1);
	let c11 = saturating_table_map_get(map, x as u32 + 1, y as u32 + 1);
	let c10 = saturating_table_map_get(map, x as u32 + 1, y as u32);

	let tx = x % 1.0;
	let ty = y % 1.0;
	[
		(c00, (1.0 - tx) * (1.0 - ty)),
		(c10, tx * (1.0 - ty)),
		(c01, (1.0 - tx) * ty),
		(c11, tx * ty),
	]
}


pub fn bicubic<T>(map: &TableMap<T>, x: f32, y: f32) -> T
where
	T: Add<Output = T> + Sub<Output = T> + Copy + Mul<f32, Output = T> + Default,
{
	let x_low = x.floor();
	let y_low = y.floor();
	let frac_x = x - x_low;
	let frac_y = y - y_low;

	let mut data: [[T; 4]; 4] = [[T::default(); 4]; 4];

	for ndatay in 0..=3 {
		for ndatax in 0..=3 {
			data[ndatay][ndatax] = *saturating_table_map_get(
				map,
				x_low as u32 + ndatax as u32,
				y_low as u32 + ndatay as u32,
			);
		}
	}

	bicubic_interpolate(data, frac_x, frac_y)
}

pub fn saturating_table_map_get<T: Default>(map: &TableMap<T>, x: u32, y: u32) -> &T {
	map.get(x.min(map.width - 1), y.min(map.height - 1))
}

pub fn bicubic_interpolate<T>(data: [[T; 4]; 4], frac_x: f32, frac_y: f32) -> T
where
	T: Add<Output = T> + Sub<Output = T> + Copy + Mul<f32, Output = T>,
{
	let x1 = cubic_interpolate(&data[0][0], &data[0][1], &data[0][2], &data[0][3], frac_x);
	let x2 = cubic_interpolate(&data[1][0], &data[1][1], &data[1][2], &data[1][3], frac_x);
	let x3 = cubic_interpolate(&data[2][0], &data[2][1], &data[2][2], &data[2][3], frac_x);
	let x4 = cubic_interpolate(&data[3][0], &data[3][1], &data[3][2], &data[3][3], frac_x);
	cubic_interpolate(&x1, &x2, &x3, &x4, frac_y)
}

pub fn cubic_interpolate<T>(v0: &T, v1: &T, v2: &T, v3: &T, frac: f32) -> T
where
	T: Add<Output = T> + Sub<Output = T> + Copy + Mul<f32, Output = T>,
{
	let a = (*v3 - *v2) - (*v0 - *v1);
	let b = (*v0 - *v1) - a;
	let c = *v2 - *v0;
	let d = *v1;

	let a_res = a * frac * frac * frac;
	let b_res = b * frac * frac;
	let c_res = c * frac;
	let d_res = d;

	a_res + b_res + c_res + d_res
}

#[cfg(test)]
mod tests {

	#[test]
	pub fn test() {
		let frac = 0.5;
		let v0 = 0.67;
		let v1 = 0.243;
		let v2 = 0.6431643;
		let v3 = 0.1543;

		let a = (v3 - v2) - (v0 - v1);
		let b = (v0 - v1) - a;
		let c = v2 - v0;
		let d = v1;
		println!("{b}");
	}
}
