use super::json_loader::JsonPlugin;
use crate::layers::Layer;
use crate::plugins::background::BG_SCALE;
use crate::{GameState, PlatformCfg, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

const TILE_SIZE_X: f32 = 16.0;
const TILE_SIZE_Y: f32 = 16.0;
// max number of tiles that can be placed horizontally;
const MAX_NUM_OF_TILES_H: isize = (DEFAULT_WIDTH / (TILE_SIZE_X * BG_SCALE.x)) as isize;

pub struct PlatformPlugin;

#[derive(Component)]
struct EditablePlatform;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(JsonPlugin::<PlatformCfg>::default())
            .add_startup_system(init_base_floor)
            .add_system_set(SystemSet::on_update(GameState::Started).with_system(init_platforms))
            .add_system_set(
                SystemSet::on_update(GameState::LoadingPlatforms).with_system(load_platforms),
            )
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(handle_mouse_input));
    }
}

fn init_base_floor(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("initializing base floor...");

    let sprite_handle: Handle<Image> = assets.load("FreeCuteTileset/Tileset.png");
    let t_atlas = TextureAtlas::from_grid(sprite_handle, Vec2::new(TILE_SIZE_X, TILE_SIZE_Y), 8, 6);
    let atlas_handle = texture_atlases.add(t_atlas);
    cmd.insert_resource(atlas_handle.clone());

    // need 4 rows of base floor tiles, where the top row are grass blocks
    // and the rest 2 rows are dirt blocks
    const ROWS: u8 = 4;
    for y in 0..ROWS {
        let tile_idx: usize = if y == ROWS - 1 { 1 } else { 9 };

        // add an extra tile at the end just in case there's a gap at the end
        for x in 0..=MAX_NUM_OF_TILES_H {
            cmd.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    anchor: Anchor::BottomLeft,
                    index: tile_idx,
                    ..Default::default()
                },
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_xyz(
                    -DEFAULT_WIDTH / 2.0 + x as f32 * TILE_SIZE_X * BG_SCALE.x,
                    -DEFAULT_HEIGHT / 2.0 + y as f32 * TILE_SIZE_Y * BG_SCALE.y,
                    Layer::Platforms.into(),
                )
                .with_scale(BG_SCALE),
                ..Default::default()
            });
        }
    }

    // add one collider box for ground
    cmd.spawn_bundle(TransformBundle {
        local: Transform::from_xyz(
            0.,
            -DEFAULT_HEIGHT / 2.0 + TILE_SIZE_Y * BG_SCALE.y * (ROWS - 1) as f32,
            Layer::Platforms.into(),
        ),
        ..Default::default()
    })
    .insert(Collider::cuboid(
        DEFAULT_WIDTH / 2.,
        TILE_SIZE_Y * BG_SCALE.y,
    ));
}

fn init_platforms(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    assets: Res<AssetServer>,
) {
    info!("initializing platform configuration...");

    let cfg_handle: Handle<PlatformCfg> = assets.load("platforms.json");
    cmd.insert_resource(cfg_handle);

    state
        .set(GameState::LoadingPlatforms)
        .expect("failed to set game state");
}

fn load_platforms(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    platform_cfg: Res<Handle<PlatformCfg>>,
    plfm_cfg_assets: Res<Assets<PlatformCfg>>,
    tileset: Res<Handle<TextureAtlas>>,
) {
    if let Some(plfm_cfg) = plfm_cfg_assets.get(&platform_cfg) {
        // loaded
        for plfm in &plfm_cfg.platforms {
            cmd.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..Default::default()
                },
                // TODO: spawn multiple block instead of stretching one
                transform: Transform::from_xyz(
                    plfm.pos_x * BG_SCALE.x,
                    plfm.pos_y * BG_SCALE.y,
                    Layer::Platforms.into(),
                )
                .with_scale(Vec3 {
                    x: BG_SCALE.x * plfm.length,
                    y: BG_SCALE.y,
                    z: BG_SCALE.z,
                }),
                texture_atlas: tileset.clone(),
                ..Default::default()
            })
            .insert(Collider::cuboid(TILE_SIZE_X / 2., TILE_SIZE_Y / 2.))
            .insert(EditablePlatform);
        }

        state
            .set(GameState::InitPlayer)
            .expect("failed to set game state");
    } else {
        // This might(?) get printed multiple times
        info!("loading platforms...");
    }
}

fn handle_mouse_input(
    mut _cmd: Commands,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_released(MouseButton::Left) {
        info!("editing platform");
    }
    else if buttons.just_released(MouseButton::Right) {
        info!("creating platform at position: {}", Vec2::new(0., 0.));
    }
}
