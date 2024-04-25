use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{
    egui,
    EguiContext,
    EguiPlugin,
};
use bevy_mod_lockdown::LockdownAdjustments;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EguiPlugin);
    app.add_plugins(bevy_mod_lockdown::LockdownPlugin);
    app.add_systems(Update, display_adjustment_status);
    app.run();
}

fn display_adjustment_status(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    lockdown_adjustments: Res<'_, LockdownAdjustments>,
) {
    egui::Window::new("Example window").show(egui_ctx.single_mut().get_mut(), |ui| {
        bevy_mod_lockdown::egui::ui_for_adjustment(ui, &lockdown_adjustments, true);
    });
}
