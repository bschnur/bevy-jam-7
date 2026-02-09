use bevy::prelude::*;

// Colors

// Bubble colors
const BLUE_BUBBLE_COLOR: Color = Color::srgb(2./255., 129./255., 253./255.);			// #0281FD
const GREEN_BUBBLE_COLOR: Color = Color::srgb(51./255., 206./255., 90./255.);			// #33CE5A
const WHITE_TEXT_COLOR: Color = Color::WHITE;											// #FFFFFF
// Darkmode colors
const DKMODE_BKG_COLOR: Color = Color::BLACK;											// #000000 (bkg color main area)
const DKMODE_SYS_TEXT_COLOR: Color = Color::srgb(118./255., 118./255., 118./255.);		// #767676 (Read, Sent, Delivered, date/time, etc.)
const DKMODE_KEY_COLOR: Color = Color::srgb(96./255., 96./255., 96./255.);				// #606060 (most keys)
const DKMODE_CAPS_COLOR: Color = Color::srgb(209./255., 209./255., 209./255.);			// #D1D1D1 (shift/caps)
const DKMODE_BKSP_COLOR: Color = Color::srgb(59./255., 59./255., 59./255.);				// #3B3B3B (123, return, bksp)
const DKMODE_KEYBD_COLOR: Color = Color::srgb(27./255., 27./255., 27./255.);			// #1B1B1B (keybd bkg)
const DKMODE_THEIR_BUBBLE_COLOR: Color = Color::srgb(38./255., 38./255., 42./255.);		// #26262A
const DKMODE_THEIR_TEXT_COLOR: Color = Color::WHITE;									// #FFFFFF
// Lightmode colors
const LTMODE_BKG_COLOR: Color = Color::WHITE;											// #FFFFFF (bkg color main area)
const LTMODE_SYS_TEXT_COLOR: Color = Color::srgb(118./255., 118./255., 118./255.);		// #767676 (Read, Sent, Delivered, date/time, etc.)
const LTMODE_KEY_COLOR: Color = Color::WHITE;											// #FFFFFF (most keys)
const LTMODE_CAPS_COLOR: Color = LTMODE_KEY_COLOR;										// #FFFFFF (shift/caps)
const LTMODE_BKSP_COLOR: Color = LTMODE_KEY_COLOR;										// #FFFFFF (123, return, bksp)
const LTMODE_KEYBD_COLOR: Color = Color::srgb(227./255., 229./255., 230./255.);			// #E3E5E6 (keybd bkg)
const LTMODE_THEIR_BUBBLE_COLOR: Color = Color::srgb(233./255., 233./255., 234./255.);	// #E9E9EA
const LTMODE_THEIR_TEXT_COLOR: Color = Color::BLACK;									// #000000
// Default colors
const DEFAULT_BKG_COLOR: Color = DKMODE_BKG_COLOR;
const DEFAULT_SYS_TEXT_COLOR: Color = DKMODE_SYS_TEXT_COLOR;
const DEFAULT_KEY_COLOR: Color = DKMODE_KEY_COLOR;
const DEFAULT_CAPS_COLOR: Color = DKMODE_CAPS_COLOR;
const DEFAULT_BKSP_COLOR: Color = DKMODE_BKSP_COLOR;
const DEFAULT_KEYBD_COLOR: Color = DKMODE_KEYBD_COLOR;
const DEFAULT_MY_BUBBLE_COLOR: Color = BLUE_BUBBLE_COLOR;
const DEFAULT_MY_TEXT_COLOR: Color = WHITE_TEXT_COLOR;
const DEFAULT_THEIR_BUBBLE_COLOR: Color = DKMODE_THEIR_BUBBLE_COLOR;
const DEFAULT_THEIR_TEXT_COLOR: Color = DKMODE_THEIR_TEXT_COLOR;

// Sizes

// The resolution we are pretending to render at / rendering at before scaling.
const VIRTUAL_RESOLUTION: Vec2 = Vec2::new(1080., 1920.);
// The below sizes are calculated based on the virtual resolution.
// Lots of things marked DEFAULT with the intention being they may be substituted for.
const DEFAULT_BUBBLE_CORNER_RADIUS: f32 = 10.;

