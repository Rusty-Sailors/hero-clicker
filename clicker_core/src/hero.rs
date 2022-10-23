use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Hero;

pub struct HeroSpawnedEvent(pub Entity);

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HeroSpawnedEvent>()
            .add_startup_system(spawn_hero)
            .register_type::<Hero>();
    }
}

fn spawn_hero(mut commands: Commands) {    
    commands.spawn()
        .insert(Hero)
        .insert(Name::new("Hero"));
}
