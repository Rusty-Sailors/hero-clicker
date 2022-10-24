use bevy::prelude::*;
use iyes_loopless::prelude::*;
use clicker_core::events::ClickEvent;
use clicker_core::AppState;
use crate::ui::HeroButton;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(
                CoreStage::PreUpdate,
                click_hero
                    .run_in_state(AppState::InGame)
            );
    }
}

fn click_hero(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<HeroButton>),
    >,
    mut writer: EventWriter<ClickEvent>
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                writer.send(ClickEvent);
            }
            _ => ()
        }
    }
}
