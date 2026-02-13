// =============================================================================
// Dependency (crate) scope imports
// =============================================================================

// use std::fmt;
use bevy::{
	prelude::*,
	window::WindowResolution,
};

// use bevy_vector_shapes::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;

// =============================================================================
// Module declarations and scope imports
// =============================================================================

mod window_utils;
mod cleanup;
mod component_utils;
mod app_state;
mod sent_message;
mod color_utils;

use window_utils::*;
use cleanup::*;
use component_utils::*;
use app_state::*;
use sent_message::*;
use color_utils::*;

// =============================================================================
// Color constants and structs - moved to color_utils.rs.
// =============================================================================

// =============================================================================
// Sizes
// =============================================================================

// The resolution we are pretending to render at / rendering at before scaling.
// const VIRTUAL_RESOLUTION: Vec2 = Vec2::new(1080., 1920.);
const VIRTUAL_RESOLUTION: UVec2 = UVec2::new(1080, 1920);

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

const WINDOW_SCALE: (bool, f32) = (true, 0.5);

// =============================================================================
// App Setup
// =============================================================================

fn main() {
	let mut app = App::new();

	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
	// Add plugins.
	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

	// Alter properties of added plugins when the defaults are not desired.

	app.add_plugins(DefaultPlugins.set(WindowPlugin {
		primary_window: Some(Window {
			title: String::from("Sick Day"),
			// Set resolution to VIRTUAL_RESOLUTION.x by VIRTUAL_RESOLUTION.y logical pixels
			// (may be resized with scale override during PostStartup)
			resolution: WindowResolution::from(VIRTUAL_RESOLUTION),
			// Set position (will be re-centered after aforementioned resize if window scale is overridden)
			position: WindowPosition::Centered(MonitorSelection::Primary),
			..default()
		}),
		..default()
	}))

	.add_plugins(Shape2dPlugin::default())

	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
	// Initialize Resource values.
	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

	// Use init_resource if (1) they implement Default and (2) we want the default values.
	// Otherwise use insert_resource and specify the initial value.

	.insert_resource(VirtualResolution(VIRTUAL_RESOLUTION))
	.insert_resource(WindowScaling(true, 0.5))
	.init_resource::<WindowAwaitsCentering>()

	.insert_resource(FeverLevel(0))

	.init_resource::<DarkModeEnabled>()
	.init_resource::<AndroidModeEnabled>()

	.init_resource::<ColorScheme>()

	.init_resource::<NextIndex>()

	.insert_resource(ClearColor(Color::BLACK)) // bevy built-in Resource, used for window clearing - might not use
	;

	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
	// Init and add systems for app states (and transistions there-betwixt).
	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

	app.init_state::<AppState>();

	// app.add_systems(OnExit(AppState::Splash), (do_something, cleanup_system::<Cleanup<Splash>>).chain());
	app.add_systems(OnExit(AppState::Splash), cleanup_system::<Cleanup<Splash>>);
	// app.add_systems(OnTransition(AppState::Splash), some_transition_system);
	// app.add_systems(OnTransition(AppState::Splash), some_transition_system);
	// app.add_systems(OnEnter(AppState::Splash), enter_system::<Enter<Splash>>);

	app.add_systems(OnExit(AppState::MainMenu), cleanup_system::<Cleanup<MainMenu>>);
	// app.add_systems(OnTransition(AppState::MainMenu), some_transition_system);
	// app.add_systems(OnEnter(AppState::MainMenu), enter_system::<Enter<MainMenu>>);

	app.add_systems(OnExit(AppState::InGame), cleanup_system::<Cleanup<InGame>>);
	// app.add_systems(OnTransition(AppState::InGame), some_transition_system);
	// app.add_systems(OnEnter(AppState::InGame), enter_system::<Enter<InGame>>);

	app.add_systems(OnExit(AppState::PauseMenu), cleanup_system::<Cleanup<PauseMenu>>);
	// app.add_systems(OnTransition(AppState::PauseMenu), some_transition_system);
	// app.add_systems(OnEnter(AppState::PauseMenu), enter_system::<Enter<PauseMenu>>);

	app.add_systems(OnExit(AppState::Won), cleanup_system::<Cleanup<Won>>);
	// app.add_systems(OnTransition(AppState::Won), some_transition_system);
	// app.add_systems(OnEnter(AppState::Won), enter_system::<Enter<Won>>);

	// To transition to a new state, pass the NextState resource (as mutable):
	// mut next_state: ResMut<NextState<AppState>>,
	// and in the system body, call e.g:
	// next_state.set(AppState::MainMenu);

	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
	// Add main loop systems.
	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

	// TODO: audit each use of chain() and other ordering/conditionality for necessity.

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// PreStartup, Startup, PostStartup: these run once, on app launch.
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	app.add_systems(PreStartup, (register_quit_cleanup_system, pre_startup).chain());
	app.add_systems(Startup, startup);
	app.add_systems(PostStartup, (init_window_resolution_scale_factor, post_startup).chain());

	// .........................................................................
	// RunMainLoop encompasses the rest of the built-in schedule labels:
	// First
	// PreUpdate
	// StateTransition
	// FixedFirst, FixedPreUpdate, FixedUpdate, FixedPostUpdate, FixedLast
	// Update
	// PostUpdate
	// Last
	// .........................................................................

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// First
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	app.add_systems(First, first);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// PreUpdate
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	app.add_systems(PreUpdate, pre_update);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// StateTransition
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	app.add_systems(StateTransition, state_transition);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// FixedUpdate (and surrounding labels): framerate-independent, predictable simulation
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	app.add_systems(FixedFirst, fixed_first)
	.add_systems(FixedPreUpdate, fixed_pre_update)
	.add_systems(FixedUpdate, (
		// resolve_velocity,
		advance_fever,
		fixed_update,
	).chain())
	.add_systems(FixedPostUpdate, fixed_post_update)
	.add_systems(FixedLast, fixed_last);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// Update: visuals, user input, and per-frame logic
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	app.add_systems(Update, (
		on_window_resized,
		// update_finger
	));
	#[cfg(debug_assertions)]
	app.add_systems(Update, (
		// (update_once).run_if(run_once),
		(
			update,
			sandbox_update,
			sandbox_clear_sent_messages,
		).chain(),
	));
	#[cfg(not(debug_assertions))]
	app.add_systems(Update, (
		update
	));

	app.add_systems(Update, (
		on_dark_mode_enabled_changed.run_if(
			resource_changed::<DarkModeEnabled>.and(not(resource_added::<DarkModeEnabled>))
		),

		on_android_mode_enabled_changed.run_if(
			resource_changed::<AndroidModeEnabled>.and(not(resource_added::<AndroidModeEnabled>))
		)));
	
	#[cfg(debug_assertions)]
	app.add_systems(Update,
		(
			update_colors_on_color_scheme_change,
			print_messages_on_color_scheme_change,
		).chain().run_if(resource_changed::<ColorScheme>.and(not(resource_added::<ColorScheme>)))
	);
	#[cfg(not(debug_assertions))]
	app.add_systems(Update,
		update_colors_on_color_scheme_change.run_if(
			resource_changed::<ColorScheme>.and(not(resource_added::<ColorScheme>))
		)
	);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// PostUpdate: similar role to Update but runs afterward.
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	app.add_systems(PostUpdate, (
		post_update,
		despawn_doomed_targets,
	));

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// Last: final schedule label encompassed by RunMainLoop.
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	app.add_systems(Last, last);

	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
	// Add observers.
	// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

	app.add_observer(on_key_pressed)

	.run();
}

