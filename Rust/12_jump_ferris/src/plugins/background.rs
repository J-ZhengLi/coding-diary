use bevy::prelude::*;
use crate::{DEFAULT_HEIGHT, DEFAULT_WIDTH};

const BG_COLOR: Color = Color::rgb(0.4196, 0.7765, 1.0);

pub struct BackgroundPlugin;

#[derive(Component)]
struct StaticCloud;

#[derive(Component)]
struct Ground;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(BG_COLOR))
            .add_startup_system(init_background_image);
    }
}

fn init_background_image(mut cmd: Commands, assets: Res<AssetServer>) {
    info!("initializing game background...");

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., -DEFAULT_HEIGHT / 2.1, 1.).with_scale(bg_scale()),
        texture: assets.load("FreeCuteTileset/BG2.png"),
        ..Default::default()
    }).insert(StaticCloud);

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., -DEFAULT_HEIGHT / 2., 2.).with_scale(bg_scale()),
        texture: assets.load("FreeCuteTileset/BG3.png"),
        ..Default::default()
    }).insert(Ground);

}

fn animate_clouds(mut cmd: Commands, time: Res<Time>, mut cloud: Query<&mut Transform, With<StaticCloud>>) {

}

fn bg_scale() -> Vec3 {
    // 320.0 is the width of that background image,
    // although it could be fetched using events but this is certainly faster.
    Vec3::splat(DEFAULT_WIDTH / 320.0)
}
