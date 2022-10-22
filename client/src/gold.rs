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

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
        .insert(Gold {amount: 0})
        .insert(Name::new("Gold"))
        .insert_bundle(
            TextBundle::from_section(
                "hello\nbevy!",
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
                    right: Val::Px(550.0),
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
