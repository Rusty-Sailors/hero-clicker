use bevy::prelude::*;

pub struct ClickEvent;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickEvent>();
    }
}
