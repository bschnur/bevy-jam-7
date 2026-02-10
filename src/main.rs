use std::fmt;

use bevy::prelude::*;

// Colors

const COLOR_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
const COLOR_BLACK: Color = Color::srgb(0.0, 0.0, 0.0);

// Bubble colors
const BLUE_BUBBLE_COLOR: Color = Color::srgb(2./255., 129./255., 253./255.);			// #0281FD
const GREEN_BUBBLE_COLOR: Color = Color::srgb(51./255., 206./255., 90./255.);			// #33CE5A
// Darkmode colors
const DKMODE_TOP_BKG_COLOR: Color = Color::srgb(18./255., 18./255., 18./255.);			// #121212 (bkg color topmost area)
const DKMODE_TOP_RULE_COLOR: Color = Color::srgb(14./255., 14./255., 14./255.);			// #0E0E0E
const DKMODE_MID_BKG_COLOR: Color = COLOR_BLACK;										// #000000 (bkg color main area)
const DKMODE_SYS_TEXT_COLOR: Color = Color::srgb(118./255., 118./255., 118./255.);		// #767676 (Read, Sent, Delivered, date/time, etc.)
const DKMODE_KEY_TEXT_COLOR: Color = COLOR_WHITE;										// #FFFFFF (glyphs on keys)
const DKMODE_KEY_COLOR: Color = Color::srgb(96./255., 96./255., 96./255.);				// #606060 (most keys)
const DKMODE_CAPS_COLOR: Color = Color::srgb(209./255., 209./255., 209./255.);			// #D1D1D1 (shift/caps)
const DKMODE_BKSP_COLOR: Color = Color::srgb(59./255., 59./255., 59./255.);				// #3B3B3B (123, return, bksp)
const DKMODE_KEYBOARD_COLOR: Color = Color::srgb(27./255., 27./255., 27./255.);			// #1B1B1B (keyboard bkg)
const DKMODE_THEIR_BUBBLE_COLOR: Color = Color::srgb(38./255., 38./255., 42./255.);		// #26262A
const DKMODE_THEIR_TEXT_COLOR: Color = COLOR_WHITE;										// #FFFFFF
// Lightmode colors
const LTMODE_TOP_BKG_COLOR: Color = Color::srgb(249./255., 249./255., 249./255.);		// #F9F9F9
const LTMODE_TOP_RULE_COLOR: Color = Color::srgb(118./255., 118./255., 118./255.);		// #767676
const LTMODE_MID_BKG_COLOR: Color = COLOR_WHITE;										// #FFFFFF (bkg color main area)
const LTMODE_SYS_TEXT_COLOR: Color = Color::srgb(118./255., 118./255., 118./255.);		// #767676 (Read, Sent, Delivered, date/time, etc.)
const LTMODE_KEY_TEXT_COLOR: Color = COLOR_BLACK;										// #000000 (glyphs on keys)
const LTMODE_KEY_COLOR: Color = COLOR_WHITE;											// #FFFFFF (most keys)
const LTMODE_CAPS_COLOR: Color = LTMODE_KEY_COLOR;										// #FFFFFF (shift/caps)
const LTMODE_BKSP_COLOR: Color = LTMODE_KEY_COLOR;										// #FFFFFF (123, return, bksp)
const LTMODE_KEYBOARD_COLOR: Color = Color::srgb(227./255., 229./255., 230./255.);		// #E3E5E6 (keyboard bkg)
const LTMODE_THEIR_BUBBLE_COLOR: Color = Color::srgb(233./255., 233./255., 234./255.);	// #E9E9EA
const LTMODE_THEIR_TEXT_COLOR: Color = COLOR_BLACK;										// #000000
// Default colors
const DEFAULT_TOP_BKG_COLOR: Color = DKMODE_TOP_BKG_COLOR;
const DEFAULT_TOP_RULE_COLOR: Color = DKMODE_TOP_RULE_COLOR;
const DEFAULT_MID_BKG_COLOR: Color = DKMODE_MID_BKG_COLOR;
const DEFAULT_SYS_TEXT_COLOR: Color = DKMODE_SYS_TEXT_COLOR;
const DEFAULT_KEY_TEXT_COLOR: Color = DKMODE_KEY_TEXT_COLOR;
const DEFAULT_KEY_COLOR: Color = DKMODE_KEY_COLOR;
const DEFAULT_CAPS_COLOR: Color = DKMODE_CAPS_COLOR;
const DEFAULT_BKSP_COLOR: Color = DKMODE_BKSP_COLOR;
const DEFAULT_KEYBOARD_COLOR: Color = DKMODE_KEYBOARD_COLOR;
const DEFAULT_MY_BUBBLE_COLOR: Color = BLUE_BUBBLE_COLOR;
const DEFAULT_MY_TEXT_COLOR: Color = COLOR_WHITE;
const DEFAULT_THEIR_BUBBLE_COLOR: Color = DKMODE_THEIR_BUBBLE_COLOR;
const DEFAULT_THEIR_TEXT_COLOR: Color = DKMODE_THEIR_TEXT_COLOR;

