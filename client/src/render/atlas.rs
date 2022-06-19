use std::collections::{BTreeMap, HashMap, HashSet};

use euclid::{point2, size2, Rect};
use eyre::Result;
use glium::texture::RawImage2d;
use image::imageops::FilterType;
use rectangle_pack::{
	pack_rects, GroupedRectsToPlace, RectToInsert, RectanglePackError, TargetBin,
};
use rustaria::{
	api::{Api, ResourceKind},
	ty::identifier::Identifier,
};
use tracing::{error, trace, warn};

use crate::Frontend;

pub struct Atlas {
	pub width: u32,
	pub height: u32,
	pub texture: glium::texture::SrgbTexture2d,
	lookup: HashMap<Identifier, Rect<f32, Atlas>>,
}

impl Atlas {
	pub fn get(&self, location: &Identifier) -> Rect<f32, Atlas> {
		*self.lookup.get(location).unwrap_or_else(|| {
			self.lookup
				.get(&Identifier::new("missing"))
				.expect("Missing is missing. ffs")
		})
	}

	pub fn new(
		frontend: &Frontend,
		api: &Api,
		image_locations: HashSet<Identifier>,
	) -> Result<Atlas> {
		let mut images = HashMap::new();
		images.insert(
			Identifier::new("missing"),
			image::load_from_memory(include_bytes!("../builtin/missing.png"))?,
		);

		// Load all images
		for location in image_locations {
			if let Ok(image) = api.resources.get_resource(ResourceKind::Assets, &location) {
				match image::load_from_memory(&image) {
					Ok(image) => {
						images.insert(location, image);
					}
					Err(error) => {
						error!("Could not load image {location}: {}", error);
					}
				}
			} else {
				warn!("Could not find image {}", location);
			}
		}

		let mut packing_setup = GroupedRectsToPlace::new();
		for (tag, image) in &images {
			packing_setup.push_rect(
				tag.clone(),
				Some(vec![0u8]),
				RectToInsert::new(image.width(), image.height(), 1),
			);
		}

		let mut width = 128;
		let mut height = 128;
		let placements = loop {
			let mut target_bins = BTreeMap::new();
			target_bins.insert(0, TargetBin::new(width, height, 1));
			match pack_rects(
				&packing_setup,
				&mut target_bins,
				&rectangle_pack::volume_heuristic,
				&rectangle_pack::contains_smallest_box,
			) {
				Err(RectanglePackError::NotEnoughBinSpace) => {
					width *= 2;
					height *= 2;
					trace!("Expanding atlas to {}x{}", width, height);
				}
				Ok(placements) => {
					break placements;
				}
			}
		};

		let texture = glium::texture::SrgbTexture2d::empty_with_mipmaps(
			&frontend.ctx,
			glium::texture::MipmapsOption::EmptyMipmapsMax(3),
			width,
			height,
		)?;

		let mut lookup = HashMap::new();
		for (identifier, (_, location)) in placements.packed_locations() {
			let gl_pos = Rect::new(
				point2(
					location.x() as f32 / width as f32,
					location.y() as f32 / height as f32,
				),
				size2(
					location.width() as f32 / width as f32,
					location.height() as f32 / height as f32,
				),
			);

			lookup.insert(identifier.clone(), gl_pos);

			// Add to opengl
			let image = images.remove(identifier).unwrap();
			for level in 0..4 {
				if let Some(mipmap) = texture.mipmap(level) {
					let left = location.x() >> level;
					let bottom = location.y() >> level;
					let width = location.width() >> level;
					let height = location.height() >> level;

					let image = image.resize_exact(width, height, FilterType::Nearest);
					mipmap.write(
						glium::Rect {
							left,
							bottom,
							width,
							height,
						},
						RawImage2d::from_raw_rgba_reversed(
							image.to_rgba8().as_ref(),
							(width, height),
						),
					);
				}
			}
		}

		Ok(Atlas {
			width,
			height,
			texture,
			lookup,
		})
	}
}
