use std::fmt;
use bevy::prelude::{
	Resource, Res, ResMut,
	Component, Bundle,
	Commands,
	Color,
	Vec2, Vec3, Vec4,
};
use bevy::transform;
use bevy::transform::components::Transform;
use bevy_vector_shapes::prelude::*;

use crate::component_utils::*;
use crate::color_utils::*;

// =============================================================================
// Components/bundle/utilities for the messages logged above the typing area.
// =============================================================================
// TODO: Add vector shape for the bubble.

#[derive(Component, Debug)]
pub struct MsgText(pub String);

#[derive(Component, Debug)]
pub struct FontColor(pub Color);

#[derive(Component, Debug)]
pub struct BkgColor(pub Color);

#[derive(Component, Debug)]
pub struct IsMine(pub bool);

#[derive(Debug, PartialEq, Eq)]
pub enum HDir { LEFT, RIGHT, }
#[derive(Component, Debug)]
pub struct Side(pub HDir);

impl fmt::Display for HDir {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			HDir::LEFT => { "LEFT" },
			HDir::RIGHT => { "RIGHT" },
		})
	}
}

#[derive(Component, Debug, PartialEq, Eq)]
pub struct Index(usize);					// A custom index that we can set to an incrementing Resource<usize> value.
										// Helpful (say) to order text messages when displaying.

#[derive(Resource)]
pub struct NextIndex(usize);		// Afore-alluded next-index Resource for SentMessage bundles.
impl Default for NextIndex {
	fn default() -> Self {
		Self(0)
	}
}
// Would be ideal to have index increment automatically on spawning a message (or creating its bundle), but
// this likely requires use of a ctor which we haven't had to do for components or bundles yet, and
// the current system (remembering to always spawn via the message spawning helper method) works ok for now.

#[derive(Bundle, Debug)]
struct SentMessageBundle {
	text: MsgText,
	font_color: FontColor,
	bkg_color: BkgColor,
	is_mine: IsMine,
	side: Side,
	index: Index,
	transform: Transform,
}

// Utility function to spawn a message based on text content, sender/owner, and whether to preserve on conversation reset.
pub fn spawn_sent_message(
	commands: &mut Commands,
	next_index: &mut ResMut<NextIndex>,
	color_scheme: & Res<ColorScheme>,
	text: &'static str,
	is_mine: bool,
	preserve_on_clear: bool,
	transform: Option<Transform>,
) {
	let transform = if let Some(transform) = transform { transform } else { Transform::from_xyz(0., 0., 0.) };

	let msg_bundle = SentMessageBundle {
		text: MsgText(String::from(text)),
		font_color: FontColor(if is_mine { color_scheme.my_text_color } else { color_scheme.their_text_color }),
		bkg_color: BkgColor(if is_mine { color_scheme.my_bubble_color } else { color_scheme.their_bubble_color },),
		is_mine: IsMine(is_mine),
		side: Side(if is_mine { HDir::RIGHT } else { HDir::LEFT }),
		index: Index(next_index.0),
		transform: transform,
	};

	let shape_bundle = ShapeBundle::rect(
		&ShapeConfig {
			color: BLUE_BUBBLE_COLOR,
			corner_radii: Vec4::splat(40.),
			// transform: Transform::from_xyz(0., 0., 0.),
			..ShapeConfig::default_2d()
		},
		Vec2::new(600., 80.),
	);

	println!("\nspawn_sent_message():{}", msg_bundle);
	// let mut entity_commands = commands.spawn(msg_bundle);
	let mut entity_commands = commands.spawn(msg_bundle);
	
	entity_commands.with_child(shape_bundle);

	if preserve_on_clear {
		entity_commands.insert(PreserveOnClear);
	}
	next_index.0 += 1;
}

// This Display implementation is only useful for the bundle itself (i.e. when we are spawning a message).
// Additional utility function below prints message details passed into it piecemeal (can be a subset).
impl fmt::Display for SentMessageBundle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let font_srgba = self.font_color.0.to_srgba();
		let bkg_srgba = self.bkg_color.0.to_srgba();
		write!(
			f,
			"\n\"{}\"\nFont Color: ({}, {}, {}, {})\nBkg Color: ({}, {}, {}, {})\nMine? {}\nSide: {}\nIndex: {}",
			self.text.0,
			font_srgba.red, font_srgba.blue, font_srgba.green, font_srgba.alpha,
			bkg_srgba.red, bkg_srgba.blue, bkg_srgba.green, bkg_srgba.alpha,
			self.is_mine.0,
			self.side.0,
			self.index.0,
		)
	}
}

// This utility function prints message details passed into it piecemeal (can be a subset).
pub fn print_sent_message(
	text: Option<&MsgText>,
	font_color: Option<&FontColor>,
	bkg_color: Option<&BkgColor>,
	is_mine: Option<&IsMine>,
	side: Option<&Side>,
	index: Option<&Index>,
) {
	println!();
	
	if let Some(text) = text {
		println!("Text: \"{}\"", text.0);
	}

	if let Some(font_color) = font_color {
		let font_srgba = font_color.0.to_srgba();
		println!(
			"FontColor: ({}, {}, {}, {})",
			font_srgba.red,
			font_srgba.blue,
			font_srgba.green,
			font_srgba.alpha,
		);
	}

	if let Some(bkg_color) = bkg_color {
		let bkg_srgba = bkg_color.0.to_srgba();
		println!(
			"BkgColor: ({}, {}, {}, {})",
			bkg_srgba.red,
			bkg_srgba.blue,
			bkg_srgba.green,
			bkg_srgba.alpha,
		);
	}

	if let Some(is_mine) = is_mine {
		println!("Mine? {}", is_mine.0);
	}

	if let Some(side) = side {
		println!("Side: {}", side.0);
	}

	if let Some(index) = index {
		println!("Index: {}", index.0);
	}
}