#[derive(Resource)]
struct ColorScheme {
	top_bkg_color: Color,
	top_rule_color: Color,
	mid_bkg_color: Color,
	sys_text_color: Color,
	key_text_color: Color,
	key_color: Color,
	key_color_caps: Color,
	key_color_bksp: Color,
	keyboard_color: Color,
	my_bubble_color: Color,
	my_text_color: Color,
	their_bubble_color: Color,
	their_text_color: Color,
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

// #[derive(Resource)]
// struct DarkModeEnabled(bool);	// Can we set up an observer for when this changes?
// impl Default for DarkModeEnabled {
// 	fn default() -> Self {
// 		Self(true)
// 	}
// }

// Sizes

// The resolution we are pretending to render at / rendering at before scaling.
const VIRTUAL_RESOLUTION: Vec2 = Vec2::new(1080., 1920.);
// The below sizes are calculated based on the virtual resolution.
// Lots of things marked DEFAULT with the intention being they may be substituted for.
const DEFAULT_BUBBLE_CORNER_RADIUS: f32 = 10.;

// Keyboard Layout

// 	suggestions
// Q W E R T Y U I O P			10
// A S D F G H J K L			9
// ⇧ X C V B N M ⇐				9 (7 + 2 special)
// 123 😊 space ↩				4 (2 mini-special + spacebar + 1 extra-special)
// ⨁			🎙

const DEFAULT_KEY_HEIGHT: f32 = 116.;

const DEFAULT_KEY_SIZE: Vec2 = Vec2::new(90., DEFAULT_KEY_HEIGHT);
const DEFAULT_SPECIAL_KEY_SIZE: Vec2 = Vec2::new(122., DEFAULT_KEY_HEIGHT);
const DEFAULT_MINI_SPECIAL_KEY_SIZE: Vec2 = Vec2::new(116.5, DEFAULT_KEY_HEIGHT);
const DEFAULT_SPACEBAR_SIZE: Vec2 = Vec2::new(519.5, DEFAULT_KEY_HEIGHT);
const DEFAULT_KEY_SPACING: f32 = 16.;

const DEFAULT_ROW_1_MARGIN: f32 = 18.;
const DEFAULT_ROW_2_MARGIN: f32 = 71.;
const DEFAULT_ROW_3_INNER_MARGIN: f32 = 37.;
const DEFAULT_ROW_3_OUTER_MARGIN: f32 = DEFAULT_ROW_1_MARGIN;
const DEFAULT_ROW_4_MARGIN: f32 = DEFAULT_ROW_1_MARGIN;

// Notes on entities needed:
// messages, keyboard, drafting area, ..., Read/Delivered/Sent, timestamp horizontal rule,
// key, finger, tooth, ghost text?, individual letters?

// Refresh on usefulness/need for bundles vs tupled components.

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)

		.insert_resource(FeverLevel(0))
		// .insert_resource(DarkModeEnabled(true))	// Unnecessary because DarkModeEnabled implements Default.
		.init_resource::<ColorScheme>()
		.init_resource::<DarkModeEnabled>()
		.init_resource::<SentMessageNextIndex>()

		.insert_resource(ClearColor(DEFAULT_MID_BKG_COLOR)) // bevy built-in Resource, used for window clearing - might not use

        // .add_systems(PreStartup, pre_startup)
        .add_systems(Startup, (sandbox_setup, setup).chain())
        // .add_systems(PostStartup, post_startup)
		// .add_systems(First, first)
        // .add_systems(PreUpdate, pre_update)
        // .add_systems(StateTransition, state_transition)
        .add_systems(FixedUpdate, (resolve_velocity, advance_fever).chain()) // framerate-independent, predictable simulation
		.add_systems(Update, ((sandbox_update, sandbox_clear_sent_messages, sandbox_process_removal_targets, update_finger).chain(), (on_dark_mode_enabled_changed, update_sent_message_colors, print_messages_after_dark_mode_change).chain().run_if(resource_changed::<DarkModeEnabled>))) // visuals, user input, and per-frame logic
        // .add_systems(PostUpdate, post_update)
        // .add_systems(Last, last)

		// .add_observer(play_keypress_sound)
		.run();
}

