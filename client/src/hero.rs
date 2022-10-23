use bevy::prelude::*;

use clicker_core::hero::*;

struct HeroSheet(Handle<TextureAtlas>);

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_system(spawn_hero);
    }
}

fn load_assets(mut command: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let hero_image = asset_server.load("hero.png");
    let hero_idle_atlas = TextureAtlas::from_grid(hero_image, Vec2::new(512.0, 512.0), 1, 1);
    let atlas_handle = texture_atlases.add(hero_idle_atlas);
    command.insert_resource(HeroSheet(atlas_handle));
}

fn spawn_hero(mut commands: Commands, mut hero_spawn_events: EventReader<HeroSpawnedEvent>, hero_assets: Res<HeroSheet>) {
    for hero_spawn in hero_spawn_events.iter() {
        let sprite: TextureAtlasSprite = TextureAtlasSprite::new(0);
    
        commands
            .entity(hero_spawn.0)
            .insert_bundle(SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: hero_assets.0.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0,0.0,1.0),
                    ..Default::default()
                },
                ..Default::default()
            });
    }
}
