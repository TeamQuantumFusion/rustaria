//! Axis-Aligned Bounding Box.
//!
//! Fancy word for "check if shit touches other shit in a 2d minecraft-clone world"
//!
//! https://www.youtube.com/watch?v=8JJ-4JgR7Dg

use std::{mem::swap, ops::Mul};

use euclid::{rect, vec2};

use crate::ty::{direction::Direction, WS};

type Vec2 = euclid::Vector2D<f32, WS>;
type Rect = euclid::Rect<f32, WS>;

#[inline(always)]
pub fn point_vs_rect(p: Vec2, r: Rect) -> bool {
	p.x >= r.origin.x
		&& p.y >= r.origin.y
		&& p.x < r.origin.x + r.size.width
		&& p.y < r.origin.y + r.size.height
}

#[inline(always)]
pub fn rect_vs_rect(r1: Rect, r2: Rect) -> bool {
	r1.origin.x < r2.origin.x + r2.size.width
		&& r1.origin.x + r1.size.width > r2.origin.x
		&& r1.origin.y < r2.origin.y + r2.size.height
		&& r1.origin.y + r1.size.height > r2.origin.y
}

pub struct RayRectCollision {
	pub contact_time: f32,
	pub contact_point: Vec2,
	pub contact_normal: Option<Direction>,
}

pub fn ray_vs_rect(ray_origin: Vec2, ray_dir: Vec2, target: Rect) -> Option<RayRectCollision> {
	// Cache division
	let invdir = Vec2::new(1.0 / ray_dir.x, 1.0 / ray_dir.y);

	// Calculate intersections with rectangle bounding axes
	let mut t_near = (target.origin - ray_origin)
		.to_vector()
		.component_mul(invdir);
	let mut t_far = (target.origin + target.size - ray_origin)
		.to_vector()
		.component_mul(invdir);

	if t_far.y.is_nan() || t_far.x.is_nan() || t_near.y.is_nan() || t_near.x.is_nan() {
		return None;
	}

	// Sort distances
	if t_near.x > t_far.x {
		swap(&mut t_near.x, &mut t_far.x);
	}
	if t_near.y > t_far.y {
		swap(&mut t_near.y, &mut t_far.y);
	}

	// Early rejection
	if t_near.x > t_far.y || t_near.y > t_far.x {
		return None;
	};

	// Furthest 'time' is contact on opposite side of target
	let t_hit_far = t_far.x.min(t_far.y);

	// Reject if ray direction is pointing away from object
	if t_hit_far < 0.0 {
		return None;
	}

	// Closest 'time' will be the first contact
	let contact_time = t_near.x.max(t_near.y);

	// Note if t_near == t_far, collision is principly in a diagonal
	// so pointless to resolve. By returning a CN={0,0} even though its
	// considered a hit, the resolver wont change anything.
	Some(RayRectCollision {
		contact_time,
		// Contact point of collision from parametric line equation
		contact_point: ray_origin.mul(contact_time).component_mul(ray_dir),
		contact_normal: if t_near.x > t_near.y {
			Some(if invdir.x < 0.0 {
				Direction::Right
			} else {
				Direction::Left
			})
		} else if t_near.x < t_near.y {
			Some(if invdir.y < 0.0 {
				Direction::Up
			} else {
				Direction::Down
			})
		} else {
			None
		},
	})
}

pub fn dynamic_rect_vs_rect(
	r_dynamic_vel: Vec2,
	r_dynamic: Rect,
	time_step: f32,
	r_static: Rect,
) -> Option<RayRectCollision> {
	// Check if dynlib rectangle is actually moving - we assume rectangles are NOT in collision to start
	if r_dynamic_vel.x == 0.0 && r_dynamic_vel.y == 0.0 {
		return None;
	}

	// Expand target rectangle by source dimensions
	let expanded_target = rect(
		r_static.origin.x - (r_dynamic.size.width / 2.0),
		r_static.origin.y - (r_dynamic.size.height / 2.0),
		r_static.size.width + r_dynamic.size.width,
		r_static.size.height + r_dynamic.size.height,
	);

	if let Some(collision) = ray_vs_rect(
		(r_dynamic.origin + (r_dynamic.size / 2.0)).to_vector(),
		r_dynamic_vel * time_step,
		expanded_target,
	) {
		if collision.contact_time >= 0.0 && collision.contact_time < 1.0 {
			return Some(collision);
		}
	}

	None
}

pub fn resolve_dynamic_rect_vs_rect(
	r_dynamic_vel: Vec2,
	r_dynamic: Rect,
	time_step: f32,
	r_static: Rect,
) -> Option<Option<(Vec2, Direction)>> {
	dynamic_rect_vs_rect(r_dynamic_vel, r_dynamic, time_step, r_static).map(|result| {
		result.contact_normal.map(|direction| {
			(
				direction
					.to_vec2()
					.component_mul(vec2(r_dynamic_vel.x.abs(), r_dynamic_vel.y.abs()))
					* (1.0 - result.contact_time),
				direction,
			)
		})
	})
}
