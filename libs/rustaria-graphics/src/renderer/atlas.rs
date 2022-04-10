use std::collections::HashSet;

use image::{DynamicImage, ImageFormat};

use aloy::atlas::{Atlas, AtlasBuilder, AtlasLocation};
use rustaria::api::Api;
use rustaria_api::plugin::archive::ArchivePath;
use rustaria_api::tag::Tag;
use rustaria_util::{eyre, Result, warn, WrapErr};

pub struct TextureAtlas {
	atlas: Atlas<Tag>
}

impl TextureAtlas {
	pub fn new(api: &Api, sprites: HashSet<Tag>) -> TextureAtlas {
		let mut builder = AtlasBuilder::new();
		for tag in sprites {
			match load_sprite(api, &tag) {
				Ok(image) => {
					builder.push(tag, image);
				}
				Err(erro) => {
					warn!("Could not load sprite {:?} {}", tag, erro);
				}
			}
		}


		TextureAtlas {
			atlas: builder.export(4)
		}
	}


	pub fn get(&self, tag: &Tag) -> Option<AtlasLocation>  {
		self.atlas.lookup.get(tag).cloned()
	}

}



fn load_sprite(api: &Api, tag: &Tag) -> Result<DynamicImage> {
	let instance = api.instance();
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