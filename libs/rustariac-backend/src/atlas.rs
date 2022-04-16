use std::collections::{BTreeMap, HashMap, HashSet};

use eyre::Result;
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat};
use rectangle_pack::{
	contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, PackedLocation,
	RectToInsert, RectanglePackError, RectanglePackOk, TargetBin,
};
use rustaria_api::ty::Tag;
use rustaria_api::Api;
use rustaria_util::warn;

use crate::ty::{AtlasLocation, Rectangle};

#[derive(Default)]
pub struct Atlas {
	lookup: HashMap<Tag, AtlasLocation>,
	missing_tag: Tag,
	width: u32,
	height: u32,
}

impl Atlas {
	pub fn get(&self, tag: &Tag) -> AtlasLocation {
		match self.lookup.get(tag) {
			Some(&value) => value,
			None => self.lookup.get(&self.missing_tag).cloned().unwrap(),
		}
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}
}

pub fn build_atlas(
	api: &Api,
	sprites: HashSet<Tag>,
) -> (Atlas, Vec<(DynamicImage, AtlasLocation)>) {
	let missing_tag = Tag::new("core:missing".to_string()).unwrap();
	let mut images: HashMap<Tag, DynamicImage> = HashMap::new();

	// Load all of the spritesfor tag in sprites
	{
		for tag in sprites {
			match load_sprite(api, &tag) {
				Ok(image) => {
					images.insert(tag, image);
				}
				Err(error) => {
					warn!("Could not load sprite {} {}", tag, error);
				}
			}
		}
	}

	// Insert builtin sprites.
	images.insert(
		missing_tag.clone(),
		image::load_from_memory(include_bytes!("./missing.png")).unwrap(),
	);

	// Setup packing
	let mut packing_setup: GroupedRectsToPlace<Tag, Option<u8>> = GroupedRectsToPlace::new();

	for (id, (tag, image)) in images.iter().enumerate() {
		packing_setup.push_rect(
			tag.clone(),
			None,
			RectToInsert::new(image.width(), image.height(), 1),
		);
	}

	// Try to insert, if it does not have enough space double the atlas size.let mut rectangle_placements = Err(RectanglePackError::NotEnoughBinSpace);
	let mut rectangle_placements = Err(RectanglePackError::NotEnoughBinSpace);
	let mut max_width = 128;
	let mut max_height = 128;
	while let Err(RectanglePackError::NotEnoughBinSpace) = rectangle_placements {
		max_width *= 2;
		max_height *= 2;

		let mut target_bins = BTreeMap::new();
		target_bins.insert(0, TargetBin::new(max_width, max_height, 1));
		rectangle_placements = pack_rects(
			&packing_setup,
			&mut target_bins,
			&volume_heuristic,
			&contains_smallest_box,
		);
	}

	// Create image and lookup
	let mut lookup = HashMap::new();
	let mut images_out = Vec::new();
	for (tag, (_, location)) in rectangle_placements.unwrap().packed_locations() {
		let image = images.remove(tag).unwrap();
		lookup.insert(
			(*tag).clone(),
			AtlasLocation {
				x: location.x() as f32 / max_width as f32,
				y: location.y() as f32 / max_height as f32,
				width: location.width() as f32 / max_width as f32,
				height: location.height() as f32 / max_height as f32,
			},
		);

		images_out.push((
			DynamicImage::from(image.to_rgba8()),
			AtlasLocation {
				x: location.x() as f32,
				y: location.y() as f32,
				width: location.width() as f32,
				height: location.height() as f32,
			},
		));
	}

	(
		Atlas {
			lookup,
			missing_tag,
			width: max_width,
			height: max_height,
		},
		images_out,
	)
}

fn load_sprite(api: &Api, tag: &Tag) -> Result<DynamicImage> {
	Ok(image::load_from_memory_with_format(
		&api.get_asset(tag)?,
		ImageFormat::Png,
	)?)
}
