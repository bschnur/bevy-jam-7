use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(HelloPlugin)
		.run();

	// Add multiple systems to a schedule by passing in a tuple.
	// Control the order of systems' execution by further wrapping them in a tuple and calling chain(). 
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
		app.add_systems(Startup, add_people);
		app.add_systems(Update, (update_people, greet_people).chain());
	}
}

fn _hello_world() {
	println!("Hello, world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(String::from("Elaina Proctor"))));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

// The Query parameter can be read as:
// an iterator over every Name component on entities that also possess a Person component.
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
	// update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
	if timer.0.tick(time.delta()).just_finished() {
		for name in &query {
			println!("hello, {}!", name.0);
		}
	}
}

// A mutable Query allows changing the iterated items.
fn update_people(mut query: Query<&mut Name, With<Person>>) {
	for mut name in &mut query {
		if name.0 == "Elaina Proctor" {
			name.0 = String::from("Elaina Hume");
		}
	}
}