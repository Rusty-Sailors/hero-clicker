use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
};

pub mod hero;
pub mod gold;
pub mod network;
pub mod autoclicker;

pub use hero::*;
pub use gold::*;
pub use network::*;
pub use autoclicker::*;

pub struct ClickerCorePlugins;

impl PluginGroup for ClickerCorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(GoldPlugin)
            .add(HeroPlugin)
            .add(AutoClickerPlugin);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
}

pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}