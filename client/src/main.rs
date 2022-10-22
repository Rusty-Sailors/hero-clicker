use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
    .add_startup_system(spawn_camera)
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::default())
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}