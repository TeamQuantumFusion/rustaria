use rsa_core::ty::Tag;

/// An entity renderer is responsible for rendering a single entity type.
pub struct EntityTypeRenderer {
	pub image: Tag,
	pub x_offset: f32,
	pub y_offset: f32,
	pub width: f32,
	pub height: f32,
}