fn on_dark_mode_enabled_changed(dark_mode_enabled: Res<DarkModeEnabled>, mut color_scheme: ResMut<ColorScheme>) {
	let dark = dark_mode_enabled.0;
	println!("\nchanging scheme colors! dark mode enabled? {dark}");
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

// For now, the only entities with Text are those created via SentMessageBundle.
// If that changes, we will need to make this filter more specific.
fn update_sent_message_colors(
	mut msgs: Query<(&mut FontColor, &mut BkgColor, &Mine), With<Text>>,
	color_scheme: Res<ColorScheme>,
) {
	for (mut font_color, mut bkg_color, mine) in &mut msgs {
		font_color.0 = if mine.0 { color_scheme.my_text_color } else { color_scheme.their_text_color };
		bkg_color.0 = if mine.0 { color_scheme.my_bubble_color } else { color_scheme.their_bubble_color };
	}
}

fn print_messages_after_dark_mode_change(
	msgs: Query<(&Text, &FontColor, &BkgColor, &Mine, &Side, &Index)>
) {
	for (text, font_color, bkg_color, mine, side, index) in &msgs {
	// for msg in &msgs {
		// println!("\nText: {}\nFontColor: {:?}\nBkgColor: {:?}\nSide: {:?}\nIndex: {}", text.0, font_color.0, bkg_color.0, side.0, index.0);
		print_sent_message(Some(text), Some(font_color), Some(bkg_color), Some(mine), Some(side), Some(index));
	}
}

// // A mutable Query allows changing the iterated items.
// fn update_people(mut query: Query<&mut Name, With<Person>>) {
// 	for mut name in &mut query {
// 		if name.0 == "Elaina Proctor" {
// 			name.0 = String::from("Elaina Hume");
// 		}
// 	}
// }

// This resource tracks the currently selected color mode (i.e. light, dark).
#[derive(Resource)]
struct DarkModeEnabled(bool);	// Can we set up an observer for when this changes?
impl Default for DarkModeEnabled {
	fn default() -> Self {
		Self(true)
	}
}

// Components of a (past) text message entity: Text, FontColor, BkgColor, Side.

#[derive(Component, Debug)]
struct Text(String);

#[derive(Component, Debug)]
struct FontColor(Color);

#[derive(Component, Debug)]
struct BkgColor(Color);

#[derive(Component, Debug)]
struct Mine(bool);					// Is this message mine? Did I send it?

#[derive(Debug, PartialEq, Eq)]
enum HDir { LEFT, RIGHT, }
#[derive(Component, Debug)]
struct Side(HDir);

impl fmt::Display for HDir {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			HDir::LEFT => { "LEFT" },
			HDir::RIGHT => { "RIGHT" },
		})
	}
}

#[derive(Component, Debug, PartialEq, Eq)]
struct Index(usize);					// A custom index that we can set to an incrementing Resource<usize> value.
										// Helpful (say) to order text messages when displaying.

#[derive(Resource)]
struct SentMessageNextIndex(usize);			// Aforementioned next-index tracking Resource for SentMessage bundles.
impl Default for SentMessageNextIndex {
	fn default() -> Self {
		Self(0)
	}
}

#[derive(Bundle, Debug)]
struct SentMessageBundle {
	text: Text,
	font_color: FontColor,
	bkg_color: BkgColor,
	mine: Mine,
	side: Side,
	index: Index,
}
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
			self.mine.0,
			self.side.0,
			self.index.0,
		)
	}
}

fn print_sent_message(
	text: Option<&Text>,
	font_color: Option<&FontColor>,
	bkg_color: Option<&BkgColor>,
	mine: Option<&Mine>,
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

	if let Some(mine) = mine {
		println!("Mine? {}", mine.0);
	}

	if let Some(side) = side {
		println!("Side: {}", side.0);
	}

	if let Some(index) = index {
		println!("Index: {}", index.0);
	}
}

#[derive(Component)]
struct PreserveOnClear;		// Add this along with other components/bundles when spawning, and use to filter out removal targets.

#[derive(Component)]
struct RemovalTarget(Entity);		// We can spawn a set of entities that store ids of other entities.
									// An example shows how to store ids in these at spawn
									// and retrieve them later to despawn target entities;
									// in our case I think we'd rather filter using With<PreserveOnClear>
									// but this may be a helpful paradigm for more dynamic removal.
									// I.e., we can spawn a RemovalTarget with the id of any entity
									// to then (in a predetermined phase of the loop / schedule) remove them,
									// much like a queue_free in Godot.

