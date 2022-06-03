<<<<<<< HEAD
use bevy::ecs::component::Component;
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

/// Uncover component, indicates a covered tile that should be uncovered
#[cfg_attr(feature = "debug", derive(Inspectable))]
=======
use bevy::prelude::Component;

/// Uncover component, indicates a covered tile that should be uncovered
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
>>>>>>> 5844198 (part 4 OK)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncover;
