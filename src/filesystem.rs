use std::path::{
    Path,
    PathBuf,
};

use bevy::prelude::*;

use crate::LockdownSet;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum FilesystemAdjustment {
    #[default]
    Unknown,
    Completed,
    CompletedPartial,
    Failed,
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct AllowedFilesystemAccess {
    pub(crate) read_only:  Vec<PathBuf>,
    pub(crate) write_only: Vec<PathBuf>,
    pub(crate) read_write: Vec<PathBuf>,
}
impl AllowedFilesystemAccess {
    pub fn new_with_required() -> Self {
        let mut read_only = Vec::new();
        let write_only = Vec::new();
        let mut read_write = Vec::new();

        // Required by bevy

        #[cfg(target_os = "linux")]
        {
            // TODO check which we really need

            // Add to read only access
            // TODO can we limit those more specific?
            read_only.push("/etc".into());
            read_only.push("/home".into());
            read_only.push("/proc".into());
            read_only.push("/run".into());
            read_only.push("/usr".into());

            // Add to read write access
            read_write.push("/dev".into());
        }

        // Asset paths
        // TODO Which path(s) do we need for assets? Same in debug and release?
        // TODO do we also need assets_processed?
        read_only.push("assets".into());
        read_write.push("imported_assets".into());

        Self {
            read_only,
            write_only,
            read_write,
        }
    }

    pub fn add_read_only(
        &mut self,
        path: &Path,
    ) {
        self.read_only.push(path.to_path_buf());
    }

    pub fn add_write_only(
        &mut self,
        path: &Path,
    ) {
        self.write_only.push(path.to_path_buf());
    }

    pub fn add_read_write(
        &mut self,
        path: &Path,
    ) {
        self.read_write.push(path.to_path_buf());
    }
}

#[derive(Debug)]
pub struct FilesystemPlugin;

impl Plugin for FilesystemPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<FilesystemAdjustment>();
        app.insert_resource(FilesystemAdjustment::Unknown);

        // Add allowed filesystem access only if it is not already added
        app.register_type::<AllowedFilesystemAccess>();
        if !app.world.contains_resource::<AllowedFilesystemAccess>() {
            let directory_access = AllowedFilesystemAccess::new_with_required();
            app.insert_resource(directory_access);
        }

        #[cfg(target_os = "linux")]
        {
            app.add_systems(Startup, try_restricting_file_access_linux.in_set(LockdownSet::Startup));
        }
    }
}

#[cfg(target_os = "linux")]
fn try_restricting_file_access_linux(
    allowed_filesystem_access: Res<'_, AllowedFilesystemAccess>,
    mut filesystem_adjustment: ResMut<'_, FilesystemAdjustment>,
) {
    use landlock::{
        path_beneath_rules,
        Access,
        AccessFs,
        Ruleset,
        RulesetAttr,
        RulesetCreatedAttr,
        RulesetStatus,
        ABI,
    };

    *filesystem_adjustment = FilesystemAdjustment::Failed;

    let abi = ABI::V3;
    let read_only = AccessFs::from_read(abi);
    let write_only = AccessFs::from_write(abi);
    let read_write = AccessFs::from_all(abi);

    let rule_set = Ruleset::default();
    let Ok(rule_set) = rule_set.handle_access(read_write) else {
        warn!("Unable to add base rule");
        return;
    };
    let Ok(rule_set) = rule_set.create() else {
        warn!("Unable to create ruleset");
        return;
    };

    // Add read only access
    let read_only_rules = path_beneath_rules(&allowed_filesystem_access.read_only, read_only);
    let Ok(rule_set) = rule_set.add_rules(read_only_rules) else {
        warn!("Unable to add ready only rules");
        return;
    };

    // Add write only access
    let write_only_rules = path_beneath_rules(&allowed_filesystem_access.write_only, write_only);
    let Ok(rule_set) = rule_set.add_rules(write_only_rules) else {
        warn!("Unable to add write only rules");
        return;
    };

    // Add read write access
    let read_write_rules = path_beneath_rules(&allowed_filesystem_access.read_write, read_write);
    let Ok(rule_set) = rule_set.add_rules(read_write_rules) else {
        warn!("Unable to add ready write rules");
        return;
    };

    // Try to enforce
    let result = rule_set.restrict_self();
    match result {
        Ok(status) => match status.ruleset {
            RulesetStatus::FullyEnforced => {
                info!("Sandboxing with landlock fully enforced");
                *filesystem_adjustment = FilesystemAdjustment::Completed;
            },
            RulesetStatus::PartiallyEnforced => {
                warn!("Sandboxing with landlock partially enforced");
                *filesystem_adjustment = FilesystemAdjustment::CompletedPartial;
            },
            RulesetStatus::NotEnforced => {
                warn!("Sandboxing with landlock not enforced. Kernel with landlock support needed");
            },
        },
        Err(e) => {
            warn!("Error while trying to restrict self with landlock: {e}");
        },
    }
}
