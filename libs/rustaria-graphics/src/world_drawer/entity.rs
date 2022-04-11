use std::collections::HashMap;

use crate::renderer::atlas::TextureAtlas;
use aloy::atlas::AtlasLocation;
use aloy::vertex::VertexBuilder;
use rustaria::api::prototype::entity::EntityPrototype;
use rustaria::api::rendering::{Pane, RenderingSystem};
use rustaria_api::lua_runtime::UserData;
use crate::Pos;
use crate::ty::{Rectangle, Texture};

pub struct EntityRenderingSystemDrawer {
    rendering: EntityRendering,
}

impl EntityRenderingSystemDrawer {
    pub fn new(rendering_system: &RenderingSystem, atlas: &TextureAtlas) -> Option<EntityRenderingSystemDrawer> {
        let rendering = match rendering_system {
            RenderingSystem::Static(pane) => {
                EntityRendering::Static(RenderingPane::new(pane, atlas)?)
            }
            RenderingSystem::State(states) => {
	            let mut out = HashMap::new();
	            for (state, pane) in states {
		            out.insert(state.clone(), RenderingPane::new(pane, atlas)?);
	            }

	            EntityRendering::State(
		            out
	            )
            },
        };

	    Some(EntityRenderingSystemDrawer {
		    rendering
	    })
    }

	// todo take in the actual entity somehow to get its state
	pub fn push(&self, builder: &mut VertexBuilder<(Pos, Texture)>, pos: (f32, f32)) {
		match &self.rendering {
			EntityRendering::Static(pane) => {
				pane.push(builder, pos);
			}
			EntityRendering::State(_) => {
				todo!("state rendering")
			}
		}
	}
}

pub enum EntityRendering {
    Static(RenderingPane),
    State(HashMap<String, RenderingPane>),
    // More implementations for dynamic lua rendering.
    // Advanced(stuff)
}

pub struct RenderingPane {
    pub x_offset: f32,
    pub y_offset: f32,
    pub width: f32,
    pub height: f32,
    pub image: AtlasLocation,
}

impl RenderingPane {
    pub fn new(pane: &Pane, atlas: &TextureAtlas) -> Option<RenderingPane> {
        Some(RenderingPane {
            x_offset: pane.x_offset,
            y_offset: pane.y_offset,
            width: pane.width,
            height: pane.height,
            image: atlas.get(&pane.sprite)?,
        })
    }

	pub fn push(&self, builder: &mut VertexBuilder<(Pos, Texture)>, pos: (f32, f32)) {
		builder.quad((
			Rectangle {
				x: self.x_offset + pos.0,
				y: self.y_offset + pos.1,
				w: self.width,
				h: self.height,
			},
			Rectangle::from(self.image),
		));
	}
}
