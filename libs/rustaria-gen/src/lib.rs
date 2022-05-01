use crate::noise::{Noise, SampleNoise};
use crate::table_map::TableMap;
use crate::ty::{
	Biome, BiomeId, BiomeSettings, Climate, ClimateId, ClimateSettings, Zone, ZoneId, ZoneSettings,
};
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro128StarStar;
use rustaria_api::ty::Tag;
use rustaria_common::ty::CHUNK_SIZE;
use std::collections::HashMap;
use std::ops::Range;

mod noise;
mod table_map;
mod ty;

pub struct Generator {
	zones: Vec<Zone>,
	climates: Vec<Climate>,
	biomes: Vec<Biome>,

	spawn_size: f32,
	biome_height_transition: f32,

	seed: u64,
	// dimensions
	width: u32,
	height: u32,
	chunk_width: u32,
	chunk_height: u32,

	noise: Noise,
	rng: Xoshiro128StarStar,
}

impl Generator {
	#[rustfmt::skip]
	pub fn new(
		mut zones: Vec<(Tag, ZoneSettings)>,
		mut climates: Vec<(Tag, ClimateSettings)>,
		mut biomes: Vec<(Tag, BiomeSettings)>,
		spawn_size: f32,
		biome_height_transition: f32,
		seed: u64,
		width: u32,
		height: u32,
	) -> Generator {
		// === Sort values
		zones.sort_by(|(_, v0), (_, v1)| v0.priority.total_cmp(&v1.priority));
		climates.sort_by(|(v0, _), (v1, _)| v0.cmp(v1));
		biomes.sort_by(|(v0, _), (v1, _)| v0.cmp(v1));

		// === Create lookup
		let zones_lookup: HashMap<Tag, ZoneId> = zones.iter().enumerate().map(|(id, (tag, _))| (tag.clone(), ZoneId(id as u16))).collect();
		let climates_lookup: HashMap<Tag, ClimateId> = climates.iter().enumerate().map(|(id, (tag, _))| (tag.clone(), ClimateId(id as u16))).collect();
		let biomes_lookup: HashMap<Tag, BiomeId> = biomes.iter().enumerate().map(|(id, (tag, _))| (tag.clone(), BiomeId(id as u16))).collect();

		// === Create values
		let mut zones_out = Vec::new();
		let mut climates_out = Vec::new();
		let mut biomes_out = Vec::new();

		// === Zones
		let mut total_height = 0.0;
		for (_, settings) in &zones {
			total_height += settings.w_height;
			zones_out.push(Zone {
				w_height: settings.w_height,
				surface_size: settings.surface_size,
				surface_transition: settings.surface_transition,
				surface_biome: *biomes_lookup.get(&settings.surface_biome).unwrap(),
				cave_biome: *biomes_lookup.get(&settings.cave_biome).unwrap(),
				child_climates: vec![],
				child_biomes: vec![],
				world_height: 0..0,
			})
		}
		// Normalize height
		zones_out.iter_mut().for_each(|zone| zone.w_height /= total_height);

		// === Climates
		let mut total_w_width = 1000.0;
		for (_, settings) in climates {
			total_w_width += settings.w_width;
			climates_out.push(Climate {
				shape: settings.shape,
				w_width: settings.w_width,
				depth: settings.depth,
				surface_size: settings.surface_size,
				surface_transition: settings.surface_transition,
				surface_biome: *biomes_lookup.get(&settings.surface_biome).unwrap(),
				cave_biome: *biomes_lookup.get(&settings.cave_biome).unwrap(),
				child_biomes: vec![],
				zones: settings.zones.iter().map(|zone| {
					let x = *zones_lookup.get(zone).unwrap();
					zones_out[x.0 as usize].child_climates.push((ClimateId(climates_out.len() as u16), 0..0));
					x
				}).collect()
			})
		}
		climates_out.iter_mut().for_each(|zone| zone.w_width /= total_w_width);


		// === Biomes
		for (id, (_, settings)) in biomes.into_iter().enumerate() {
			biomes_out.push(Biome {
				color: settings.color,
				air_ratio: settings.air_ratio,
				height_range: settings.height_range,
				noise: Noise::Sample(Box::new(SampleNoise::new(
					seed.overflowing_add(id as u64).0,
					settings.air_scale,
				))),
				zones: settings.zones.iter().map(|zone| {
					let x = *zones_lookup.get(zone).unwrap();
					zones_out[x.0 as usize].child_biomes.push(BiomeId(biomes_out.len() as u16));
					x
				}).collect(),
				climates: settings.climates.iter().map(|zone| {
					let x = *climates_lookup.get(zone).unwrap();
					climates_out[x.0 as usize].child_biomes.push(BiomeId(biomes_out.len() as u16));
					x
				}).collect(),
			})
		}


		Generator {
			zones: zones_out,
			climates: climates_out,
			biomes: biomes_out,
			spawn_size,
			biome_height_transition,
			seed,
			width,
			height,
			chunk_width: width / CHUNK_SIZE as u32,
			chunk_height: height / CHUNK_SIZE as u32,
			noise: Noise::octave(seed, 5.0, 4),
			rng: rand_xoshiro::Xoshiro128StarStar::seed_from_u64(seed)
		}
	}

