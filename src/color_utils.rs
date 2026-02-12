use bevy::prelude::{
	Resource, Res, ResMut,
	Query, With,
	Color,
};

use crate::sent_message::{
	MsgText,
	FontColor,
	BkgColor,
	IsMine,
	Side,
	Index,
	print_sent_message,
};

// =============================================================================
// Color constants
// =============================================================================

pub const COLOR_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
pub const COLOR_BLACK: Color = Color::srgb(0.0, 0.0, 0.0);

// Bubble colors
pub const BLUE_BUBBLE_COLOR: Color = Color::srgb(2./255., 129./255., 253./255.);			// #0281FD
pub const GREEN_BUBBLE_COLOR: Color = Color::srgb(51./255., 206./255., 90./255.);			// #33CE5A
// Darkmode colors
pub const DKMODE_TOP_BKG_COLOR: Color = Color::srgb(18./255., 18./255., 18./255.);			// #121212 (bkg color topmost area)
pub const DKMODE_TOP_RULE_COLOR: Color = Color::srgb(14./255., 14./255., 14./255.);			// #0E0E0E
pub const DKMODE_MID_BKG_COLOR: Color = COLOR_BLACK;										// #000000 (bkg color main area)
pub const DKMODE_SYS_TEXT_COLOR: Color = Color::srgb(118./255., 118./255., 118./255.);		// #767676 (Read, Sent, Delivered, date/time, etc.)
pub const DKMODE_KEY_TEXT_COLOR: Color = COLOR_WHITE;										// #FFFFFF (symbols on keys)
pub const DKMODE_KEY_COLOR: Color = Color::srgb(96./255., 96./255., 96./255.);				// #606060 (most keys)
pub const DKMODE_CAPS_COLOR: Color = Color::srgb(209./255., 209./255., 209./255.);			// #D1D1D1 (shift/caps)
pub const DKMODE_BKSP_COLOR: Color = Color::srgb(59./255., 59./255., 59./255.);				// #3B3B3B (123, return, bksp)
pub const DKMODE_KEYBOARD_COLOR: Color = Color::srgb(27./255., 27./255., 27./255.);			// #1B1B1B (keyboard bkg)
pub const DKMODE_THEIR_BUBBLE_COLOR: Color = Color::srgb(38./255., 38./255., 42./255.);		// #26262A
pub const DKMODE_THEIR_TEXT_COLOR: Color = COLOR_WHITE;										// #FFFFFF
// Lightmode colors
pub const LTMODE_TOP_BKG_COLOR: Color = Color::srgb(249./255., 249./255., 249./255.);		// #F9F9F9
pub const LTMODE_TOP_RULE_COLOR: Color = Color::srgb(118./255., 118./255., 118./255.);		// #767676
pub const LTMODE_MID_BKG_COLOR: Color = COLOR_WHITE;										// #FFFFFF (bkg color main area)
pub const LTMODE_SYS_TEXT_COLOR: Color = Color::srgb(118./255., 118./255., 118./255.);		// #767676 (Read, Sent, Delivered, date/time, etc.)
pub const LTMODE_KEY_TEXT_COLOR: Color = COLOR_BLACK;										// #000000 (symbols on keys)
pub const LTMODE_KEY_COLOR: Color = COLOR_WHITE;											// #FFFFFF (most keys)
pub const LTMODE_CAPS_COLOR: Color = LTMODE_KEY_COLOR;										// #FFFFFF (shift/caps)
pub const LTMODE_BKSP_COLOR: Color = LTMODE_KEY_COLOR;										// #FFFFFF (123, return, bksp)
pub const LTMODE_KEYBOARD_COLOR: Color = Color::srgb(227./255., 229./255., 230./255.);		// #E3E5E6 (keyboard bkg)
pub const LTMODE_THEIR_BUBBLE_COLOR: Color = Color::srgb(233./255., 233./255., 234./255.);	// #E9E9EA
pub const LTMODE_THEIR_TEXT_COLOR: Color = COLOR_BLACK;										// #000000
// Default colors
pub const DEFAULT_TOP_BKG_COLOR: Color = DKMODE_TOP_BKG_COLOR;
pub const DEFAULT_TOP_RULE_COLOR: Color = DKMODE_TOP_RULE_COLOR;
pub const DEFAULT_MID_BKG_COLOR: Color = DKMODE_MID_BKG_COLOR;
pub const DEFAULT_SYS_TEXT_COLOR: Color = DKMODE_SYS_TEXT_COLOR;
pub const DEFAULT_KEY_TEXT_COLOR: Color = DKMODE_KEY_TEXT_COLOR;
pub const DEFAULT_KEY_COLOR: Color = DKMODE_KEY_COLOR;
pub const DEFAULT_CAPS_COLOR: Color = DKMODE_CAPS_COLOR;
pub const DEFAULT_BKSP_COLOR: Color = DKMODE_BKSP_COLOR;
pub const DEFAULT_KEYBOARD_COLOR: Color = DKMODE_KEYBOARD_COLOR;
pub const DEFAULT_MY_BUBBLE_COLOR: Color = BLUE_BUBBLE_COLOR;
pub const DEFAULT_MY_TEXT_COLOR: Color = COLOR_WHITE;
pub const DEFAULT_THEIR_BUBBLE_COLOR: Color = DKMODE_THEIR_BUBBLE_COLOR;
pub const DEFAULT_THEIR_TEXT_COLOR: Color = DKMODE_THEIR_TEXT_COLOR;

