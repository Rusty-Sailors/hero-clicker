use bevy::prelude::*;
use iyes_loopless::prelude::*;
use clicker_core::AppState;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct HeroButton;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GoldText;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct LeftBar;


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GoldText>()
            .register_type::<LeftBar>()
            .register_type::<HeroButton>()
            .add_enter_system(AppState::InGame, render_left_bar);
    }
}

fn render_left_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                margin: UiRect {
                    right: Val::Auto,
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Left Bar"))
        .insert(LeftBar)
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
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
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        top: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                }),
            );
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        bottom: Val::Px(50.0),
                        ..default()
                    },
                    ..default()
                },
                    color: Color::NONE.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(175.0), Val::Px(175.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::rgb(0.9,0.9,0.9).into(),
                    image: UiImage(asset_server.load("hero.png")),
                    ..default()
                })
                .insert(HeroButton)
                .insert(Name::new("Hero Button"));
            });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            });
        });
}