// #[derive(Component)]
// struct Mine(bool);	// sender/ownership ought determine Side and default colors...

// Maybe a function that we can pass 'commands' into that we call from setup / system-function,
// that spawns a msg according to a string [slice ref] and a bool (is_mine).

// And maybe the current my/their colors are Resources just like DarkModeEnabled, along with other global state.

// Might want to store a global message index to order them above the typing area -
// simply increment when we spawn one, and reset if resetting the playing field.
// Would be ideal to have this happen automatically on spawning or creating one, but
// (1) this likely requires use of a ctor which we haven't had to do for components or bundles yet, and
// (2) then we also have to worry about dangling... messages. Damn it, still have to worry about that.
//  Well, for (2) we can just run a query when we clear the field and nuke all of them whose index is above X.

// Sandbox systems to play in.

// Utility function to spawn a message so we aren't repeating ourselves quite so much.

fn spawn_sent_message(
	commands: &mut Commands,
	next_index: &mut ResMut<SentMessageNextIndex>,
	color_scheme: & Res<ColorScheme>,
	text: &'static str,
	mine: bool,
	preserve_on_clear: bool,
) {
	let bundle = SentMessageBundle {
		text: Text(String::from(text)),
		font_color: FontColor(if mine { color_scheme.my_text_color } else { color_scheme.their_text_color }),
		bkg_color: BkgColor(if mine { color_scheme.my_bubble_color } else { color_scheme.their_bubble_color },),
		mine: Mine(mine),
		side: Side(if mine { HDir::RIGHT } else { HDir::LEFT }),
		index: Index(next_index.0),
	};
	println!("\nSpawning message: {}", bundle);
	let mut id = commands.spawn(bundle);
	if preserve_on_clear {
		id.insert(PreserveOnClear);
	}
	next_index.0 += 1;
}

fn sandbox_setup(
	dark_mode_enabled: Res<DarkModeEnabled>,
	mut next_index: ResMut<SentMessageNextIndex>,
	color_scheme: Res<ColorScheme>,
	mut commands: Commands
) {
	println!("\nDark Mode Enabled? {}", dark_mode_enabled.0);
	
	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "Signing off for today", true, true);

	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "Roger. See you tomorrow.", false, true);
	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "FYI, you're leading standups.", false, true);

	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "Ok, on it", true, true);
	
	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "You got this! 😎", false, true);

	// commands.remove_resource::<DarkModeEnabled>(); // This will cause a panic.
}

// Below: an example of querying via parameters that may or may not be present on each matching entity.

// fn output_players(players: Query<(&Name, &Hp, Option<&Mp>)>) {
// 	println!("All players:");
// 	for (name, hp, mp) in &players {
// 		print!("Name: {}, HP: {}", name.0, hp.0);
// 		match mp {
// 			Some(v) => println!(", MP: {}", v.0),
// 			None => println!(),
// 		}
// 	}
// 	println!();
// }

// Below: Examples of filtering using With and Without:

// fn output_wizards(wizards: Query<(&Name, &Hp), With<Mp>>) {	// More performant if we don't need to access MP component.
// 	println!("All wizards:");
// 	for (name, hp) in &wizards {
// 		println!("Name: {}, HP: {}", name.0, hp.0);
// 	}
// 	println!();
// }

// fn output_soldiers(soldiers: Query<(&Name, &Hp), Without<Mp>>) {
// 	println!("All soldiers:");
// 	for (name, hp) in &soldiers {
// 		println!("Name: {}, HP: {}", name.0, hp.0);
// 	}
// 	println!();
// }

fn sandbox_update(
	mut dark_mode_enabled: ResMut<DarkModeEnabled>,
	// msgs: Query<(Entity, &Text, &FontColor, &BkgColor, &Side)>,
	msgs: Query<(Entity, &Text, &FontColor, &BkgColor, &Mine, &Side, &Index)>,
	mut commands: Commands
) {
	if dark_mode_enabled.0 {
		dark_mode_enabled.0 = false;
		println!("\nDark Mode Enabled? {}", dark_mode_enabled.0);

		for (id, text, font_color, bkg_color, mine, side, index) in &msgs {
		// for msg in &msgs {
			// println!("\nText: {}\nFontColor: {:?}\nBkgColor: {:?}\nSide: {:?}\nIndex: {}", text.0, font_color.0, bkg_color.0, side.0, index.0);
			print_sent_message(Some(text), Some(font_color), Some(bkg_color), Some(mine), Some(side), Some(index));

			if side.0 == HDir::RIGHT {
				commands.spawn(RemovalTarget(id));
			}
		}
	}
}