#[derive(Resource, PartialEq)]
pub struct ColorScheme {
	pub top_bkg_color: Color,
	pub top_rule_color: Color,
	pub mid_bkg_color: Color,
	pub sys_text_color: Color,
	pub key_text_color: Color,
	pub key_color: Color,
	pub key_color_caps: Color,
	pub key_color_bksp: Color,
	pub keyboard_color: Color,
	pub my_bubble_color: Color,
	pub my_text_color: Color,
	pub their_bubble_color: Color,
	pub their_text_color: Color,
}
impl Default for ColorScheme {
	fn default() -> Self {
		Self {
			top_bkg_color: DEFAULT_TOP_BKG_COLOR,
			top_rule_color: DEFAULT_TOP_RULE_COLOR,
			mid_bkg_color: DEFAULT_MID_BKG_COLOR,
			sys_text_color: DEFAULT_SYS_TEXT_COLOR,
			key_text_color: DEFAULT_KEY_TEXT_COLOR,
			key_color: DEFAULT_KEY_COLOR,
			key_color_caps: DEFAULT_CAPS_COLOR,
			key_color_bksp: DEFAULT_BKSP_COLOR,
			keyboard_color: DEFAULT_KEYBOARD_COLOR,
			my_bubble_color: DEFAULT_MY_BUBBLE_COLOR,
			my_text_color: DEFAULT_MY_TEXT_COLOR,
			their_bubble_color: DEFAULT_THEIR_BUBBLE_COLOR,
			their_text_color: DEFAULT_THEIR_TEXT_COLOR,
		}
	}
}

// This resource tracks the currently selected color mode (i.e. light, dark).
#[derive(Resource)]
pub struct DarkModeEnabled(pub bool);	// Can we set up an observer for when this changes?
impl Default for DarkModeEnabled {
	fn default() -> Self {
		Self(true)
	}
}

// This resource tracks whether we're in SMS/Android land (green bubbles).
#[derive(Resource)]
pub struct AndroidModeEnabled(pub bool);	// Can we set up an observer for when this changes?
impl Default for AndroidModeEnabled {
	fn default() -> Self {
		Self(false)
	}
}

