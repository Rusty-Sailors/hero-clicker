use bevy::prelude::*;
use iyes_loopless::prelude::*;
use clicker_core::gold::*;
use clicker_core::AppState;
use crate::ui::GoldText;

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_gold_text_system
                    .run_in_state(AppState::InGame)
            );
    }
}


fn update_gold_text_system(gold: Res<Gold>, mut query: Query<&mut Text, With<GoldText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = gold.amount.to_string();
}
