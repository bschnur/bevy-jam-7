use bevy::prelude::States;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    InGame,
    PauseMenu,
    Won,
}

// Need to knopw about Cleanup to apply treait bound to outer generic type (see below).
use crate::cleanup::Cleanup;

// ZST / marker types for use with the generic cleanup function found in component_utils.rs.
// (These are used to give the generic a type, which lets it target entities
// possessing the corresponding cleanup marker / component.) 

#[derive(Debug, Eq, PartialEq)]
pub struct Splash;

#[derive(Debug, Eq, PartialEq)]
pub struct MainMenu;

#[derive(Debug, Eq, PartialEq)]
pub struct InGame;

#[derive(Debug, Eq, PartialEq)]
pub struct PauseMenu;

#[derive(Debug, Eq, PartialEq)]
pub struct Won;

// This empty trait is a marker to make sure that we can restrict generics
// to allow only these types, i.e. when we want to operate on an AppState-associated type
// (remember, unlike the enum values these struct are otherwise not bound together in any way).

pub trait AppStateTrait {}

impl AppStateTrait for Cleanup<Splash> {}
impl AppStateTrait for Cleanup<MainMenu> {}
impl AppStateTrait for Cleanup<InGame> {}
impl AppStateTrait for Cleanup<PauseMenu> {}
impl AppStateTrait for Cleanup<Won> {}