use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{layers::Layer, GameState};

use super::background::BG_SCALE;

const FERRIS_SIZE_X: f32 = 34.0;
const FERRIS_SIZE_Y: f32 = 21.0;

pub struct PlayerPlugin;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct IdleAnim;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InitPlayer).with_system(init_player_animations),
        )
        .add_system_set(SystemSet::on_update(GameState::LoadingPlayer).with_system(init_player))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(anim_player));
    }
}

fn init_player_animations(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("initializing ferris...");

    let ferris_idle: Handle<Image> = assets.load("ferris-idle.png");
    let t_atlas = TextureAtlas::from_grid(ferris_idle, Vec2::new(FERRIS_SIZE_X, FERRIS_SIZE_Y), 6, 1);
    let atlas_handle = texture_atlases.add(t_atlas);
    cmd.insert_resource(atlas_handle);

    state
        .set(GameState::LoadingPlayer)
        .expect("failed to set game state");
}

fn init_player(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    idle_handle: Res<Handle<TextureAtlas>>,
) {
    cmd.spawn_bundle(SpriteSheetBundle {
        texture_atlas: idle_handle.clone(),
        transform: Transform::from_xyz(-80., -200., Layer::Characters.into()).with_scale(BG_SCALE),
        ..Default::default()
    })
    .insert(Collider::cuboid(FERRIS_SIZE_X / 2., FERRIS_SIZE_Y / 2.))
    .insert(RigidBody::Dynamic)
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    state
        .set(GameState::Running)
        .expect("failed to set game state");
}

// ripped straight from bevy examples!!!
fn anim_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