	pub fn compute_zone_heights(&mut self) {
		// Compute zone heights
		let mut current_height = 0u32;
		let length = self.zones.len();
		for (id, zone) in self.zones.iter_mut().enumerate() {
			// Find how tall the biome is
			let zone_height = if id == length - 1 {
				// if last set to max pos
				self.chunk_height - current_height
			} else {
				(self.chunk_height as f32 * zone.w_height) as u32
			};

			zone.world_height = current_height..current_height + zone_height;
			current_height += zone_height;
		}
	}

	pub fn compute_climate_widths(&mut self) {
		// Wizord™™ Wizard© algorithm©®™
		let cluster_width = (1.0 - self.spawn_size) / 2.0;
		for zone in &mut self.zones {
			zone.child_climates.shuffle(&mut self.rng);

			let left_size = self.climates.len() / 2;
			let size = [left_size, self.climates.len() - (left_size)];
			let mut cluster_spacer_width = [cluster_width, cluster_width];
			for (pos, (id, _)) in zone.child_climates.iter_mut().enumerate() {
				let climate = &self.climates[id.0 as usize];
				cluster_spacer_width[(pos < size[0]) as usize] -= climate.w_width
			}

			let mut current_width = [0.0, cluster_width + self.spawn_size];
			for (pos, (id, width)) in zone.child_climates.iter_mut().enumerate() {
				let climate = &self.climates[id.0 as usize];
				let idx = (pos < size[0]) as usize;

				current_width[idx] += cluster_spacer_width[idx] / (size[idx] as f32 + 1.0);
				let start = current_width[idx];
				current_width[idx] += climate.w_width;
				let end = current_width[idx];

				*width = (self.chunk_width as f32 * start) as u32
					..(self.chunk_width as f32 * end) as u32;
			}
		}
	}

	pub fn generate_biome_map(&mut self) -> TableMap<BiomeId> {
		let mut out = TableMap::new(self.chunk_width, self.chunk_height);

		for zone in &self.zones {
			// Fill the zones default biome
			for y in zone.world_height.clone() {
				for x in 0..self.chunk_width {
					out.insert(x, y, self.get_y_biome(
						x,
						y,
						&zone.world_height,
						zone.surface_biome,
						zone.cave_biome,
						zone.surface_size,
						zone.surface_transition,
					));
				}
			}
			// Generate the zone biomes. (below the climates)
			for biome_id in &zone.child_biomes {
				let biome = &self.biomes[biome_id.0 as usize];
				for y in zone.world_height.clone() {
					for x in 0..self.chunk_width {
						if self.sample_biome(x, y, biome) {
							out.insert(x, y, *biome_id);
						}
					}
				}
			}

			for (climate_id, climate_x_range) in &zone.child_climates {
				let climate = &self.climates[climate_id.0 as usize];

				let x_offset = climate_x_range.start;
				let y_offset = zone.world_height.start;

				let width = climate_x_range.end - x_offset;
				let height = ((zone.world_height.end - y_offset) as f32 * climate.depth) as u32;

				let climate_x_range = climate_x_range.clone();
				let climate_y_range = y_offset..(y_offset + height);

				// Fill the climates default biome
				for y in climate_y_range.clone() {
					for x in climate_x_range.clone() {
						if climate.shape.inside(
							(x - x_offset) as f32 / width as f32,
							(y - y_offset) as f32 / height as f32,
						) {
							out.insert(
								x,
								y,
								self.get_y_biome(
									x,
									y,
									&zone.world_height,
									climate.surface_biome,
									climate.cave_biome,
									climate.surface_size,
									climate.surface_transition,
								),
							);
						}
					}
				}

				// Generate the climate biomes.
				for biome_id in &climate.child_biomes {
					let biome = &self.biomes[biome_id.0 as usize];

					for y in climate_y_range.clone() {
						for x in climate_x_range.clone() {
							let y_f = (y - y_offset) as f32 / height as f32;
							let x_f = (x - x_offset) as f32 / width as f32;
							if climate.shape.inside(x_f, y_f) && self.sample_biome(x, y, biome) {
								out.insert(x, y, *biome_id);
							}
						}
					}
				}
			}
		}

		out
	}



