use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    // pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Image>,
    pub bullet: Handle<Image>,
    pub lives: Handle<Image>,
    pub asteroids: [Handle<Image>; 4],
    pub font: Handle<Font>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        // asteroid: asset_server.load("sprites/Player.png"),
        spaceship: asset_server.load("sprites/Player.png"),
        bullet: asset_server.load("sprites/Square.png"),
        lives: asset_server.load("sprites/Lives.png"),
        asteroids: [
            asset_server.load("sprites/Asteroid_01.png"), 
            asset_server.load("sprites/Asteroid_02.png"), 
            asset_server.load("sprites/Asteroid_03.png"), 
            asset_server.load("sprites/Asteroid_04.png"), 
        ],
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    }
}
