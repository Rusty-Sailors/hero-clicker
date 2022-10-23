use bevy::prelude::*;

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .insert_resource(Gold { amount: 0 })
            .add_system(hero_clicked_system);
    }
}

pub struct ClickEvent;

pub struct Gold {
    pub amount: u64
}

fn hero_clicked_system(mut click_events: EventReader<ClickEvent>, mut gold: ResMut<Gold>) {
    for _ in click_events.iter() {
        gold.amount += calculate_gold_per_click();
        info!(gold.amount);
    }
}

fn calculate_gold_per_click() -> u64 {
    1
}