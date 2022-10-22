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
            .add_system(mouse_click_system)
            .add_system(update_gold_text_system);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Gold {
    amount: u64
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
        .insert(Gold {amount: 0})
        .insert(Name::new("Gold"))
        .insert_bundle(
            TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/Augusta.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(500.0),
                    right: Val::Px(605.0),
                    ..default()
                },
                ..default()
            }),
        );
}

fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>, mut query: Query<&mut Gold>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut gold = query.single_mut();
        gold.amount += 1;
        info!(gold.amount);
    }
}

fn update_gold_text_system(mut query: Query<(&mut Text, &Gold)>) {
    let data = query.single_mut();
    let gold = data.1;
    let mut text = data.0;
    text.sections[0].value = gold.amount.to_string();
}
