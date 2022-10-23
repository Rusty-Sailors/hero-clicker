use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::AppState;
use crate::gold;

pub struct AutoClickerPlugin;

impl Plugin for AutoClickerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<AutoClicker>()
            .add_enter_system(
                AppState::InGame,
                spawn_auto_clicker,
            )
            .add_system(
                auto_click_system
                    .run_in_state(AppState::InGame)
            );
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct AutoClicker {
    num_clickers: i64,
    base_rate_per_sec: f64,
    // fractional clicks leftover from last tick
    clicks_leftover: f64,
}

fn spawn_auto_clicker(mut commands: Commands) {
    commands.spawn()
        .insert(AutoClicker {
            num_clickers: 1,
            base_rate_per_sec: 1.0,
            clicks_leftover: 0.0,
        });
}

fn auto_click_system(time: Res<Time>, mut query: Query<&mut AutoClicker>, mut gold: ResMut<gold::Gold>) {
    query.for_each_mut(|mut clicker| {
        let clicks_to_perform: f64 = 
            (
                clicker.num_clickers as f64 *
                clicker.base_rate_per_sec *
                time.delta_seconds_f64()
            ) + clicker.clicks_leftover;

        gold.amount += clicks_to_perform as u64 * gold::calculate_gold_per_click();

        // we need to carry over fractional clicks to the next tick
        clicker.clicks_leftover = clicks_to_perform - clicks_to_perform.floor();
    });
}
