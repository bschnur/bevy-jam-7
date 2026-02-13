use std::marker::PhantomData;
use bevy::{
	prelude::{
		Component, Entity,
		Resource,
		Query, With,
		Commands,
	},
	ecs::system::SystemId,
};

use crate::app_state::AppStateTrait;

// Cleanup marker generic as alluded to in Bevy best practices:
// https://github.com/tbillington/bevy_best_practices
// And with inspiration from the Bevy cheatbook:
// https://bevy-cheatbook.github.io/patterns/generic-systems.html#example-cleanup
// Add one along with Name any time an entity is spawned,
// to indicate when it should be cleaned up.

// My twist: monomorphize the cleanup marker - say, with a GameState-associated marker struct / ZST.

#[derive(Component)]
pub struct Cleanup<T> {
	_marker: PhantomData<T>,
}
impl<T> Cleanup<T> {
    pub fn new() -> Self {
        Cleanup { _marker: PhantomData }
    }
}

// Use the marker to monomorphize the generic cleanup function below,
// then add that specific cleanup function as a system triggered on
// state exit transitions (see linked cheat sheet example).

// If an entity is not specific to a particular state and should only be cleaned up, say, on program exit,
// we won't want to mark them for cleanup on any particular state exit (or particular state-to-state transition).

// Thus, providing the following option:
#[derive(Debug, Eq, PartialEq)]
pub struct Quit;
// Note that we don't need to explicitly call despawn on such entities generally,
// as Bevy will despawn the whole World on exit, but there may be tasks associated with such entities.
// Or rather, with the act of exiting generally. (See quit_cleanup_system below.)

// Example:
// app.add_systems(OnExit(AppState::Splash), cleanup_system::<Cleanup<Splash>>);

// Notice that only components with our app state marker trait can be passed in to this generic. 
pub fn cleanup_system<T: Component + AppStateTrait>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn quit_cleanup_system(
	mut commands: Commands,
    query: Query<Entity, With<Cleanup<Quit>>>,
) {
	// Program exit requested. Perform any necessary cleanup.
	
	// TODO: audit our app quit cleanup needs (close streams, save persistent state, etc), and satisfy them here. 
	
	// IIUC calling despawn() is not necessary as Bevy will drop the World,
	// but it is probably not hurting anything to have our redundant handling in place.
	for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource)]
pub struct QuitCleanupSystemId(pub SystemId);

pub fn register_quit_cleanup_system(mut commands: Commands) {
    let system_id = commands.register_system(quit_cleanup_system);
    commands.insert_resource(QuitCleanupSystemId(system_id));
}