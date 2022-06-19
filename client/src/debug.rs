use std::time::{Duration, Instant};

use euclid::{rect, vec2, Rect, Vector2D};
use eyre::Result;
use glium::{program::SourceCode, uniform, Blend, DrawParameters, Frame, Program};
use rustaria::{
	debug::{DebugCategory, DebugDraw, DebugEvent, DebugRendererImpl},
	ty::WS,
	TPS,
};
use tracing::info;

use crate::{
	render::ty::{
		mesh_buffer::MeshDrawer, mesh_builder::MeshBuilder, vertex::PosColorVertex,
		viewport::Viewport,
	},
	Frontend,
};

pub struct Debug {
	program: Program,
	builder: MeshBuilder<PosColorVertex>,
	drawer: MeshDrawer<PosColorVertex>,
	events: Vec<DebugEvent>,

	tick_event_times: Vec<Duration>,
	tick_times: Vec<Duration>,
	draw_times: Vec<Duration>,
	last_print: Instant,

	line_size: f32,
	categories: DebugCategory,
}

impl Debug {
	pub fn new(frontend: &Frontend) -> Result<Debug> {
		Ok(Debug {
			program: frontend.create_program(SourceCode {
				vertex_shader: include_str!("./builtin/pos_color.vert.glsl"),
				tessellation_control_shader: None,
				tessellation_evaluation_shader: None,
				geometry_shader: None,
				fragment_shader: include_str!("./builtin/pos_color.frag.glsl"),
			})?,
			drawer: frontend.create_drawer()?,
			builder: MeshBuilder::new(),
			line_size: 0.1,
			events: vec![],
			tick_event_times: Default::default(),
			tick_times: Default::default(),
			draw_times: Default::default(),
			last_print: Instant::now(),
			categories: DebugCategory::Temporary,
		})
	}

	pub fn enable(&mut self, kind: DebugCategory) { self.categories |= kind; }

	pub fn disable(&mut self, kind: DebugCategory) {
		self.categories &= !kind;
		self.categories |= DebugCategory::Temporary;
	}

	pub fn log_event(&mut self, start: Instant) { self.tick_event_times.push(start.elapsed()); }

	pub fn log_tick(&mut self, start: Instant) { self.tick_times.push(start.elapsed()); }

	pub fn log_draw(&mut self, start: Instant) { self.draw_times.push(start.elapsed()); }

	pub fn tick(&mut self) {
		for event in &mut self.events {
			if event.ticks_remaining > 0 {
				event.ticks_remaining -= 1;
			}
		}
		self.events.drain_filter(|event| event.ticks_remaining == 0);

		if self.last_print.elapsed() > Duration::from_secs_f64(1.0) {
			self.last_print = Instant::now();
			let mut event_time = 0.0;
			let events = self.tick_event_times.len();
			{
				let len = events as f64;
				for x in self.tick_event_times.drain(..) {
					event_time += x.as_secs_f64() / len;
				}
			};

			let mut tick_time = 0.0;
			let ticks = self.tick_times.len();
			{
				let len = ticks as f64;
				for x in self.tick_times.drain(..) {
					tick_time += x.as_secs_f64() / len;
				}
			};

			let mut draw_time = 0.0;
			let draws = self.draw_times.len();
			{
				let len = draws as f64;
				for x in self.draw_times.drain(..) {
					draw_time += x.as_secs_f64() / len;
				}
			};

			info!(
				"Events {}@{:.2}ms | Tick {:.1}% {}@{:.2}ms | Draw {}@{:.2}ms",
				events,
				event_time * 1000.0,
				(tick_time / (1.0 / TPS as f64)) * 100.0,
				ticks,
				tick_time * 1000.0,
				draws,
				draw_time * 1000.0
			)
		}
	}

	pub fn draw(
		&mut self,
		frontend: &Frontend,
		camera: &Viewport,
		frame: &mut Frame,
	) -> Result<()> {
		for event in &self.events {
			if self.categories.contains(event.category) {
				let color = Self::get_color(event.color);
				let line_size = self.line_size;
				Self::mesh_event(&mut self.builder, color, line_size, event);
			}
		}
		self.drawer.upload(&self.builder)?;
		self.builder.clear();

		let uniforms = uniform! {
			screen_ratio: frontend.aspect_ratio,
			player_pos: camera.pos.to_array(),
			zoom: camera.zoom,
		};

		let draw_parameters = DrawParameters {
			blend: Blend::alpha_blending(),
			..DrawParameters::default()
		};
		self.drawer
			.draw(frame, &self.program, &uniforms, &draw_parameters)?;
		Ok(())
	}

