use bevy::prelude::*;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum SystemCallAdjustment {
    #[default]
    Unknown,
}

#[derive(Debug)]
pub struct SystemCallPlugin;

impl Plugin for SystemCallPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<SystemCallAdjustment>();
        app.insert_resource(SystemCallAdjustment::Unknown);

        // TODO maybe use extrasafe, contains more than just system_calls
    }
}
