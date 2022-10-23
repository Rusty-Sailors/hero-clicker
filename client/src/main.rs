use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use iyes_loopless::prelude::*;

use clicker_core::AppState;


mod hero;
mod gold;
mod camera;
mod render;
mod menu;
mod multiplayer;

pub use camera::*;
pub use render::*;
pub use menu::*;
pub use multiplayer::*;

fn main() {
    App::new()
        .add_loopless_state(AppState::MainMenu)
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: "Hero Clicker".to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugin(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugins(clicker_core::ClickerCorePlugins)
        .add_plugins(RenderPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(MultiplayerPlugin)
        .run();
}