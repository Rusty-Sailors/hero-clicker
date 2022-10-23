use bevy::prelude::*;
use iyes_loopless::prelude::*;
use clicker_core::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(
                AppState::MainMenu,
                render_menu
            )
            .add_exit_system(
                AppState::MainMenu,
                clicker_core::despawn_with::<OnMenuScreen>
            )
            .add_system(
                button_system
                    .run_in_state(AppState::MainMenu)
            );
    }
}

#[derive(Component)]
struct OnMenuScreen;

fn render_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Augusta.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::WHITE.into(),
    };

    commands.spawn_bundle(NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::CRIMSON.into(),
        ..default()
    })
    .insert(OnMenuScreen)
    .with_children(|parent| {
        // Display the game name
        parent.spawn_bundle(
            TextBundle::from_section(
                "Hero Clucker",
                TextStyle {
                    font: font.clone(),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
            }),
        );
        parent
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "New Game",
                button_text_style.clone(),
            ));
        });
    });
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>),
    >
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(AppState::InGame));
            }
            _ => ()
        }
    }
}