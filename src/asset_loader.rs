use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    // pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Image>,
    pub bullet: Handle<Image>,
    pub lives: Handle<Image>,
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
        bullet: asset_server.load("sprite/Square.png"),
        lives: asset_server.load("sprite/Lives.png"),
    }
}
