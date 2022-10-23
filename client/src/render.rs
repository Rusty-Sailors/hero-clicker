
use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
};

use crate::hero::*;
use crate::gold::*;

pub struct RenderPlugins;

impl PluginGroup for RenderPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(HeroPlugin)
            .add(GoldPlugin);
    }
}