	fn get_y_biome(
		&self,
		x: u32,
		y: u32,
		zone_range: &Range<u32>,
		surface_biome: BiomeId,
		cave_biome: BiomeId,
		surface_size: f32,
		surface_transition: f32,
	) -> BiomeId {
		let float_y = (y - zone_range.start) as f32 / (zone_range.end - zone_range.start) as f32;

		if float_y >= surface_size + surface_transition {
			cave_biome
		} else if float_y >= surface_size {
			let float_y = (float_y - surface_size) / surface_transition;
			let noise = self.noise.sample(x as f32 / 10.0, y as f32 / 10.0);
			if noise < float_y {
				cave_biome
			} else {
				surface_biome
			}
		} else {
			surface_biome
		}
	}

	fn sample_biome(&self, x: u32, y: u32, biome: &Biome) -> bool {
		let height_range = biome.height_range.clone();

		let float_y = y as f32 / (self.chunk_height as f32);
		let bias = if height_range.contains(&float_y) {
			1.0
		} else {
			let height = height_range.end - height_range.start;
			let middle = height_range.start + (height / 2.0);
			let middle_distance = (float_y - middle).abs();
			let distance = (middle_distance - (height / 2.0)).max(0.0);
			1.0 - (distance.clamp(0.0, self.biome_height_transition) / self.biome_height_transition)
		};

		biome.noise.sample(x as f32, y as f32) <= biome.air_ratio * bias
	}
}

#[cfg(test)]
mod tests {
	use crate::ty::ClimateShape;
	use crate::{BiomeSettings, ClimateSettings, Generator, ZoneSettings};
	use image::{Rgb, RgbImage};
	use rustaria_api::ty::Tag;

