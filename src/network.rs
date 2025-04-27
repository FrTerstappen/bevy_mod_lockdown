use bevy::prelude::*;

#[derive(Resource, Reflect, Debug, Default)]
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
