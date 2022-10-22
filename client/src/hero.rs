use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Hero {
}

pub struct HeroPlugin;

struct HeroSheet(Handle<TextureAtlas>);

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_startup_system(spawn_hero)
            .register_type::<Hero>();
    }
}

fn load_assets(mut command: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let hero_image = asset_server.load("hero.png");
    let hero_idle_atlas = TextureAtlas::from_grid(hero_image, Vec2::new(512.0, 512.0), 1, 1);
    let atlas_handle = texture_atlases.add(hero_idle_atlas);
    command.insert_resource(HeroSheet(atlas_handle));
}

fn spawn_hero(mut commands: Commands, hero_assets: Res<HeroSheet>) {
    let sprite: TextureAtlasSprite = TextureAtlasSprite::new(0);
    // sprite.color = Color::rgb(0.3, 0.3, 0.9);
    // sprite.custom_size = Some(Vec2::splat(1.0));
    
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: hero_assets.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0,0.0,1.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Name::new("Hero"));
}
