use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
};

pub mod hero;

pub use hero::*;

pub struct ClickerCorePlugins;

impl PluginGroup for ClickerCorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(HeroPlugin);
    }
}