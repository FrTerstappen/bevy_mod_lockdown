use bevy::prelude::*;

use crate::LockdownSet;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum PrivilegeAdjustment {
    #[default]
    Unknown,
    Completed,
    Failed,
    NotNeeded,
}

#[derive(Debug)]
pub struct PrivilegePlugin;

impl Plugin for PrivilegePlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<PrivilegeAdjustment>();
        app.insert_resource(PrivilegeAdjustment::Unknown);

        #[cfg(target_os = "windows")]
        {
            app.add_systems(PostStartup, try_adjusting_privilege_windows.in_set(LockdownSet::PostStartup));
        }

        // TODO: we use nix for this, this means theoretical this runs on Android, iOS, Linux and MacOs
        #[cfg(target_os = "linux")]
        {
            app.add_systems(PostStartup, try_dropping_root_nix.chain().in_set(LockdownSet::PostStartup));
        }
    }
}

#[cfg(target_os = "linux")]
fn try_dropping_root_nix(mut privilege_adjustment: ResMut<'_, PrivilegeAdjustment>) {
    *privilege_adjustment = PrivilegeAdjustment::Failed;

    // Check if running as root
    match nix::unistd::getresuid() {
        Ok(uid) => {
            if uid.real.is_root() || uid.effective.is_root() || uid.saved.is_root() {
                warn!("User is root. Trying to drop root");
            } else {
                info!("Did not run as root. No adjustment needed");
                *privilege_adjustment = PrivilegeAdjustment::NotNeeded;
                return;
            }
        },
        Err(e) => {
            warn!("Unable to get uid/gid: {e}. Still trying to adjust uid and gid");
        },
    }

    // Get user id (from caller of sudo)
    let Ok(user_id) = std::env::var("SUDO_UID") else {
        warn!("Unable to get user id of calling user. Unable to adjust");
        return;
    };
    let Ok(user_id) = user_id.parse() else {
        warn!("Unable to parse user id. Unable to adjust");
        return;
    };
    let user_id = nix::unistd::Uid::from_raw(user_id);

    // Get group id (from caller of sudo)
    let Ok(group_id) = std::env::var("SUDO_GID") else {
        warn!("Unable to get group id of calling user. Unable to adjust");
        return;
    };
    let Ok(group_id) = group_id.parse() else {
        warn!("Unable to parse group id. Unable to adjust");
        return;
    };
    let group_id = nix::unistd::Gid::from_raw(group_id);

    // Change group and user
    // IMPORTANT: gid needs to be changed before uid
    // We do not have the right to change the gid after adjusting the uid
    match nix::unistd::setresgid(group_id, group_id, group_id) {
        Ok(()) => debug!("Adjusted gid"),
        Err(e) => warn!("Unable to set gid: {e}"),
    }
    match nix::unistd::setresuid(user_id, user_id, user_id) {
        Ok(()) => debug!("Adjusted uid"),
        Err(e) => warn!("Unable to set uid: {e}"),
    }

    // Check changes
    match nix::unistd::getresuid() {
        Ok(uid) => {
            if uid.real.is_root() || uid.effective.is_root() || uid.saved.is_root() {
                warn!("User is root after changing");
            } else {
                info!("Adjustment successful. No longer running as root");
                *privilege_adjustment = PrivilegeAdjustment::Completed;
            }
        },
        Err(e) => {
            warn!("Unable to get uid: {e}");
            *privilege_adjustment = PrivilegeAdjustment::Unknown;
        },
    }
}

#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
fn try_adjusting_privilege_windows(mut privilege_adjustment: ResMut<'_, PrivilegeAdjustment>) {
    use windows::Win32::{
        Foundation,
        Foundation::{
            CloseHandle,
            HANDLE,
        },
        Security::{
            AdjustTokenPrivileges,
            TOKEN_ASSIGN_PRIMARY,
            TOKEN_DUPLICATE,
            TOKEN_PRIVILEGES,
        },
        System::Threading::{
            GetCurrentProcess,
            OpenProcessToken,
        },
    };

    *privilege_adjustment = PrivilegeAdjustment::Failed;

    // Get a handle to the current process's primary token
    let process = unsafe { GetCurrentProcess() };
    let mut token = HANDLE::default();
    let desired_access = TOKEN_DUPLICATE | TOKEN_ASSIGN_PRIMARY;
    let result = unsafe { OpenProcessToken(process, desired_access, &mut token) };
    if let Err(e) = result {
        warn!("Unable to open process token: {e}");
        return;
    }

    // Switch to restricted token
    let disable_all_privilege = Foundation::TRUE;
    let new_state: ::core::option::Option<*const TOKEN_PRIVILEGES> = None;
    let buffer_length: u32 = 0;
    let previous_state: ::core::option::Option<*mut TOKEN_PRIVILEGES> = None;
    let return_length: ::core::option::Option<*mut u32> = None;
    let result = unsafe {
        AdjustTokenPrivileges(token, disable_all_privilege, new_state, buffer_length, previous_state, return_length)
    };
    if let Err(e) = result {
        warn!("Unable to adjust token privilege: {e}");
    } else {
        *privilege_adjustment = PrivilegeAdjustment::Completed;
    }

    // Cleanup
    if let Err(e) = unsafe { CloseHandle(token) } {
        warn!("Unable to close handle for token: {e}");
    }

    if let Err(e) = unsafe { CloseHandle(process) } {
        warn!("Unable to close handle for process: {e}");
    }
}
