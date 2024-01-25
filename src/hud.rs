use bevy::{prelude::*};

use crate::{state::AppState, asset_loader::SceneAssets, schedules::InGameSet};

pub const IMAGE_MARGIN: f32 = 8.0;
pub const IMAGE_SIZE: f32 = 32.0;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn_hud.run_if(in_state(AppState::InGame)).after(InGameSet::CollisionDetection))
            /* .add_systems(Update, (
                update_hud,
            ).run_if(in_state(AppState::InGame))) */
            .add_systems(OnExit(AppState::InGame), cleanup_hud);
    }
}

#[derive(Component)]
pub struct Lives;

#[derive(Resource)]
pub struct ScoreData {
    txt_score: Entity,
}

#[derive(Component)]
pub struct Score;

fn spawn_hud(
    mut commands: Commands, 
    scene_assets: Res<SceneAssets>,
) {
    let font_res = scene_assets.font.clone();
    let life = UiImage::new(scene_assets.lives.clone());

    commands.spawn((NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            margin: UiRect::new(Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN)),
            
            ..default()
        },
        background_color: BackgroundColor(Color::rgba_u8(0, 0, 0, 155)),
        ..default()
        }, Lives)).with_children(
            |parent| {
                for _ in 0..3 {
                    parent.spawn(ImageBundle {
                        style: Style {
                            height: Val::Px(64.0),
                            width: Val::Px(64.0),
                            margin: UiRect::new(Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN)),
                            ..default()
                        },
                        image: life.clone(),
                        ..default()
                    });
                } 
            }
    );
    let txt_score = commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: font_res,
                font_size: 18.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }), Score),
    ).id();
    commands.insert_resource(ScoreData { txt_score });
}

fn cleanup_hud(mut commands: Commands, ) {

}