	fn mesh_event(
		builder: &mut MeshBuilder<PosColorVertex>,
		color: [f32; 3],
		line_size: f32,
		event: &DebugEvent,
	) {
		let opacity = if event.duration > 0 {
			event.ticks_remaining as f32 / event.duration as f32
		} else {
			1.0
		};
		let color = [color[0], color[1], color[2], opacity];
		let line_size = line_size * event.line_size;
		match event.draw {
			DebugDraw::Quad(quad) => {
				Self::mesh_rect(builder, color, quad, line_size);
			}
			DebugDraw::Line { start, stop } => {
				Self::mesh_line(builder, color, line_size, start, stop);
			}
			DebugDraw::Point(pos) => {
				Self::mesh_point(builder, color, line_size, pos);
			}
		}
	}

	fn mesh_rect(
		builder: &mut MeshBuilder<PosColorVertex>,
		color: [f32; 4],
		rect: Rect<f32, WS>,
		border_size: f32,
	) {
		if border_size == 0.0
			|| (border_size * 2.0 > rect.width() && border_size * 2.0 > rect.height())
		{
			// common case where the border is full
			builder.push_quad((rect, color));
		} else {
			// top
			{
				let mut rect = rect;
				let height = rect.height();
				rect.origin.y += height - border_size;
				rect.size.height = border_size;
				builder.push_quad((rect, color));
			}

			// bottom
			{
				let mut rect = rect;
				rect.size.height = border_size;
				builder.push_quad((rect, color));
			}

			// right
			{
				let mut rect = rect;
				let width = rect.width();
				rect.origin.x += width - border_size;
				rect.size.width = border_size;
				// overlap correction
				rect.size.height -= border_size * 2.0;
				rect.origin.y += border_size;
				builder.push_quad((rect, color));
			}

			// left
			{
				let mut rect = rect;
				rect.size.width = border_size;
				// overlap correction
				rect.size.height -= border_size * 2.0;
				rect.origin.y += border_size;
				builder.push_quad((rect, color));
			}
		}
	}

	fn mesh_point(
		builder: &mut MeshBuilder<PosColorVertex>,
		color: [f32; 4],
		size: f32,
		pos: Vector2D<f32, WS>,
	) {
		let half = (size / 2.0);
		builder.push_quad((rect::<_, WS>(pos.x - half, pos.y - half, size, size), color))
	}

	fn mesh_line(
		builder: &mut MeshBuilder<PosColorVertex>,
		color: [f32; 4],
		line_size: f32,
		start: Vector2D<f32, WS>,
		stop: Vector2D<f32, WS>,
	) {
		let furbertensvector = stop - start;
		let not_furbertensvector =
			vec2::<_, WS>(-furbertensvector.y, furbertensvector.x).normalize();

		let t_start = start + (not_furbertensvector * (line_size / 2.0));
		let b_start = start + (-not_furbertensvector * (line_size / 2.0));
		let t_stop = stop + (not_furbertensvector * (line_size / 2.0));
		let b_stop = stop + (-not_furbertensvector * (line_size / 2.0));
		builder.push_quad((
			[
				t_start.to_array(),
				b_start.to_array(),
				b_stop.to_array(),
				t_stop.to_array(),
			],
			color,
		));
	}

	fn get_color(color: u32) -> [f32; 3] {
		let b = (color & 0xff) as f32 / 255.0;
		let g = ((color >> 8) & 0xff) as f32 / 255.0;
		let r = ((color >> 16) & 0xff) as f32 / 255.0;

		fn convert(f: f32) -> f32 {
			if f <= 0.04045 {
				f / 12.92
			} else {
				let a = 0.055_f32;

				((f + a) * (1. + a).powf(-1.)).powf(2.4)
			}
		}

		[convert(r), convert(g), convert(b)]
	}
}

impl DebugRendererImpl for Debug {
	fn event(&mut self, event: DebugEvent) { self.events.push(event); }
}
