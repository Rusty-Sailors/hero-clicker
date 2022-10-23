use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
};

pub mod hero;
pub mod gold;

pub use hero::*;
pub use gold::*;

pub struct ClickerCorePlugins;

impl PluginGroup for ClickerCorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(GoldPlugin)
            .add(HeroPlugin);
    }
}