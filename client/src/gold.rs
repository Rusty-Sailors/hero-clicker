use bevy::prelude::*;
use clicker_core::gold::*;

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GoldText>()
            .add_startup_system(spawn_)
            .add_system(mouse_click_system)
            .add_system(update_gold_text_system);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct GoldText;

fn spawn_(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
        .insert(GoldText)
        .insert(Name::new("Gold Text"))
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

fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>, mut writer: EventWriter<ClickEvent>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        writer.send(ClickEvent);
    }
}

fn update_gold_text_system(gold: Res<Gold>, mut query: Query<&mut Text, With<GoldText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = gold.amount.to_string();
}
