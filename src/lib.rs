#[cfg(feature = "egui")]
pub mod egui;

#[cfg(feature = "filesystem")]
pub mod filesystem;

#[cfg(feature = "network")]
pub mod network;

#[cfg(feature = "privilege")]
pub mod privilege;

#[cfg(feature = "system_call")]
pub mod system_call;

use bevy::prelude::*;

#[derive(Debug, SystemSet, Clone, PartialEq, Eq, Hash)]
pub enum LockdownSet {
    PreStartup,
    Startup,
    PostStartup,
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct LockdownAdjustments {
    #[cfg(feature = "filesystem")]
    filesystem:  filesystem::FilesystemAdjustment,
    #[cfg(feature = "network")]
    network:     network::NetworkAdjustment,
    #[cfg(feature = "privilege")]
    privilege:   privilege::PrivilegeAdjustment,
    #[cfg(feature = "system_call")]
    system_call: system_call::SystemCallAdjustment,
}

#[derive(Debug)]
pub struct LockdownPlugin;

impl Plugin for LockdownPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<LockdownAdjustments>();
        app.init_resource::<LockdownAdjustments>();

        #[cfg(feature = "feature_warning")]
        if std::mem::size_of::<LockdownAdjustments>() == 0 {
            warn!("No features activated for bevy_mod_lockdown.");
            warn!("This plugin does offer most functionality behind feature flags.");
            warn!("See README for more information and a list of available features.");
            warn!(
                "Enable a feature other than 'feature_warning' to remove this warning (or disable the default features)."
            );
        }

        #[cfg(feature = "filesystem")]
        app.add_plugins(filesystem::FilesystemPlugin);

        #[cfg(feature = "network")]
        app.add_plugins(network::NetworkPlugin);

        #[cfg(feature = "privilege")]
        app.add_plugins(privilege::PrivilegePlugin);

        #[cfg(feature = "system_call")]
        app.add_plugins(system_call::SystemCallPlugin);
    }
}