// Keybd Layout

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
		.init_resource::<DarkModeEnabled>()

		.insert_resource(ClearColor(DEFAULT_BKG_COLOR)) // bevy built-in Resource, used for window clearing - might not use

        // .add_systems(PreStartup, pre_startup)
        .add_systems(Startup, (setup_sandbox, setup).chain())
        // .add_systems(PostStartup, post_startup)
		// .add_systems(First, first)
        // .add_systems(PreUpdate, pre_update)
        // .add_systems(StateTransition, state_transition)
        .add_systems(FixedUpdate, (resolve_velocity, advance_fever).chain()) // framerate-independent, predictable simulation
		.add_systems(Update, (update_sandbox, update_finger).chain()) // visuals, user input, and per-frame logic
        // .add_systems(PostUpdate, post_update)
        // .add_systems(Last, last)

		// .add_observer(play_keypress_sound)
		.run();
}

// This resource tracks the currently selected color mode (i.e. light, dark).
#[derive(Resource)]
struct DarkModeEnabled(bool);	// Can we set up an observer for when this changes?
impl Default for DarkModeEnabled {
	fn default() -> Self {
		Self(true)
	}
}

// Components of a (past) text message entity: Text, FontColor, BkgColor, Side.

#[derive(Component)]
struct Text(String);

#[derive(Component)]
struct FontColor(Color);		// Put this in a tuple struct with the string?

#[derive(Component)]
struct BkgColor(Color);

#[derive(Debug)]
enum HDir { LEFT, RIGHT, }
#[derive(Component)]
struct Side(HDir);

#[derive(Bundle)]
struct SentMessage {
	text: Text,
	font_color: FontColor,
	bkg_color: BkgColor,
	side: Side,
}

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

fn setup_sandbox(dark_mode_enabled: Res<DarkModeEnabled>, mut commands: Commands) {
	println!("Dark Mode Enabled? {}", dark_mode_enabled.0);
	
	// commands.spawn((
	// 	Text(String::from("Signing off for today")),
	// 	FontColor(DEFAULT_MY_TEXT_COLOR),
	// 	BkgColor(DEFAULT_MY_BUBBLE_COLOR),
	// 	Side(HDir::RIGHT)
	// ));

	commands.spawn(SentMessage {
		text: Text(String::from("Signing off for today")),
		font_color: FontColor(DEFAULT_MY_TEXT_COLOR),
		bkg_color: BkgColor(DEFAULT_MY_BUBBLE_COLOR),
		side: Side(HDir::RIGHT)
	});

	commands.spawn(SentMessage {
		text: Text(String::from("Roger. See you tomorrow.")),
		font_color: FontColor(DKMODE_THEIR_TEXT_COLOR),
		bkg_color: BkgColor(DEFAULT_THEIR_BUBBLE_COLOR),
		side: Side(HDir::LEFT)
	});
	commands.spawn(SentMessage {
		text: Text(String::from("FYI, you're leading standups.")),
		font_color: FontColor(DKMODE_THEIR_TEXT_COLOR),
		bkg_color: BkgColor(DEFAULT_THEIR_BUBBLE_COLOR),
		side: Side(HDir::LEFT)
	});
	
	commands.spawn(SentMessage {
		text: Text(String::from("Ok, on it")),
		font_color: FontColor(DEFAULT_MY_TEXT_COLOR),
		bkg_color: BkgColor(DEFAULT_MY_BUBBLE_COLOR),
		side: Side(HDir::RIGHT)
	});

	commands.spawn(SentMessage {
		text: Text(String::from("You got this! 😎")),
		font_color: FontColor(DKMODE_THEIR_TEXT_COLOR),
		bkg_color: BkgColor(DEFAULT_THEIR_BUBBLE_COLOR),
		side: Side(HDir::LEFT)
	});

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

fn update_sandbox(
	mut dark_mode_enabled: ResMut<DarkModeEnabled>,
	msgs: Query<(&Text, &FontColor, &BkgColor, &Side)>
) {
	if dark_mode_enabled.0 {
		dark_mode_enabled.0 = false;
		println!("Dark Mode Enabled? {}", dark_mode_enabled.0);

		for (text, color, bkg, side) in &msgs {
			println!("\nText: {}\nFontColor: {:#?}\nBkgColor: {:#?}\nScreenSide: {:?}", text.0, color.0, bkg.0, side.0);
		}
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

// // A mutable Query allows changing the iterated items.
// fn update_people(mut query: Query<&mut Name, With<Person>>) {
// 	for mut name in &mut query {
// 		if name.0 == "Elaina Proctor" {
// 			name.0 = String::from("Elaina Hume");
// 		}
// 	}
// }