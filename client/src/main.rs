use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod hero;

pub use hero::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Gold {
    amount: u64
}

fn main() {
    App::new()
    .add_startup_system(startup)
    .add_startup_system(spawn_camera)
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::default())
    .add_plugin(HeroPlugin)
    .register_type::<Gold>()
    .add_system(mouse_click_system)
    .run();
}

fn startup(mut commands: Commands) {
    commands.spawn().insert(Gold {amount: 0});
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>, mut query: Query<&mut Gold>) {//mut gold: ResMut<Gold>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut gold = query.single_mut();
        gold.amount += 1;
        info!(gold.amount);
    }
}