// A system that despawns all entities with a Text component (for now, that's just SentMessage)
// unless they possess a PreserveOnClear component.
fn sandbox_clear_sent_messages(mut commands: Commands, msgs: Query<(Entity, &Text), Without<PreserveOnClear>>) {
    for (id, text) in &msgs {
        println!("\nRemoving message with text: {}", text.0);
        commands.entity(id).despawn();
    }
}

// A method of removal that can be more dynamically targeted - does not respect PreserveOnClear.
fn sandbox_process_removal_targets(mut commands: Commands, targets: Query<(Entity, &RemovalTarget)>) {
    for (removal_target, id) in &targets {
		// Check first if the entity is still valid / not despawned.
		// commands.get_entity(Entity) will return (in a Result)
		// the specific entity's commands object.
		// Inner portion of Result pattern marked mutable because needed to call despawn().
		if let Ok(mut entity_commands) = commands.get_entity(id.0) {
			entity_commands.despawn();
			println!("\nRemoved message entity with id: {}", id.0);
		} else {
			println!("\nSkipped removal of entity with id: {}", id.0);
		}
		// Either way, remove the targeting entity as its purpose is complete.
		commands.entity(removal_target).despawn();
        println!("\nRemoved targeting entity with id: {}", removal_target);
    }
}

// Below is an example of retrieving components in a mutable state.
// fn mut_update_sandbox(
// 	mut dark_mode_enabled: ResMut<DarkModeEnabled>,
// 	mut msgs: Query<(&Text, &mut FontColor)>
// ) {
// 	println!("\nChanging font color");
// 	for (text, mut color) in &mut msgs {
// 		color.0 = Color::BLACK;
// 		println!("\nText: {}\nFontColor: {:#?}", text.0, color.0);
// 	}
// }

fn setup() {

}

// Update transforms based on linear [and angular] velocity of entities such as roaming keys, falling teeth, sliding/melting letters. 
fn resolve_velocity() {}

// Move the finger/hand shadow/silhouette/sprite to track the cursor (or move elsewise when it doesn't).
fn update_finger() {}

// This resource tracks the player's progress through feverish events.
#[derive(Resource)]
struct FeverLevel(usize);
// Check whether conditions are met (time elapsed, typing progress made, etc)
// to trigger the next feverish event (and if so, initiate it).
fn advance_fever(mut fever_level: ResMut<FeverLevel>) {
	// The . method call syntax auto-dereferences - equivalent to (*fever_level).0.
	// Alternatively we could derive Deref/DerefMut traits on the FeverLevel resource,
	// but then we have to double dereference since no . means no auto dereference,
	// hence the first * is to unwrap the ResMut<T>, and the second is to get at the FeverLevel-wrapped value.
	// When docs etc say that Deref lets you treat the wrapper as if it were the wrapped value,
	// I think that is to say that if you are using the . operator this seems true (but IMO is misleading).
	// Thus, I am not deriving Deref on FeverLevel (it's not really useful unless I am using 
	// the . syntax for some other reason, like to call a method of the wrapped value).
	match fever_level.0 {
		0 => {
			// e.g. if enough time elapsed OR player made enough typing progress:
			if true || true {
				fever_level.0 += 1;
				// Also trigger feverish event 0.
			}
		}
		1 => {}
		_ => {}
	}
}

// fn output_txts (query: Query<&MessageText, With<Alignment>>) {
// 	// for txt in query.
// }

// pub struct HelloPlugin;
// impl Plugin for HelloPlugin {
// 	fn build(&self, app: &mut App) {
// 		app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
// 		app.add_systems(Startup, add_people);
// 		app.add_systems(Update, (update_people, greet_people).chain());
// 	}
// }

// fn _hello_world() {
// 	println!("Hello, world!");
// }

// #[derive(Component)]
// struct Person;

// #[derive(Component)]
// struct Name(String);

// fn add_people(mut commands: Commands) {
//     commands.spawn((Person, Name(String::from("Elaina Proctor"))));
//     commands.spawn((Person, Name("Renzo Hume".to_string())));
//     commands.spawn((Person, Name("Zayna Nieves".to_string())));
// }

// #[derive(Resource)]
// struct GreetTimer(Timer);

// // The Query parameter can be read as:
// // an iterator over every Name component on entities that also possess a Person component.
// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
// 	// update our timer with the time elapsed since the last update
//     // if that caused the timer to finish, we say hello to everyone
// 	if timer.0.tick(time.delta()).just_finished() {
// 		for name in &query {
// 			println!("hello, {}!", name.0);
// 		}
// 	}
// }