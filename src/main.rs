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
const DEFAULT_KEY_BUTTON_SIZE: Vec2 = Vec2::new(20., 20.);
const DEFAULT_KEY_BUTTON_SPACING: f32 = 4.;

// Keybd Layout

// 	suggestions
// Q W E R T Y U I O P
// A S D F G H J K L
// ⇧ X C V B N M ⇐
// 123 😊 space ↩
// ⨁			🎙

// Notes on entities needed:
// messages, keyboard, drafting area, ..., Read/Delivered/Sent, time horizontal rule, key, finger, tooth, ghost text?, individual letters?

// Refresh on usefulness/need for bundles vs tupled components.

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(FeverLevel(0))
		.insert_resource(ClearColor(DEFAULT_BKG_COLOR))
		.add_systems(Startup, setup)
		.add_systems(FixedUpdate, (apply_velocity, move_finger, advance_fever).chain())
		// .add_systems(Update, update_hud) // something solely display related? check what is considered good practice to do in (varied-length) update
		// .add_observer(play_keypress_sound)
		.run();
}

enum ColorMode {
	LIGHT,
	DARK,
}

// This resource tracks the currently selected color mode (i.e. light, dark).
#[derive(Resource, Deref, DerefMut)]
struct CurrentMode(ColorMode);





#[derive(Component)]
struct Text(String);

enum Side {
	LEFT,
	RIGHT,
}

#[derive(Component)]
struct Alignment(Side);

fn setup(mut commands: Commands) {
	// commands.spawn((Text(String::from("Reminder, you're leading standups tomorrow.")), Alignment(Side::LEFT)));
	// commands.spawn((Text(String::from("No excuses")), Alignment(Side::LEFT)));
}

// Update transforms based on linear [and angular] velocity of entities such as roaming keys, falling teeth, sliding/melting letters. 
fn apply_velocity() {}

// Move the finger/hand shadow/silhouette/sprite to track the cursor (or move elsewise when it doesn't).
fn move_finger() {}

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
		_ => {}
	}
}

// fn output_txts (query: Query<&Text, With<Alignment>>) {
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