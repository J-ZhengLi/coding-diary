use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{layers::Layer, GameState};

use super::background::BG_SCALE;

const FERRIS_SIZE_X: f32 = 34.0;
const FERRIS_SIZE_Y: f32 = 21.0;

// min/max initial speed when jumping
const MIN_JUMP_SPEED: f32 = 50.;
const MAX_JUMP_SPEED: f32 = 200.;

const JUMP_CHARGE_SCALE: f32 = 180.;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MoveSpeed(f32);

#[derive(Component)]
struct JumpSpeed(f32);

#[derive(Component, Clone, Copy)]
struct Grounded(bool);

impl Grounded {
    fn set(&mut self, is_grounded: bool) {
        self.0 = is_grounded;
    }
}

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
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(anim_player)
                .with_system(player_movement)
                .with_system(ground_detection),
        );
    }
}

fn init_player_animations(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("initializing ferris...");

    let ferris_idle: Handle<Image> = assets.load("J-ZhengLi/ferris-idle.png");
    let t_atlas =
        TextureAtlas::from_grid(ferris_idle, Vec2::new(FERRIS_SIZE_X, FERRIS_SIZE_Y), 6, 1);
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
    .insert(Player)
    .insert(MoveSpeed(0.5))
    .insert(Collider::cuboid(FERRIS_SIZE_X / 2., FERRIS_SIZE_Y / 2.))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(RigidBody::Dynamic)
    .insert(GravityScale(2.0))
    .insert(Velocity::zero())
    .insert(JumpSpeed(MIN_JUMP_SPEED))
    .insert(Grounded(false))
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

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<
        (
            &mut Transform,
            &MoveSpeed,
            &mut Velocity,
            &mut JumpSpeed,
            &Grounded,
        ),
        With<Player>,
    >,
) {
    let (mut tr, speed, mut velocity, mut j_speed, grounded) = player
        .get_single_mut()
        .expect("no player was added to the scene");
    // freeze rotation
    tr.rotation = Quat::IDENTITY;

    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        tr.translation.x -= speed.0;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        tr.translation.x += speed.0;
    }
    if grounded.0 && keys.pressed(KeyCode::Space) {
        if j_speed.0 < MAX_JUMP_SPEED {
            j_speed.0 += JUMP_CHARGE_SCALE * time.delta_seconds();
        }
    }
    if grounded.0 && keys.just_released(KeyCode::Space) {
        info!("jumping with initial speed: {}", j_speed.0);
        velocity.linvel = Vec2::new(0., j_speed.0);
        j_speed.0 = MIN_JUMP_SPEED;
    }
}

fn ground_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut player: Query<(&mut Grounded, Entity), With<Player>>,
) {
    let (mut grounded, entity) = player
        .get_single_mut()
        .expect("no player was added to the scene");

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent_a, ent_b, _) = collision_event {
            if ent_a.id() == entity.id() || ent_b.id() == entity.id() {
                grounded.set(true);
            }
        } else {
            grounded.set(false);
        }
    }
}
