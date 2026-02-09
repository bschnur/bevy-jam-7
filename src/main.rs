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
// const DEFAULT_KEY_SIZE: Vec2 = Vec2::new(20., 20.);
// const DEFAULT_KEY_SPACING: f32 = 4.;

// Keybd Layout

// 	suggestions
// Q W E R T Y U I O P			10
// A S D F G H J K L			9
// ⇧ X C V B N M ⇐				9 (7 + 2 special)
// 123 😊 space ↩				4 (2 mini-special + spacebar + 1 extra-special)
// ⨁			🎙

// VIRTUAL_RESOLUTION.0 (1080) 		= 10 * DEFAULT_KEY_SIZE.0 + 9 * DEFAULT_KEY_SPACING + 2 * DEFAULT_ROW_1_MARGIN
// 									= 9 * DEFAULT_KEY_SIZE.0 + 8 * DEFAULT_KEY_SPACING + 2 * DEFAULT_ROW_2_MARGIN
// 									= 6 * DEFAULT_KEY_SIZE.0 + 5 * DEFAULT_KEY_SPACING + 2 * DEFAULT_ROW_3_INNER_MARGIN
// 										+ 2 * DEFAULT_SPECIAL_KEY_SIZE.0 + 2 * DEFAULT_ROW_3_OUTER_MARGIN
// 									= 2 * DEFAULT_MINI_SPECIAL_KEY_SIZE.0 + DEFAULT_SPACEBAR_SIZE.0 + DEFAULT_EX_SPECIAL_KEY_SIZE.0
// 										+ 3 * DEFAULT_KEY_SPACING + 2 * DEFAULT_ROW_4_MARGIN

// Assume:
// 			DEFAULT_ROW_1_MARGIN = DEFAULT_ROW_3_OUTER_MARGIN = DEFAULT_ROW_4_MARGIN
// 			DEFAULT_EX_SPECIAL_KEY_SIZE.0 = DEFAULT_KEY_SIZE.0 + DEFAULT_ROW_3_INNER_MARGIN + DEFAULT_SPECIAL_KEY_SIZE.0
// 			

// Rewriting the above:

// VIRTUAL_RESOLUTION.0 (1080) 		= 10 * DEFAULT_KEY_SIZE.0 + 9 * DEFAULT_KEY_SPACING + 2 * DEFAULT_ROW_1_MARGIN
// 									= 9 * DEFAULT_KEY_SIZE.0 + 8 * DEFAULT_KEY_SPACING + 2 * DEFAULT_ROW_2_MARGIN
// 									= 6 * DEFAULT_KEY_SIZE.0 + 5 * DEFAULT_KEY_SPACING + 2 * DEFAULT_ROW_3_INNER_MARGIN
// 										+ 2 * DEFAULT_SPECIAL_KEY_SIZE.0 + 2 * DEFAULT_ROW_1_MARGIN
// 									= 2 * DEFAULT_MINI_SPECIAL_KEY_SIZE.0 + DEFAULT_SPACEBAR_SIZE.0 + (DEFAULT_KEY_SIZE.0 + DEFAULT_ROW_3_INNER_MARGIN + DEFAULT_SPECIAL_KEY_SIZE.0)
// 										+ 3 * DEFAULT_KEY_SPACING + 2 * DEFAULT_ROW_1_MARGIN

// Rename for calculation:
// 				DEFAULT_KEY_SIZE.0 => 					X
// 				DEFAULT_KEY_SPACING => 					Y
// 				DEFAULT_ROW_1_MARGIN =>					Z
//				DEFAULT_ROW_2_MARGIN => 				A
// 				DEFAULT_ROW_3_INNER_MARGIN => 			B
// 				DEFAULT_SPECIAL_KEY_SIZE.0 => 			C
// 				DEFAULT_MINI_SPECIAL_KEY_SIZE.0 => 		D
// 				DEFAULT_SPACEBAR_SIZE.0 => 				E


// VIRTUAL_RESOLUTION.0 (1080) 		= 10X + 9Y + 2Z
// 									= 9X + 8Y + 2A
// 									= 7X + 6Y + 2B + 2C + 2Z
// 									= 3D + E + X + B + 3Y + 2Z

// Comparing the right side equivalent sums above,
// AND assuming:

//	2D + Y = C + B + X
// X = 90
// Y = 16
// Z = 18
// => 2D + 16 = C + B + 90	=>	2D = C + B + 74

// 	=>	1080 = 9(90) + 8(16) + 2A	=>	1080 = 810 + 128 + 2A	=>	142 = 2A	=>												A = 71
// 	=>	1080 = 7(90) + 6(16) + 2B + 2C + 2(18)	=>	1080 = 630 + 96 + 2(B + C) + 36	=>	2(B + C) = 318	=>	B + C = 159

// 	=>	2D = 159 + 74	=>																										D = 116.5

// 	=>	1080 = 3(116.5) + E + 90 + B + 3(16) + 2(18)	=>	1080 = 349.5 + E + 90 + B + 48 + 36		=>	556.5 = E + B
// 	=>	556.5 - E + C = 159		=>	C + 397.5 = E

// 7X + 6Y + 2B + 2C + 2Z = 3D + E + X + B + 3Y + 2Z
// 6X + 3Y + B + 2C = 3D + E
// 6(90) + 3(16) + B + 2C = 3(116.5) + E
// 540 + 48 + B + 2C = 349.5 + E
// 238.5 + B + 2C = E	=>	238.5 + B + 2C = (C + 397.5)	=>	B + C = 159		fuck.

// B + C = 159
// B + E = 556.5
// E - C = 397.5

// Let's assume C = D + 5.5 = 122.

// B + 122 = 159		=>		B = 37
// B + E = 556.5		=>		E = 519.5
// E - 118.5 = 397.5	=>		519.5 - 122 = 397.5 (consistent).

// That gives us:

// 				DEFAULT_KEY_SIZE.0 => 					90
// 				DEFAULT_KEY_SPACING => 					16
// 				DEFAULT_ROW_1_MARGIN =>					18
//				DEFAULT_ROW_2_MARGIN => 				71
// 				DEFAULT_ROW_3_INNER_MARGIN => 			37
// 				DEFAULT_SPECIAL_KEY_SIZE.0 => 			122
// 				DEFAULT_MINI_SPECIAL_KEY_SIZE.0 => 		116.5
// 				DEFAULT_SPACEBAR_SIZE.0 => 				519.5

const DEFAULT_KEY_HEIGHT: f32 = 116.;

const DEFAULT_KEY_SIZE: Vec2 = Vec2::new(90., DEFAULT_KEY_HEIGHT);
const DEFAULT_SPECIAL_KEY_SIZE: Vec2 = Vec2::new(122., DEFAULT_KEY_HEIGHT);
const DEFAULT_MINI_SPECIAL_KEY_SIZE: Vec2 = Vec2::new(116.5, DEFAULT_KEY_HEIGHT);
const DEFAULT_SPACEBAR_SIZE: Vec2 = Vec2::new(519.5, DEFAULT_KEY_HEIGHT);
const DEFAULT_KEY_SPACING: f32 = 16.;

const DEFAULT_ROW_1_MARGIN: f32 = 18.;
const DEFAULT_ROW_2_MARGIN: f32 = 71.;
const DEFAULT_ROW_3_INNER_MARGIN: f32 = 37.;

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