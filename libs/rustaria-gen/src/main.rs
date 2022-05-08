use rand::prelude::SliceRandom;
use std::time::Instant;

use settings::biome::BiomeSettings;
use settings::climate::ClimateSettings;
use settings::zone::ZoneSettings;
use table_map::TableMap;

use crate::biome_map::BiomeMap;
use crate::settings::GenerationSettings;
use crate::ty::{Biome, BiomeId, Climate, ClimateId, Zone, ZoneId};

pub mod biome_map;
pub mod pipeline;
pub mod settings;
pub mod ty;
pub mod table_map;

pub const BATCH_SIZE: usize = 128;

pub struct Generator<T: Clone + Default + Send + Sync> {
	zones: Vec<Zone>,
	climates: Vec<Climate>,
	pub biomes: Vec<Biome<T>>,

	spawn_size: f32,
	biome_height_transition: f32,

	seed: u32,
	// dimensions
	pub width: u32,
	pub height: u32,

	rng: Xoshiro128StarStar,
}

impl<T: Clone + Default + Send + Sync> Generator<T> {
	#[rustfmt::skip]
	pub fn new(
		mut settings: GenerationSettings<T>
	) -> Generator<T> {
		settings.sort();

		// === Create lookup
		let zones_lookup = settings.zone_lookup();
		let climates_lookup= settings.climate_lookup();
		let biomes_lookup = settings.biome_lookup();

		// === Create values
		let mut zones_out = Vec::new();
		let mut climates_out = Vec::new();
		let mut biomes_out = Vec::new();

		// === Zones
		let mut total_height = 0.0;
		for (_, settings) in settings.zones {
			total_height += settings.w_height;
			zones_out.push(Zone::new(settings, &biomes_lookup))
		}
		// Normalize height
		zones_out.iter_mut().for_each(|zone| zone.w_height /= total_height);

		// === Climates
		let mut total_w_width = 1000.0;
		for (_, settings) in settings.climates {
			total_w_width += settings.w_width;
			climates_out.push(Climate::new(settings, &mut zones_out, &zones_lookup, &biomes_lookup, climates_out.len() as u16))
		}
		climates_out.iter_mut().for_each(|zone| zone.w_width /= total_w_width);

		// === Biomes
		for (_, (_, biome)) in settings.biomes.into_iter().enumerate() {
			biomes_out.push(Biome {
				color: biome.label,
				biome_ratio: biome.biome_ratio,
				selection_sampler: biome.selection_sampler,
				height_range: biome.height_range,
				painter: biome.painter,
				zones: biome.zones.iter().map(|zone| {
					let x = *zones_lookup.get(zone).unwrap();
					zones_out[x.0 as usize].child_biomes.push(BiomeId(biomes_out.len() as u16));
					x
				}).collect(),
				climates: biome.climates.iter().map(|zone| {
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
			spawn_size: settings.spawn_size,
			biome_height_transition: settings.biome_height_transition,
			seed: settings.seed,
			width: settings.width,
			height: settings.height,
			rng: Xoshiro128StarStar::seed_from_u64(settings.seed as u64),
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
				self.height - current_height
			} else {
				(self.height as f32 * zone.w_height) as u32
			};

			zone.world_range = current_height..current_height + zone_height;
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

				*width = (self.width as f32 * start) as u32..(self.width as f32 * end) as u32;
			}
		}
	}

	pub fn generate_terrain_map(&mut self, biome_map: &BiomeMap) -> TableMap<T> {
		let mut map = TableMap::new_default(self.width, self.height);

		for zone in &self.zones {
			if zone.terrain_size != 0.0 {
				struct TerrainPassValue<'a, T> {
					baked: Vec<Option<BakedBrush<'a, T>>>,
					slice: TableMapSlice<'a, BiomeId>,
				}

				Pipeline::for_zone(self, zone)
					// Calculate all biomes that are shown in this pass. Bake those biomes and save the biome map in the TerrainPassValue.
					.map_with_map(&biome_map.data, |pass, context, _, map| {
						let mut pass_value = TerrainPassValue::<T> {
							baked: Vec::new(),
							slice: map,
						};

						for _ in 0..context.generator.biomes.len() {
							pass_value.baked.push(None);
						}

						pass_value.slice.for_each(|_, _, value| {
							let idx = value.0 as usize;

							if pass_value.baked[idx].is_none() {
								let baked_brush = context.generator.biomes[idx]
									.painter
									.bake(context.clone(), pass);
								pass_value.baked[idx] = Some(baked_brush);
							}
						});

						pass_value
					})
					// Create the terrain
					.apply(&mut map, |x, y, _, _, value, pass_value| {

						let option = pass_value.baked[pass_value.slice.get(x, y).0 as usize]
							.as_ref()
							.unwrap();
						option.apply(x, y, value)
					})
					.complete();
			}
		}

		map
	}
}

use image::{RgbImage};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro128StarStar;

use rustaria_api::ty::Tag;
use rustaria_common::ty::Direction;

use crate::pipeline::brush::BakedBrush;
use crate::pipeline::sampler::fade::FadeSampler;
use crate::pipeline::sampler::graph::GraphSampler;
use crate::pipeline::sampler::layer::LayerSampler;
use crate::pipeline::sampler::noise::NoiseSampler;
use crate::pipeline::sampler::split::SplitSampler;
use crate::pipeline::Pipeline;
use crate::settings::BiomeProducerSettings;
use crate::ty::ClimateShape;
use table_map::TableMapSlice;
use pipeline::brush::Brush;
use pipeline::sampler::{NoiseKind, Sampler};

fn main() {

	fn zone_biome(
		color: [u8; 3],
		name: &str,
		scale: f32,
		channel: u32,
		octaves: u8,
	) -> (Tag, BiomeSettings<[u8; 3]>) {
		(
			Tag::builtin(name),
			BiomeSettings {
				label: color,
				biome_ratio: 1.0,
				#[rustfmt::skip]
				painter: Brush::layered(vec![
					// Ground
					Brush::noise(
						SplitSampler::new(Direction::Down, 0.0..0.2,
							GraphSampler::new(Direction::Up,
								LayerSampler::new_weighted(vec![
									(10.0, NoiseSampler::new_offset(200.0, NoiseKind::Simplex, 1.0)),
									(10.0, NoiseSampler::new_offset(150.0, NoiseKind::Simplex, 5.0)),
									(1.0, NoiseSampler::new_offset(20.0, NoiseKind::Simplex, 5.0)),
								]),
								Sampler::Const(0.0),
								Sampler::Const(1.0),
							),
							Sampler::Const(0.0)
						),
						vec![Brush::Fill(color), Brush::Fill([0, 0, 0])],
					),
					// Caves
					Brush::noise_weighted(
						FadeSampler::new(Direction::Down,
							LayerSampler::new_weighted(vec![
								(5.0, NoiseSampler::new(50.0, NoiseKind::Simplex)),
								(1.5, NoiseSampler::new(20.0, NoiseKind::Simplex)),
								(1.0, Sampler::Const(0.0)),
							]),
							LayerSampler::new_weighted(vec![
								(5.0, NoiseSampler::new(50.0, NoiseKind::Simplex)),
								(1.5, NoiseSampler::new(20.0, NoiseKind::Simplex)),
							]),
						),
						vec![(1.5, Brush::Ignore), (1.0, Brush::Fill([0, 0, 0]))],
					),
				]),
				height_range: Default::default(),
				zones: vec![],
				climates: vec![],
				selection_sampler: Sampler::Const(0.0),
			},
		)
	}
	let zones = vec![
		(
			Tag::builtin("sky"),
			ZoneSettings {
				w_height: 1000.0,
				priority: 0.0,
				terrain_size: 0.0,
				biome_producer: BiomeProducerSettings {
					surface_size: 0.0,
					surface_transition: 0.0,
					surface_biome: Tag::builtin("sky"),
					cave_biome: Tag::builtin("sky"),
				},
			},
		),
		(
			Tag::builtin("surface"),
			ZoneSettings {
				w_height: 5000.0,
				priority: 0.0,
				terrain_size: 0.1,
				biome_producer: BiomeProducerSettings {
					surface_size: 0.0,
					surface_transition: 0.3,
					surface_biome: Tag::builtin("surface"),
					cave_biome: Tag::builtin("cave"),
				},
			},
		),
		(
			Tag::builtin("underworld"),
			ZoneSettings {
				w_height: 1000.0,
				priority: 0.0,
				// todo noise template
				terrain_size: 1.0,
				biome_producer: BiomeProducerSettings {
					surface_size: 1.0,
					surface_transition: 0.0,
					surface_biome: Tag::builtin("underworld"),
					cave_biome: Tag::builtin("underworld"),
				},
			},
		),
	];
	let climates = vec![
		(
			Tag::builtin("desert"),
			ClimateSettings {
				shape: ClimateShape::Oval { offset_y: 0.2 },
				w_width: 150.0,
				terrain_size: 0.2,
				depth: 0.3,
				biome_producer: BiomeProducerSettings {
					surface_size: 0.0,
					surface_transition: 0.3,
					surface_biome: Tag::builtin("desert_surface"),
					cave_biome: Tag::builtin("desert"),
				},
				zones: vec![Tag::builtin("surface")],
			},
		),
		(
			Tag::builtin("ice"),
			ClimateSettings {
				shape: ClimateShape::Triangle { offset_y: 0.6 },
				w_width: 300.0,
				terrain_size: 0.2,
				depth: 0.6,
				biome_producer: BiomeProducerSettings {
					surface_size: 0.0,
					surface_transition: 0.3,
					surface_biome: Tag::builtin("ice_surface"),
					cave_biome: Tag::builtin("ice"),
				},
				zones: vec![Tag::builtin("surface")],
			},
		),
		(
			Tag::builtin("jungle"),
			ClimateSettings {
				shape: ClimateShape::Rectangle { sheer: 0.2 },
				w_width: 300.0,
				terrain_size: 0.2,
				depth: 1.0,
				biome_producer: BiomeProducerSettings {
					surface_size: 0.0,
					surface_transition: 0.3,
					surface_biome: Tag::builtin("jungle_surface"),
					cave_biome: Tag::builtin("jungle"),
				},
				zones: vec![Tag::builtin("surface")],
			},
		),
	];
	let biomes = vec![
		zone_biome([155, 209, 255], "sky", 1.0, 0, 1),
		zone_biome([128, 128, 128], "cave", 1.0, 0, 1),
		zone_biome([151, 107, 75], "surface", 0.5, 0, 1),
		zone_biome([68, 68, 76], "underworld", 0.25, 0, 1),
		zone_biome([144, 195, 232], "ice", 1.0, 0, 1),
		zone_biome([211, 236, 241], "ice_surface", 1.0, 0, 1),
		zone_biome([212, 192, 100], "desert", 0.5, 0, 1),
		zone_biome([255, 218, 56], "desert_surface", 1.0, 0, 1),
		zone_biome([98, 124, 55], "jungle", 1.0, 0, 1),
		zone_biome([53, 80, 30], "jungle_surface", 1.0, 0, 1),
		(
			Tag::builtin("marble"),
			BiomeSettings {
				label: [168, 178, 204],
				biome_ratio: 0.1,
				painter: Brush::set([168, 178, 204]),
				height_range: 0.6..0.8,
				zones: vec![Tag::builtin("surface")],
				climates: vec![Tag::builtin("ice"), Tag::builtin("jungle")],
				selection_sampler: NoiseSampler::new_offset(400.0, NoiseKind::Simplex, 1.0),
			},
		),
		(
			Tag::builtin("granite"),
			BiomeSettings {
				label: [50, 46, 104],
				biome_ratio: 0.1,
				painter: Brush::set([50, 46, 104]),
				height_range: 0.5..0.8,
				zones: vec![Tag::builtin("surface")],
				climates: vec![Tag::builtin("ice")],
				selection_sampler: NoiseSampler::new_offset(350.0, NoiseKind::Simplex, 2.0),
			},
		),
		(
			Tag::builtin("beez"),
			BiomeSettings {
				label: [248, 166, 2],
				biome_ratio: 0.125,
				painter: Brush::set([248, 166, 2]),
				height_range: 0.6..0.8,
				zones: vec![],
				climates: vec![Tag::builtin("jungle")],
				selection_sampler: NoiseSampler::new_offset(400.0, NoiseKind::Simplex, 3.0),
			},
		),
		(
			Tag::builtin("mushroom"),
			BiomeSettings {
				label: [93, 127, 255],
				biome_ratio: 0.07,
				painter: Brush::set([93, 127, 255]),
				height_range: 0.6..0.8,
				zones: vec![Tag::builtin("surface")],
				climates: vec![],
				selection_sampler: NoiseSampler::new_offset(800.0, NoiseKind::Simplex, 4.0),
			},
		),
		(
			Tag::builtin("sky_island"),
			BiomeSettings {
				label: [223, 255, 255],
				biome_ratio: 0.5,
				painter: Brush::set([223, 255, 255]),
				height_range: 0.0..1.0,
				zones: vec![Tag::builtin("sky")],
				climates: vec![],
				selection_sampler: NoiseSampler::new_offset(200.0, NoiseKind::Simplex, 5.0),
			},
		),
	];

	let mut generator = Generator::new(GenerationSettings {
		zones,
		climates,
		biomes,

		spawn_size: 0.1,
		biome_height_transition: 0.2,
		seed: 69,
		width: 6400,
		height: 1800,
	});

	let start = Instant::now();
	let start_biome = Instant::now();
	let biome_map = BiomeMap::new(&mut generator);
	println!("[Biome Map] {}ms", start_biome.elapsed().as_millis());

//let mut image = RgbImage::new(generator.width, generator.height);

//biome_map.data.for_each(|x, y, value| {
//	image.put_pixel(x, y, Rgb(generator.biomes[value.0 as usize].color));
//});

//image.save("biomes.png").unwrap();

	let start_biome = Instant::now();
	let terrain_map = generator.generate_terrain_map(&biome_map);
	println!("[Terrain Map] {}ms", start_biome.elapsed().as_millis());
	println!("[Total] {}ms", start.elapsed().as_millis());

	println!("Exporting map");

	let vec1: Vec<u8> = terrain_map.data.into_iter().flatten().collect();
	let image = RgbImage::from_vec(generator.width, generator.height, vec1).unwrap();
	println!("Saving");
	image.save("terrain.png").unwrap();
}
