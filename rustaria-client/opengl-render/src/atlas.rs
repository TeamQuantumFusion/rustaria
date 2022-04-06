use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, GenericImageView};
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    RectanglePackError, TargetBin,
};
use tracing::info;

use crate::texture::{
    InternalFormat, TextureData, TextureDataFormat, TextureDescriptor, TextureLod,
    TextureMagFilter, TextureMinFilter, TextureType,
};
use crate::Texture;

pub struct AtlasBuilder<T: Hash + Ord + Clone> {
    images: Vec<(T, DynamicImage)>,
}

impl<T: Hash + Ord + Clone> AtlasBuilder<T> {
    pub fn new() -> AtlasBuilder<T> {
        AtlasBuilder { images: vec![] }
    }

    pub fn push(&mut self, tag: T, image: DynamicImage) {
        self.images.push((tag, image));
    }

    pub fn export(self, levels: u8) -> Atlas<T> {
        // Pack everything
        info!(target: "opengl", "Packing atlas.");
        let mut rects_to_place = GroupedRectsToPlace::new();

        for (id, (_, image)) in self.images.iter().enumerate() {
            rects_to_place.push_rect(
                id as u32,
                Some(vec![0u8]),
                RectToInsert::new(image.width(), image.height(), 1),
            );
        }

        let mut rectangle_placements = Err(RectanglePackError::NotEnoughBinSpace);
        let mut max_width = 8;
        let mut max_height = 8;
        while let Err(RectanglePackError::NotEnoughBinSpace) = rectangle_placements {
            max_width *= 2;
            max_height *= 2;

            let mut target_bins = BTreeMap::new();
            target_bins.insert(0, TargetBin::new(max_width, max_height, 1));
            rectangle_placements = pack_rects(
                &rects_to_place,
                &mut target_bins,
                &volume_heuristic,
                &contains_smallest_box,
            );
        }

        info!(target: "opengl", "Creating atlas");
        // Create image and lookup
        let pack = rectangle_placements.unwrap();
        let locations = pack.packed_locations();
        let mut image = image::DynamicImage::new_rgba8(max_width, max_height);
        let mut lookup = HashMap::new();
        for (id, (_, location)) in locations {
            let (tag, source) = &self.images[*id as usize];
            lookup.insert(
                tag.clone(),
                AtlasLocation {
                    x: location.x() as f32 / max_width as f32,
                    y: location.y() as f32 / max_height as f32,
                    width: location.width() as f32 / max_width as f32,
                    height: location.height() as f32 / max_height as f32,
                },
            );
            let x_offset = location.x();
            let y_offset = location.y();
            for y in 0..location.height() {
                for x in 0..location.width() {
                    image.put_pixel(x_offset + x, y_offset + y, source.get_pixel(x, y));
                }
            }
        }

        // FIXME(leocth) save it in the run dir
        image.save("./atlas.png");

        info!(target: "opengl", "Mipmapping atlas");
        // Generate Mipmaps
        let mut images = Vec::new();
        for level in 0..levels {
            let image = image.resize(
                image.width() >> level as u32,
                image.height() >> level as u32,
                FilterType::Nearest,
            );
            images.push(TextureData {
                texture_data: image.into_bytes(),
                texture_format: TextureDataFormat::Rgba,
            });
        }

        info!(target: "opengl", "Uploading atlas");
        // upload
        let texture = Texture::new(
            TextureType::Texture2d {
                images: Some(images),
                internal: InternalFormat::Rgba,
                width: max_width,
                height: max_height,
                border: 0,
            },
            TextureDescriptor {
                lod: TextureLod {
                    max_level: levels as i32,
                    lod_bias: 0.1,
                    min: 0.0,
                    max: 1.0,
                },
                min_filter: TextureMinFilter::Mipmap(
                    crate::texture::FilterType::Nearest,
                    crate::texture::FilterType::Linear,
                ),
                mag_filter: TextureMagFilter(crate::texture::FilterType::Nearest),
                ..Default::default()
            },
        );

        info!("Created atlas {}x{}", max_width, max_height);
        Atlas { texture, lookup }
    }
}

pub struct Atlas<T: Hash + Ord> {
    pub texture: Texture,
    pub lookup: HashMap<T, AtlasLocation>,
}

#[derive(Copy, Clone)]
pub struct AtlasLocation {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
