use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use glium::backend::Context;
use glium::Program;

use rsa_core::api::{Api};
use rustaria::prototypes;
use rsa_core::ty::Prototype;
use rsa_core::error::{ContextCompat, Result, WrapErr};

use crate::Internals;
use crate::camera::Camera;
use crate::draw::atlas::DrawAtlas;

pub mod atlas;
pub mod buffer;

/// Our belowed Drawer, is responsible for drawing the game at hyper fast speeds. Here are the reasons why its so fast.
/// 1. Its a 2d game which is really easy to cull/render and we take the full advantage of that.
/// 2. Its terrarias main class we are comparing to. EMOTIONAL DAMAGE!!
pub(crate) struct Drawer {
	pub(crate) context: Rc<Context>,
	pub(crate) internals: Rc<Internals>,
	// We kinda only have 1 atlas because a lot of what we do is really small
	// and everything can fit on a single image.
	pub(crate) atlas: DrawAtlas,
	programs: HashMap<String, Program>,

	// Parameters
	pub(crate) screen_ratio: f32,
	pub(crate) camera: Camera,
}

impl Drawer {
	pub(crate) fn new(internals: Rc<Internals>) -> Drawer {
		Drawer {
			context: unsafe {
				Context::new(internals.clone(), true, Default::default())
			}.expect("Critical Failure when creating opengl context."),
			internals,
			atlas: DrawAtlas::new(),
			programs: Default::default(),
			screen_ratio: 0.0,
			camera: Camera {
				pos: Default::default(),
				scale: 0.0
			},
		}
	}

	pub(crate) fn load_program(&mut self, name: &str, program: Program) {
		self.programs.insert(name.to_string(), program);
	}

	pub(crate) fn get_program(&self, name: &str) -> Result<&Program> {
		self.programs.get(name).wrap_err("Cannot find program")
	}

	pub(crate) fn resize(&mut self, width: u32, height: u32) {
		self.screen_ratio = width as f32 / height as f32;
	}

	pub(crate) fn reload(&mut self, api: &Api) -> Result<()> {
		let carrier = api.get_carrier();
		let mut sprites = HashSet::new();

		prototypes!({
			for prototype in carrier.get::<P>().iter() {
				prototype.get_sprites(&mut sprites);
			}
		});
		self.atlas.reload(api, &self.context, sprites).wrap_err("Failed to create atlas")?;

		Ok(())
	}
}