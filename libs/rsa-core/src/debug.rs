use bitflags::bitflags;
use euclid::{rect, vec2, Rect, Vector2D};

use crate::ty::WS;

pub trait DebugRendererImpl {
	fn event(&mut self, call: DebugEvent);
}

pub struct DummyRenderer;

impl DebugRendererImpl for DummyRenderer {
	fn event(&mut self, _: DebugEvent) {}
}

bitflags! {
	pub struct DebugCategory: u32 {
		const Tile = 1 << 0;
		const Temporary = 1 << 1;
		const TileSpread = 1 << 2;
		const ChunkBorders = 1 << 3;
		const ChunkMeshing = 1 << 4;
		const EntityVelocity = 1 << 5;
		const EntityCollision = 1 << 6;
	}
}

#[derive(Copy, Clone)]
pub enum DebugDraw {
	Quad(Rect<f32, WS>),
	Line {
		start: Vector2D<f32, WS>,
		stop: Vector2D<f32, WS>,
	},
	Point(Vector2D<f32, WS>),
}

impl From<Rect<f32, WS>> for DebugDraw {
	fn from(quad: Rect<f32, WS>) -> Self { DebugDraw::Quad(quad) }
}

impl From<(Vector2D<f32, WS>, Vector2D<f32, WS>)> for DebugDraw {
	fn from((start, stop): (Vector2D<f32, WS>, Vector2D<f32, WS>)) -> Self {
		DebugDraw::Line { start, stop }
	}
}

impl From<Vector2D<f32, WS>> for DebugDraw {
	fn from(pos: Vector2D<f32, WS>) -> Self { DebugDraw::Point(pos) }
}

pub struct DebugEvent {
	pub category: DebugCategory,
	pub draw: DebugDraw,
	pub color: u32,
	pub line_size: f32,
	pub duration: u32,
	pub ticks_remaining: u32,
}

#[macro_export]
macro_rules! draw_debug {
	($DEBUG:expr, $CATEGORY:expr, $DRAW:expr, $COLOR:expr, $LINE_SIZE:expr, $TIME:literal) => {
		$crate::debug::DebugRendererImpl::event(
			$DEBUG,
			$crate::debug::DebugEvent {
				category: $CATEGORY,
				draw: $DRAW.into(),
				color: $COLOR,
				line_size: $LINE_SIZE,
				duration: ($TIME * $crate::TPS as f32) as u32,
				ticks_remaining: ($TIME * $crate::TPS as f32) as u32,
			},
		);
	};
	($DEBUG:expr, $CATEGORY:expr, $DRAW:expr, $COLOR:expr, $LINE_SIZE:expr) => {
		$crate::draw_debug!($DEBUG, $CATEGORY, $DRAW, $COLOR, $LINE_SIZE, 0.0)
	};
	($DEBUG:expr, $CATEGORY:expr, $DRAW:expr, $COLOR:expr) => {
		$crate::draw_debug!($DEBUG, $CATEGORY, $DRAW, $COLOR, 1.0)
	};
	($DEBUG:expr, $CATEGORY:expr, $DRAW:expr) => {
		$crate::draw_debug!($DEBUG, $CATEGORY, $DRAW, 0xc1c0c0)
	};
}
impl DebugCategory {
	pub fn tile(debug: &mut impl DebugRendererImpl) {
		draw_debug!(debug, DebugCategory::Tile, (vec2(0.0, 0.0), vec2(0.0, 0.0)));
	}
}
