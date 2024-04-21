use bevy::prelude::*;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum NetworkAdjustment {
    #[default]
    Unknown,
}

#[derive(Debug)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<NetworkAdjustment>();
        app.insert_resource(NetworkAdjustment::Unknown);
    }
}