// =============================================================================
// Systems
// =============================================================================

// ### Once: ###

fn pre_startup() {
}

fn startup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut next_index: ResMut<NextIndex>,
	color_scheme: Res<ColorScheme>,
) {
	commands.spawn((
		Name::new("SplashCamera"),
		Cleanup::<Splash>::new(),
		Camera2d::default(),
		Msaa::Off,
	));

	commands.spawn((
		Name::new("MusicPlayer"),
		Cleanup::<MainMenu>::new(),	// TODO: change this is the music should persist through to the InGame app state.
		AudioPlayer::new(asset_server.load("audio/music/chillopen.ogg")),
		PlaybackSettings::LOOP,
	));

	// TODO: instead of passing in a transform, the message spawning function
	// should handle placing a new message at a default bottom-edge alignment -
	// so a position based on bubble height, in turn based on message length -
	// and should set the x position based on Side and bubble width, in turn based on message length.
	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "Signing off for today", true, true, Some(Transform::from_xyz(400., 200., 0.)));
	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "Roger. See you tomorrow.", false, true, None);
	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "FYI, you're leading standups.", false, true, None);
	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "Ok, on it", true, true, None);
	spawn_sent_message(&mut commands, &mut next_index, &color_scheme, "You got this! 😎", false, true, None);

	// TODO: adapt the below to draw a message bubble per sent_message
	// commands.spawn(
    //     ShapeBundle::rect(
    //         &ShapeConfig {
    //             color: BLUE_BUBBLE_COLOR,
    //             corner_radii: Vec4::splat(40.),
    //             ..ShapeConfig::default_2d()
    //         },
    //         Vec2::new(600., 80.),
    //     ),
    // );
}

fn post_startup() {}

// ### Each pass through the RunMainLoop schedule label: ###

fn first() {}

fn pre_update() {}

fn state_transition() {}

fn fixed_first() {}
fn fixed_pre_update() {}
fn fixed_update() {}
fn fixed_post_update() {}
fn fixed_last() {}

fn update(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut exit: MessageWriter<AppExit>,
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
        // Request to exit program.
		exit.write(AppExit::Success);
    }
}

