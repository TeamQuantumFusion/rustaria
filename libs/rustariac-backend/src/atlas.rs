use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::RwLockReadGuard;

use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat};
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    RectanglePackError, TargetBin,
};
use rustaria_util::{Context, eyre, Result, warn};
use rustaria::api::Api;
use rustaria_api::ApiHandler;
use rustaria_api::plugin::archive::ArchivePath;
use rustaria_api::tag::Tag;

use crate::ty::AtlasLocation;

#[derive(Default)]
pub struct Atlas {
    lookup: HashMap<Tag, AtlasLocation>,
    missing_tag: Tag,
}

impl Atlas {
    pub fn get(&self, tag: &Tag) -> AtlasLocation {
        match self.lookup.get(tag) {
            Some(value) => value.clone(),
            None => {
                self.lookup.get(&self.missing_tag).cloned().unwrap()
            }
        }
    }
}

pub fn build_atlas(api: &Api, sprites: HashSet<Tag>) -> (Atlas, DynamicImage) {
    let missing_tag = Tag::from_string("core:missing".to_string()).unwrap();
    let mut images: Vec<(Tag, DynamicImage)> = Vec::new();

    let instance = api.instance();
    for tag in sprites {
        match load_sprite(&instance, &tag) {
            Ok(image) => {
                images.push((tag, image));
            }
            Err(error) => {
                warn!("Could not load sprite {} {}", tag, error);
            }
        }
    }

    images.push((missing_tag.clone(), image::load_from_memory(include_bytes!("./missing.png")).unwrap()));



    let mut rects_to_place = GroupedRectsToPlace::new();

    for (id, (_, image)) in images.iter().enumerate() {
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

    // Create image and lookup
    let pack = rectangle_placements.unwrap();
    let locations = pack.packed_locations();
    let mut lookup = HashMap::new();
    let mut image = image::DynamicImage::new_rgba8(max_width, max_height);
    for (id, (_, location)) in locations {
        let (tag, source) = &images[*id as usize];
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

    (Atlas { lookup, missing_tag }, image)
}

fn load_sprite(instance: &RwLockReadGuard<ApiHandler>, tag: &Tag) -> Result<DynamicImage> {
    let plugin = instance.get_plugin(tag.plugin_id()).ok_or_else(|| {
        eyre!(
                "Plugin {} does not exist or is not loaded.",
                tag.plugin_id()
            )
    })?;
    let data = plugin
        .archive
        .get_asset(&ArchivePath::Asset(tag.name().to_string()))
        .wrap_err(format!("Sprite does not exist {}", tag))?;
    Ok(image::load_from_memory_with_format(data, ImageFormat::Png)?)
}
