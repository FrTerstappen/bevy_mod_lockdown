use bevy::prelude::*;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum NetworkAdjustment {
    #[default]
    NotImplemented,
}

#[derive(Debug)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
        // TODO landlock now has initial network support
    }
}