fn post_update(
	mut commands: Commands,
	mut exit_messages: MessageReader<AppExit>,
	quit_cleanup_system_id: Res<QuitCleanupSystemId>,
) {
	for _msg in exit_messages.read() {
		// Program exit requested. Run pre-quit cleanup.
		commands.run_system(quit_cleanup_system_id.0);
		break;
	}
}

fn last() {}

// =============================================================================
// Window size and position - Moved to window_utils.rs.
// =============================================================================

// =============================================================================
// Events, observers, and reactions
// =============================================================================

#[derive(Event, Debug)]
struct KeyTap {
	glyph: char
}

fn on_key_pressed(event: On<KeyTap>) {
	println!("Key pressed: {}", event.glyph);
	// TODO Play sound
	// TODO type character in field
}

// =============================================================================
// General use components and related functions - moved to component_utils.rs
// =============================================================================

// =============================================================================
// Components/bundle/utilities for the messages logged above the typing area.
// 		- moved to sent_message.rs
// =============================================================================

// =============================================================================
// FeverLevel
// =============================================================================

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

// =============================================================================
// Miscellaneous Stubs
// =============================================================================

// Update transforms based on linear [and angular] velocity of entities such as roaming keys, falling teeth, sliding/melting letters. 
fn _resolve_velocity() {}

// Move the finger/hand shadow/silhouette/sprite to track the cursor (or move elsewise when it doesn't).
fn _update_finger() {}

// =============================================================================
// Sandbox / testing
// =============================================================================

// A system that despawns all entities with a Text component (for now, that's just SentMessage)
// unless they possess a PreserveOnClear component.
// TODO: remove sandbox systems from schedule
fn sandbox_clear_sent_messages(mut commands: Commands, msgs: Query<(Entity, &MsgText), Without<PreserveOnClear>>) {
    for (id, text) in &msgs {
        println!("\nRemoving message with text: {}", text.0);
        commands.entity(id).despawn();
    }
}

// TODO: remove sandbox systems from schedule
#[cfg(debug_assertions)]
fn sandbox_update(
	mut dark_mode_enabled: ResMut<DarkModeEnabled>,
	mut android_mode_enabled: ResMut<AndroidModeEnabled>,
	// msgs: Query<(Entity, &Text, &FontColor, &BkgColor, &Side)>,
	_msgs: Query<(Entity, &MsgText, &FontColor, &BkgColor, &IsMine, &Side, &Index)>,
	mut commands: Commands,
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	// if dark_mode_enabled.0 {
	// 	dark_mode_enabled.0 = false;
	// 	println!("\nDark Mode Enabled? {}", dark_mode_enabled.0);

	// 	// for (id, _text, _font_color, _bkg_color, _is_mine, side, _index) in &msgs {
	// 		// print_sent_message(Some(text), Some(font_color), Some(bkg_color), Some(is_mine), Some(side), Some(index));

	// 		// if side.0 == HDir::RIGHT {
	// 		// 	commands.spawn(RemovalTarget(id));
	// 		// }
	// 	// }
	// }

	if keyboard_input.just_pressed(KeyCode::KeyD) {
		dark_mode_enabled.0 = !dark_mode_enabled.0;
		println!("\nDEBUG: dark mode toggle ({})", dark_mode_enabled.0);
	}

	if keyboard_input.just_pressed(KeyCode::KeyA) {
		android_mode_enabled.0 = !android_mode_enabled.0;
		println!("\nDEBUG: Android mode toggle ({})", android_mode_enabled.0);
	}

	if keyboard_input.just_pressed(KeyCode::KeyF) {
		commands.trigger(KeyTap { glyph: 'f' });
	}
}

// =============================================================================
// Examples
// =============================================================================

// Custom plugin example

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

// Example: a mutable Query allows changing the iterated items.

// fn update_people(mut query: Query<&mut Name, With<Person>>) {
// 	for mut name in &mut query {
// 		if name.0 == "Elaina Proctor" {
// 			name.0 = String::from("Elaina Hume");
// 		}
// 	}
// }

// Example: rotating a 3d shape in an Update system

// fn update_shapes(time: Res<Time>, mut shapes: Query<&mut Transform, With<ShapeMaterial>>) {
//     shapes.iter_mut().for_each(|mut tf| {
//         tf.rotate_local_z(time.delta_secs());
//     })
// }

// Example: using multiple bundles to spawn a single entity.

// Chain approach: later bundles override matching fields in earlier ones.
// commands.spawn(BundleA::default()).insert(BundleB::default());

// Tuple approach: any overlap in bundle fields may cause a panic.
// commands.spawn((BundleA::default(), BundleB::default()));

// Nest approach: Nest bundles inside another outer one.

// #[derive(Bundle)]
// struct CombinedEntityBundle {
//     // Other components
//     marker: MyMarkerComponent,
//     // Nested bundles
//     sprite_bundle: SpriteBundle,
//     text_bundle: TextBundle,
// }

// commands.spawn(CombinedEntityBundle {
//     marker: MyMarkerComponent,
//     sprite_bundle: SpriteBundle::default(),
//     text_bundle: TextBundle::default(),
//     // ...
// });