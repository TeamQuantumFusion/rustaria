use crate::pipeline::context::Context;
use crate::pipeline::sampler::{BakedSampler, NoiseKind};
use crate::pipeline::Pipeline;
use crate::settings::BiomeProducer;
use crate::{Biome, BiomeId, Generator, NoiseSampler, TableMap};

pub struct BiomeMap {
	pub data: TableMap<BiomeId>,
}

impl BiomeMap {
	pub fn new<T: Clone + Default + Send + Sync>(gen: &mut Generator<T>) -> BiomeMap {
		gen.compute_zone_heights();
		gen.compute_climate_widths();
		let mut out = BiomeMap {
			data: TableMap::new_default(gen.width, gen.height),
		};

		for zone in &gen.zones {
			// Fill the zones default biome
			let biome_transition_sampler = NoiseSampler::new(10.0, NoiseKind::Simplex);

			Pipeline::for_zone(gen, zone)
				.map(|pass, ctx, _| biome_transition_sampler.bake(ctx, pass))
				.apply(&mut out.data, |x, y, _, ctx, value, sampler| {
					*value = Self::get_y_biome(&ctx, x, y, sampler, &zone.biome_producer);
				})
				.complete();

			// Generate the zone biomes. (below the climates)
			for biome_id in &zone.child_biomes {
				let biome = &gen.biomes[biome_id.0 as usize];
				Pipeline::for_zone(gen, zone)
					.map(|pass, ctx, _| biome.selection_sampler.bake(ctx, pass))
					.apply(&mut out.data, |x, y, _, ctx, value, sampler| {
						if Self::sample_biome(&ctx, x, y, sampler, biome) {
							*value = *biome_id;
						}
					})
					.complete();
			}

			for (climate_id, climate_x_range) in &zone.child_climates {
				let climate = &gen.climates[climate_id.0 as usize];

				let x_offset = climate_x_range.start;
				let y_offset = zone.world_range.start;

				let width = climate_x_range.end - x_offset;
				let height = ((zone.world_range.end - y_offset) as f32 * climate.depth) as u32;

				// Generate the default biome.
				Pipeline::for_climate(gen, zone, climate, climate_x_range)
					.map(|pass, ctx, _| biome_transition_sampler.bake(ctx, pass))
					.apply(&mut out.data, |x, y, _, ctx, value, sampler| {
						if climate.shape.inside(
							(x - x_offset) as f32 / width as f32,
							(y - y_offset) as f32 / height as f32,
						) {
							*value =
								Self::get_y_biome(&ctx, x, y, sampler, &climate.biome_producer);
						}
					})
					.complete();

				// Generate the climate biomes.
				for biome_id in &climate.child_biomes {
					let biome = &gen.biomes[biome_id.0 as usize];
					Pipeline::for_climate(gen, zone, climate, climate_x_range)
						.map(|pass, ctx, _| biome.selection_sampler.bake(ctx, pass))
						.apply(&mut out.data, |x, y, _, ctx, value, sampler| {

							let y_f = (y - y_offset) as f32 / height as f32;
							let x_f = (x - x_offset) as f32 / width as f32;
							if climate.shape.inside(x_f, y_f)
								&& Self::sample_biome(&ctx, x, y, sampler, biome)
							{
								*value = *biome_id;
							}
						})
						.complete();
				}
			}
		}

		out
	}

	pub(crate) fn get_y_biome<T: Clone + Default + Send + Sync>(
		ctx: &Context<T>,
		x: u32,
		y: u32,
		sampler: &BakedSampler,
		producer: &BiomeProducer,
	) -> BiomeId {
		let float_y = (y - ctx.min_y()) as f32 / ctx.height() as f32;
		if float_y >= producer.surface_size + producer.surface_transition {
			producer.cave_biome
		} else if float_y >= producer.surface_size {
			let float_y = 1.0 - ((float_y - producer.surface_size) / producer.surface_transition);

			if sampler.get(x, y) > float_y {
				producer.cave_biome
			} else {
				producer.surface_biome
			}
		} else {
			producer.surface_biome
		}
	}

	pub(crate) fn sample_biome<T: Clone + Default + Send + Sync>(
		ctx: &Context<T>,
		x: u32,
		y: u32,
		sampler: &BakedSampler,
		biome: &Biome<T>,
	) -> bool {
		let height_range = biome.height_range.clone();

		let float_y = y as f32 / (ctx.generator.height as f32);
		let bias = if height_range.contains(&float_y) {
			1.0
		} else {
			let height = height_range.end - height_range.start;
			let middle = height_range.start + (height / 2.0);
			let middle_distance = (float_y - middle).abs();
			let distance = (middle_distance - (height / 2.0)).max(0.0);
			1.0 - (distance.clamp(0.0, ctx.generator.biome_height_transition)
				/ ctx.generator.biome_height_transition)
		};

		sampler.get(x, y) < biome.biome_ratio * bias
	}

	pub fn get_safe(&self, x: u32, y: u32) -> BiomeId {
		*self.data.get(
			x.clamp(0, self.data.width - 1),
			y.clamp(0, self.data.height - 1),
		)
	}
}
