use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    EguiPlugin,
    egui,
};
use bevy_mod_lockdown::LockdownAdjustments;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true
    });
    app.add_plugins(bevy_mod_lockdown::LockdownPlugin);
    app.add_systems(Update, display_adjustment_status);
    app.run();
}

fn display_adjustment_status(
    mut contexts: EguiContexts,
    lockdown_adjustments: Res<'_, LockdownAdjustments>,
) {
    egui::Window::new("Example window").show(contexts.ctx_mut(), |ui| {
        bevy_mod_lockdown::egui::ui_for_adjustment(ui, &lockdown_adjustments, true);
    });
}