	#[test]
	fn export_biome_map() {
		fn zone_biome(color: [u8; 3], name: &str) -> (Tag, BiomeSettings) {
			(
				Tag::builtin(name),
				BiomeSettings {
					color,
					air_ratio: 1.0,
					air_scale: 0.0,
					height_range: Default::default(),
					zones: vec![],
					climates: vec![],
				},
			)
		}
		let mut generator = Generator::new(
			vec![
				(
					Tag::builtin("sky"),
					ZoneSettings {
						w_height: 1000.0,
						priority: 0.0,
						surface_size: 0.0,
						surface_transition: 0.0,
						surface_biome: Tag::builtin("sky"),
						cave_biome: Tag::builtin("sky"),
					},
				),
				(
					Tag::builtin("surface"),
					ZoneSettings {
						w_height: 5000.0,
						priority: 0.0,
						surface_size: 0.0,
						surface_transition: 0.3,
						surface_biome: Tag::builtin("surface"),
						cave_biome: Tag::builtin("cave"),
					},
				),
				(
					Tag::builtin("underworld"),
					ZoneSettings {
						w_height: 1000.0,
						priority: 0.0,
						surface_size: 0.5,
						surface_transition: 0.2,
						surface_biome: Tag::builtin("underworld_surface"),
						cave_biome: Tag::builtin("underworld"),
					},
				),
			],
			vec![
				(
					Tag::builtin("desert"),
					ClimateSettings {
						shape: ClimateShape::Oval { offset_y: 0.2 },
						w_width: 300.0,
						depth: 0.5,
						surface_size: 0.0,
						surface_transition: 0.3,
						surface_biome: Tag::builtin("desert_surface"),
						cave_biome: Tag::builtin("desert"),
						zones: vec![Tag::builtin("surface")],
					},
				),
				(
					Tag::builtin("ice"),
					ClimateSettings {
						shape: ClimateShape::Triangle { offset_y: 0.6 },
						w_width: 300.0,
						depth: 0.6,
						surface_size: 0.0,
						surface_transition: 0.3,
						surface_biome: Tag::builtin("ice_surface"),
						cave_biome: Tag::builtin("ice"),
						zones: vec![Tag::builtin("surface")],
					},
				),
				(
					Tag::builtin("jungle"),
					ClimateSettings {
						shape: ClimateShape::Rectangle { sheer: 0.2 },
						w_width: 300.0,
						depth: 1.0,
						surface_size: 0.0,
						surface_transition: 0.3,
						surface_biome: Tag::builtin("jungle_surface"),
						cave_biome: Tag::builtin("jungle"),
						zones: vec![Tag::builtin("surface")],
					},
				),
			],
			vec![
				zone_biome([155, 209, 255], "sky"),
				zone_biome([128, 128, 128], "cave"),
				zone_biome([151, 107, 75], "surface"),
				zone_biome([68, 68, 76], "underworld"),
				zone_biome([51, 0, 0], "underworld_surface"),
				zone_biome([144, 195, 232], "ice"),
				zone_biome([211, 236, 241], "ice_surface"),
				zone_biome([212, 192, 100], "desert"),
				zone_biome([255, 218, 56], "desert_surface"),
				zone_biome([98, 124, 55], "jungle"),
				zone_biome([53, 80, 30], "jungle_surface"),
				(
					Tag::builtin("marble"),
					BiomeSettings {
						color: [168, 178, 204],
						air_ratio: 0.3,
						air_scale: 20.0,
						height_range: 0.5..0.8,
						zones: vec![Tag::builtin("surface")],
						climates: vec![Tag::builtin("ice"), Tag::builtin("desert")],
					},
				),
				(
					Tag::builtin("granite"),
					BiomeSettings {
						color: [50, 46, 104],
						air_ratio: 0.2,
						air_scale: 25.0,
						height_range: 0.5..0.8,
						zones: vec![Tag::builtin("surface")],
						climates: vec![Tag::builtin("ice"), Tag::builtin("desert")],
					},
				),
				(
					Tag::builtin("beez"),
					BiomeSettings {
						color: [248, 166, 2],
						air_ratio: 0.25,
						air_scale: 10.0,
						height_range: 0.0..0.8,
						zones: vec![],
						climates: vec![Tag::builtin("jungle")],
					},
				),
				(
					Tag::builtin("mushroom"),
					BiomeSettings {
						color: [93, 127, 255],
						air_ratio: 0.15,
						air_scale: 40.0,
						height_range: 0.2..0.8,
						zones: vec![Tag::builtin("surface")],
						climates: vec![],
					},
				),
				(
					Tag::builtin("sky_island"),
					BiomeSettings {
						color: [223, 255, 255],
						air_ratio: 0.4,
						air_scale: 40.0,
						height_range: 0.049..0.050,
						zones: vec![Tag::builtin("surface")],
						climates: vec![],
					},
				),
			],
			0.1,
			0.2,
			69420,
			6400,
			1800,
		);

		generator.compute_zone_heights();
		generator.compute_climate_widths();
		let map = generator.generate_biome_map();
		let mut image = RgbImage::new(generator.chunk_width, generator.chunk_height);

		map.for_each(|x, y, value| {
			image.put_pixel(x, y, Rgb(generator.biomes[value.0 as usize].color));
		});

		image.save("biomes.png");
		assert_eq!(2 + 2, 4);
	}
}
