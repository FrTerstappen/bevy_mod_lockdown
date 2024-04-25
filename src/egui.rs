use crate::LockdownAdjustments;

pub fn ui_for_adjustment(
    ui: &mut egui::Ui,
    _lockdown_adjustments: &LockdownAdjustments,
    open_by_default: bool,
) {
    egui::CollapsingHeader::new("Lockdown adjustments").default_open(open_by_default).show(ui, |ui| {
        #[cfg(feature = "feature_warning")]
        if std::mem::size_of::<LockdownAdjustments>() == 0 {
            ui.label("No features activated for bevy_mod_lockdown.");
            ui.label("This plugin does offer most functionality behind feature flags.");
            ui.label("See README for more information and a list of available features.");
            ui.label(
                "Enable a feature other than 'feature_warning' to remove this warning (or disable the default features)."
            );
        }

        ui.columns(2, |_columns| {
            #[cfg(feature = "filesystem")]
            {
                _columns[0].label("Filesystem");
                _columns[1].label(format!("{:?}", _lockdown_adjustments.filesystem));
            }

            #[cfg(feature = "network")]
            {
                _columns[0].label("Network");
                _columns[1].label(format!("{:?}", _lockdown_adjustments.network));
            }

            #[cfg(feature = "privilege")]
            {
                _columns[0].label("Privilege");
                _columns[1].label(format!("{:?}", _lockdown_adjustments.privilege));
            }

            #[cfg(feature = "system_call")]
            {
                _columns[0].label("System call");
                _columns[1].label(format!("{:?}", _lockdown_adjustments.system_call));
            }
        });
    });
}
