use bevy::prelude::*;
use iyes_loopless::prelude::*;
use clicker_core::hero::*;
use clicker_core::events::ClickEvent;
use clicker_core::AppState;

struct HeroSheet(Handle<TextureAtlas>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct HeroButton;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                spawn_hero
                    .run_in_state(AppState::InGame)
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                click_hero
                    .run_in_state(AppState::InGame)
            );
    }
}

fn spawn_hero(mut commands: Commands, heros: Query<Entity, Added<Hero>>, asset_server: Res<AssetServer>) {
    for hero in heros.iter() {
        commands
            .entity(hero)
            //GoldText should go here
            .insert_bundle(NodeBundle {
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
            }).with_children(|parent| {
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                });
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
                            size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: Color::rgb(0.9,0.9,0.9).into(),
                        image: UiImage(asset_server.load("clicker_icon.png")),
                        ..default()
                    }).insert(HeroButton);
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
