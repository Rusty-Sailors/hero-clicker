use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Hero {
}

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Gold>()
            .add_startup_system(startup)
            .add_system(mouse_click_system);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Gold {
    amount: u64
}

fn startup(mut commands: Commands) {
    commands.spawn()
        .insert(Gold {amount: 0})
        .insert(Name::new("Gold"));
}

fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>, mut query: Query<&mut Gold>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut gold = query.single_mut();
        gold.amount += 1;
        info!(gold.amount);
    }
}

