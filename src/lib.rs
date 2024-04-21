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

#[derive(Debug)]
pub struct LockdownPlugin;

impl Plugin for LockdownPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
        #[cfg(feature = "feature_warning")]
        if env!("HAS_FEATURES") == "false" {
            warn!("No features activated for bevy_mod_lockdown.");
            warn!("This plugin does offer most functionality behind feature flags.");
            warn!("See README for more information and a list of available features.");
            warn!(
                "Enable a feature other than 'feature_warning' to remove this warning (or disable the default features)."
            );
        }

        #[cfg(feature = "filesystem")]
        _app.add_plugins(filesystem::FilesystemPlugin);

        #[cfg(feature = "network")]
        _app.add_plugins(network::NetworkPlugin);

        #[cfg(feature = "privilege")]
        _app.add_plugins(privilege::PrivilegePlugin);

        #[cfg(feature = "system_call")]
        _app.add_plugins(system_call::SystemCallPlugin);
    }
}
