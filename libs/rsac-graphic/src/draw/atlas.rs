use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

use glium::backend::Context;
use glium::texture::RawImage2d;
use glium::uniforms::Sampler;
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, RgbaImage};
use rectangle_pack::{
	contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
	RectanglePackError, TargetBin,
};

use rsa_core::api::{Api, AssetKind};
use rsa_core::error::Result;
use rsa_core::logging::{debug, trace, warn};
use rsa_core::math::{point2, size2, AtlasSpace, Rect};
use rsa_core::ty::Tag;

/// The renderatlas is an opengl image containing the image, reload will return the Atlas lookup for compiling meshes and such.
pub(crate) struct DrawAtlas {
	width: u32,
	height: u32,
	texture: Option<glium::texture::SrgbTexture2d>,
	lookup: HashMap<Tag, (Rect<f32, AtlasSpace>, Rect<u32, AtlasSpace>)>,
}

impl DrawAtlas {
	pub fn new() -> DrawAtlas {
		DrawAtlas {
			width: 0,
			height: 0,
			texture: None,
			lookup: Default::default(),
		}
	}

	pub fn get(&self, tag: &Tag) -> Rect<f32, AtlasSpace> {
		match self.lookup.get(tag) {
			Some(&value) => value.0,
			None => self.lookup.get(&Tag::rsa("missing")).cloned().unwrap().0,
		}
	}

	pub(crate) fn sampler<'a>(&'a self) -> Sampler<'a, glium::texture::SrgbTexture2d> {
		Sampler::new(self.texture.as_ref().unwrap())
	}

	pub fn reload(
		&mut self,
		api: &Api,
		context: &Rc<Context>,
		sprites: HashSet<Tag>,
	) -> Result<()> {
		assert!(!sprites.is_empty());
		let mut images = Self::load_images(api, sprites);
		self.compile_images(&images);

		self.texture = Some(glium::texture::SrgbTexture2d::empty_with_mipmaps(
			context,
			glium::texture::MipmapsOption::EmptyMipmapsMax(3),
			self.width,
			self.height,
		)?);

		for (image, location) in &self.lookup {
			let image = DynamicImage::from(images.remove(image).unwrap());
			for level in 0..4 {
				if let Some(mipmap) = self.texture.as_ref().unwrap().mipmap(level) {
					let left = location.1.min_x() >> level;
					let bottom = location.1.min_y() >> level;
					let width = location.1.width() >> level;
					let height = location.1.height() >> level;

					let image = image.resize_exact(width, height, FilterType::Nearest);
					mipmap.write(
						glium::Rect {
							left,
							bottom,
							width,
							height,
						},
						RawImage2d::from_raw_rgba(image.to_rgba8().to_vec(), (width, height)),
					);
				}
			}
		}

		Ok(())
	}

	fn compile_images(&mut self, images: &HashMap<Tag, RgbaImage>) {
		debug!("Compiling {} images", images.len());

		let mut packing_setup = GroupedRectsToPlace::new();
		for (tag, image) in images {
			trace!("Adding image {tag}, {}x{}", image.width(), image.height());
			packing_setup.push_rect(
				tag.clone(),
				Some(vec![0u8]),
				RectToInsert::new(image.width(), image.height(), 1),
			);
		}

		// Try to insert, if it does not have enough space double the atlas size.let mut rectangle_placements = Err(RectanglePackError::NotEnoughBinSpace);
		self.width = 128;
		self.height = 128;
		let placements = loop {
			let mut target_bins = BTreeMap::new();
			target_bins.insert(0, TargetBin::new(self.width, self.height, 1));
			match pack_rects(
				&packing_setup,
				&mut target_bins,
				&volume_heuristic,
				&contains_smallest_box,
			) {
				Err(RectanglePackError::NotEnoughBinSpace) => {
					self.width *= 2;
					self.height *= 2;
					trace!(
						"Trying to pack again on size {}x{}",
						self.width,
						self.height
					);
				}
				Ok(placements) => {
					break placements;
				}
			}
		};

		self.lookup.clear();
		for (tag, (_, location)) in placements.packed_locations() {
			trace!("Found image {tag} at {location:?}");

			let pos = Rect::new(
				point2(location.x(), location.y()),
				size2(location.width(), location.height()),
			);

			let gl_pos = Rect::new(
				point2(
					location.x() as f32 / self.width as f32,
					location.y() as f32 / self.height as f32,
				),
				size2(
					location.width() as f32 / self.width as f32,
					location.height() as f32 / self.height as f32,
				),
			);

			self.lookup.insert(tag.clone(), (gl_pos, pos));
		}
	}

	fn load_images(api: &Api, sprites: HashSet<Tag>) -> HashMap<Tag, RgbaImage> {
		let mut images = HashMap::new();
		images.insert(
			Tag::rsa("missing"),
			image::load_from_memory(include_bytes!("../builtin/missing.png"))
				.unwrap()
				.to_rgba8(),
		);

		for location in sprites {
			match Self::load_image(api, &location) {
				Ok(image) => {
					trace!("Loaded image {location}");
					images.insert(location, image);
				}
				Err(error) => {
					warn!(target: "init@rustariac.backend", "Could not find sprite {} {}", location, error);
				}
			}
		}

		images
	}

	fn load_image(api: &Api, location: &Tag) -> Result<RgbaImage> {
		Ok(image::load_from_memory_with_format(
			&api.get_asset(AssetKind::Asset, location)?,
			ImageFormat::Png,
		)?
		.to_rgba8())
	}
}
