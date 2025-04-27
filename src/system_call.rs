use bevy::prelude::*;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub enum SystemCallAdjustment {
    #[default]
    NotImplemented,
}

#[derive(Debug)]
pub struct SystemCallPlugin;

impl Plugin for SystemCallPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
        // TODO maybe use extrasafe, contains more than just system_calls
    }
}
