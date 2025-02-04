use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::ui::system::ui_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, ui_system);
    }
}
