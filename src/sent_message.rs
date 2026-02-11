use std::fmt;
use bevy::prelude::{
	Resource, Res, ResMut,
	Component, Bundle,
	Color,
	// Entity,
	// Query,
	Commands,
};
use crate::component_utils::{
	PreserveOnClear,
};
use crate::color_utils::{
	ColorScheme,
};

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
}

// Utility function to spawn a message based on text content, sender/owner, and whether to preserve on conversation reset.
pub fn spawn_sent_message(
	commands: &mut Commands,
	next_index: &mut ResMut<NextIndex>,
	color_scheme: & Res<ColorScheme>,
	text: &'static str,
	is_mine: bool,
	preserve_on_clear: bool,
) {
	let bundle = SentMessageBundle {
		text: MsgText(String::from(text)),
		font_color: FontColor(if is_mine { color_scheme.my_text_color } else { color_scheme.their_text_color }),
		bkg_color: BkgColor(if is_mine { color_scheme.my_bubble_color } else { color_scheme.their_bubble_color },),
		is_mine: IsMine(is_mine),
		side: Side(if is_mine { HDir::RIGHT } else { HDir::LEFT }),
		index: Index(next_index.0),
	};
	println!("\nspawn_sent_message():{}", bundle);
	let mut id = commands.spawn(bundle);
	if preserve_on_clear {
		id.insert(PreserveOnClear);
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