// This runs when DarkModeEnabled changes (see App setup).
pub fn on_dark_mode_enabled_changed(
	dark_mode_enabled: Res<DarkModeEnabled>,
	mut color_scheme: ResMut<ColorScheme>
) {
	let dark = dark_mode_enabled.0;
	// println!("\nchanging scheme colors! dark mode enabled? {dark}");
	color_scheme.top_bkg_color = if dark { DKMODE_TOP_BKG_COLOR } else { LTMODE_TOP_BKG_COLOR };
	color_scheme.top_rule_color = if dark { DKMODE_TOP_RULE_COLOR } else { LTMODE_TOP_RULE_COLOR };
	color_scheme.mid_bkg_color = if dark { DKMODE_MID_BKG_COLOR } else { LTMODE_MID_BKG_COLOR };
	color_scheme.sys_text_color = if dark { DKMODE_SYS_TEXT_COLOR } else { LTMODE_SYS_TEXT_COLOR };
	color_scheme.key_text_color = if dark { DKMODE_KEY_TEXT_COLOR } else { LTMODE_KEY_TEXT_COLOR };
	color_scheme.key_color = if dark { DKMODE_KEY_COLOR } else { LTMODE_KEY_COLOR };
	color_scheme.key_color_caps = if dark { DKMODE_CAPS_COLOR } else { LTMODE_CAPS_COLOR };
	color_scheme.key_color_bksp = if dark { DKMODE_BKSP_COLOR } else { LTMODE_BKSP_COLOR };
	color_scheme.keyboard_color = if dark { DKMODE_KEYBOARD_COLOR } else { LTMODE_KEYBOARD_COLOR };
	// Don't need the following two lines unless we decide to change player's text/bubble colors with dark/light mode change.
	// color_scheme.my_bubble_color = if dark { DKMODE_MY_BUBBLE_COLOR } else { LTMODE_MY_BUBBLE_COLOR };
	// color_scheme.my_text_color = if dark { DKMODE_MY_TEXT_COLOR } else { LTMODE_MY_TEXT_COLOR };
	color_scheme.their_bubble_color = if dark { DKMODE_THEIR_BUBBLE_COLOR } else { LTMODE_THEIR_BUBBLE_COLOR };
	color_scheme.their_text_color = if dark { DKMODE_THEIR_TEXT_COLOR } else { LTMODE_THEIR_TEXT_COLOR };
}

// This runs when AndroidModeEnabled changes (see App setup).
pub fn on_android_mode_enabled_changed(
	android_mode_enabled: Res<AndroidModeEnabled>,
	mut color_scheme: ResMut<ColorScheme>
) {
	color_scheme.my_bubble_color = if android_mode_enabled.0 { GREEN_BUBBLE_COLOR } else { BLUE_BUBBLE_COLOR };
}

// This runs when ColorScheme changes (see App setup).
pub fn update_colors_on_color_scheme_change(
	// For now, the only entities with Text are those created via SentMessageBundle.
	// If that changes, we will need to make this filter more specific.
	mut msgs: Query<(&mut FontColor, &mut BkgColor, &IsMine), With<MsgText>>,
	color_scheme: Res<ColorScheme>,
) {
	for (mut font_color, mut bkg_color, is_mine) in &mut msgs {
		font_color.0 = if is_mine.0 { color_scheme.my_text_color } else { color_scheme.their_text_color };
		bkg_color.0 = if is_mine.0 { color_scheme.my_bubble_color } else { color_scheme.their_bubble_color };
	}
}

pub fn print_messages_on_color_scheme_change(
	msgs: Query<(&MsgText, &FontColor, &BkgColor, &IsMine, &Side, &Index)>
) {
	println!("\nprint_messages_on_color_scheme_change()");
	for (text, font_color, bkg_color, is_mine, side, index) in &msgs {
	// for msg in &msgs {
		// println!("\nText: {}\nFontColor: {:?}\nBkgColor: {:?}\nSide: {:?}\nIndex: {}", text.0, font_color.0, bkg_color.0, side.0, index.0);
		print_sent_message(Some(text), Some(font_color), Some(bkg_color), Some(is_mine), Some(side), Some(index));
	}
}