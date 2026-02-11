use bevy::prelude::{
	Component,
	Entity,
	Query,
	Commands,
};

// =============================================================================
// General use components and related functions
// =============================================================================

#[derive(Component)]
pub struct PreserveOnClear;		// Add this along with other components/bundles when spawning, and use to filter out removal targets.

#[derive(Component)]
pub struct Doomed(pub Entity);		// We can spawn a set of entities that store ids of other entities.
									// An example shows how to store ids in these at spawn
									// and retrieve them later to despawn target entities;
									// in our case I think we'd rather filter using With<PreserveOnClear>
									// but this may be a helpful paradigm for more dynamic removal.
									// I.e., we can spawn a RemovalTarget with the id of any entity
									// to then (in a predetermined phase of the loop / schedule) remove them,
									// much like a queue_free in Godot.

// A method of removal that can be more dynamically targeted - does not respect PreserveOnClear.
// While despawn() is safe to call mid frame, the below allows us to queue entities for removal at leisure
// and despawn them all in a wave at a time of our choosing (say, in response to an event via run_if).
pub fn despawn_doomed_targets(mut commands: Commands, targets: Query<(Entity, &Doomed)>) {
    for (doomed, id) in &targets {
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
		commands.entity(doomed).despawn();
        println!("\nRemoved targeting entity with id: {}", doomed);
    }
}