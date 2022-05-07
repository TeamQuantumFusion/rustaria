use crate::settings::BiomeProducer;
use crate::sweep::sampler::noise::NoiseSampler;
use crate::sweep::sampler::{NoiseKind, Sampler};
use crate::sweep::Sweep;
use crate::{Biome, BiomeId, Generator, TableMap};

pub struct BiomeMap {
	pub data: TableMap<BiomeId>,
}

impl BiomeMap {
	pub fn new<T: Clone + Default>(gen: &mut Generator<T>) -> BiomeMap {
		gen.compute_zone_heights();
		gen.compute_climate_widths();
		let mut out = BiomeMap {
			data: TableMap::new(gen.width, gen.height),
		};

		for zone in &gen.zones {
			println!("fdvsa");
			// Fill the zones default biome
			let biome_transition_sampler = NoiseSampler::new(10.0, NoiseKind::Simplex);
			Sweep::zone(gen, zone).apply_sampler(
				&biome_transition_sampler,
				|sweep, x, y, noise| {
					let value = Self::get_y_biome(sweep, x, y, noise, &zone.biome_producer);
					out.data.insert(x, y, value);
				},
			);

			// Generate the zone biomes. (below the climates)
			for biome_id in &zone.child_biomes {
				let biome = &gen.biomes[biome_id.0 as usize];
				Sweep::zone(gen, zone).apply_sampler(&biome.selection_sampler,|sweep, x, y, noise| {
					if Self::sample_biome(sweep, x, y, noise, biome) {
						out.data.insert(x, y, *biome_id);
					}
				});
			}

			for (climate_id, climate_x_range) in &zone.child_climates {
				let climate = &gen.climates[climate_id.0 as usize];

				let x_offset = climate_x_range.start;
				let y_offset = zone.world_range.start;

				let width = climate_x_range.end - x_offset;
				let height = ((zone.world_range.end - y_offset) as f32 * climate.depth) as u32;

				// Fill the climates default biome
				Sweep::climate(gen, zone, climate, climate_x_range).apply_sampler(
					&biome_transition_sampler,
					|sweep, x, y, noise| {
						if climate.shape.inside(
							(x - x_offset) as f32 / width as f32,
							(y - y_offset) as f32 / height as f32,
						) {
							out.data.insert(
								x,
								y,
								Self::get_y_biome(sweep, x, y, noise, &climate.biome_producer),
							);
						}
					},
				);

				// Generate the climate biomes.
				for biome_id in &climate.child_biomes {
					let biome = &gen.biomes[biome_id.0 as usize];
					Sweep::climate(gen, zone, climate, climate_x_range).apply_sampler(&biome.selection_sampler, |sweep, x, y, noise| {
						let y_f = (y - y_offset) as f32 / height as f32;
						let x_f = (x - x_offset) as f32 / width as f32;
						if climate.shape.inside(x_f, y_f) && Self::sample_biome(sweep, x, y, noise, biome)
						{
							out.data.insert(x, y, *biome_id);
						}
					});
				}
			}
		}

		out
	}

	pub(crate) fn get_y_biome<T: Clone + Default>(
		sweep: &Sweep<T>,
		x: u32,
		y: u32,
		noise: f32,
		producer: &BiomeProducer,
	) -> BiomeId {
		let float_y = (y - sweep.min_y()) as f32 / sweep.height() as f32;
		if float_y >= producer.surface_size + producer.surface_transition {
			producer.cave_biome
		} else if float_y >= producer.surface_size {
			let float_y = 1.0 - ((float_y - producer.surface_size) / producer.surface_transition);

			if noise > float_y {
				producer.cave_biome
			} else {
				producer.surface_biome
			}
		} else {
			producer.surface_biome
		}
	}

	pub(crate) fn sample_biome<T: Clone + Default>(
		sweep: &Sweep<T>,
		x: u32,
		y: u32,
		noise: f32,
		biome: &Biome<T>,
	) -> bool {
		let height_range = biome.height_range.clone();

		let float_y = y as f32 / (sweep.generator.height as f32);
		let bias = if height_range.contains(&float_y) {
			1.0
		} else {
			let height = height_range.end - height_range.start;
			let middle = height_range.start + (height / 2.0);
			let middle_distance = (float_y - middle).abs();
			let distance = (middle_distance - (height / 2.0)).max(0.0);
			1.0 - (distance.clamp(0.0, sweep.generator.biome_height_transition)
				/ sweep.generator.biome_height_transition)
		};

		noise <= biome.biome_ratio * bias
	}

	pub fn get_safe(&self, x: u32, y: u32) -> BiomeId {
		*self.data.get(
			x.clamp(0, self.data.width - 1),
			y.clamp(0, self.data.height - 1),
		)
	}
}
