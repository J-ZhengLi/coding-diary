use super::json_loader::JsonPlugin;
use crate::layers::Layer;
use crate::plugins::background::BG_SCALE;
use crate::{Cursors, GameFont, GameState, PlatformCfg, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

const TILE_SIZE_X: f32 = 16.0;
const TILE_SIZE_Y: f32 = 16.0;
// max number of tiles that can be placed horizontally;
const MAX_NUM_OF_TILES_H: isize = (DEFAULT_WIDTH / (TILE_SIZE_X * BG_SCALE.x)) as isize;

pub struct PlatformPlugin;

#[derive(Default)]
struct AllSprites {
    handle: Handle<TextureAtlas>,
}

#[derive(Component)]
struct EditablePlatform;

#[derive(Default)]
pub struct SpawnPoints {
    pub ground: Vec2,
    pub top: Vec2,
}

#[derive(Default)]
struct RunningStatus {
    on_editing: bool,
}

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(JsonPlugin::<PlatformCfg>::default())
            .init_resource::<AllSprites>()
            .init_resource::<RunningStatus>()
            .init_resource::<SpawnPoints>()
            .add_startup_system(init_base_floor)
            .add_system_set(SystemSet::on_update(GameState::Started).with_system(init_platforms))
            .add_system_set(
                SystemSet::on_update(GameState::LoadingPlatforms).with_system(load_platforms),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Running).with_system(handle_mouse_input),
            )
            .add_system(handle_keyboard_input);
    }
}

fn init_base_floor(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sprites: ResMut<AllSprites>,
    mut status: ResMut<RunningStatus>,
    mut spawn_points: ResMut<SpawnPoints>,
) {
    info!("initializing base floor...");

    // initialize some settings
    status.on_editing = false;

    let sprite_handle: Handle<Image> = assets.load("FreeCuteTileset/Tileset.png");
    let t_atlas = TextureAtlas::from_grid(sprite_handle, Vec2::new(TILE_SIZE_X, TILE_SIZE_Y), 8, 6);
    let atlas_handle = texture_atlases.add(t_atlas);
    sprites.handle = atlas_handle.clone();

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

    spawn_points.ground = Vec2::new(
        0.,
        -DEFAULT_HEIGHT / 2.0 + ROWS as f32 * TILE_SIZE_Y * BG_SCALE.y,
    );

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
    tileset: Res<AllSprites>,
    game_font: Res<GameFont>,
    spawn_points: Res<SpawnPoints>,
) {
    if let Some(plfm_cfg) = plfm_cfg_assets.get(&platform_cfg) {
        // loaded
        for plfm in &plfm_cfg.platforms {
            spawn_platform(
                &mut cmd,
                tileset.handle.clone(),
                game_font.main.clone(),
                Vec2::new(plfm.pos_x, spawn_points.ground.y + plfm.pos_y),
                plfm.length,
                &plfm.text,
            );
        }

        state
            .set(GameState::InitPlayer)
            .expect("failed to set game state");
    } else {
        // This might(?) get printed multiple times
        info!("loading platforms...");
    }
}

fn handle_keyboard_input(
    mut status: ResMut<RunningStatus>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if keys.pressed(KeyCode::Return) {
        if status.on_editing {
            state
                .set(GameState::Running)
                .expect("unable to set game state to running");
            status.on_editing = false;
        } else {
            state
                .set(GameState::Editing)
                .expect("unable to set game state to editing");
            status.on_editing = true;
        }
    }
}

fn handle_mouse_input(
    mut cmd: Commands,
    tileset: Res<AllSprites>,
    game_font: Res<GameFont>,
    buttons: Res<Input<MouseButton>>,
    mut cursors: Query<(&Style, &mut UiImage, &mut Cursors)>,
    camera: Query<&Transform, With<Camera2d>>,
    platforms: Query<&Transform, (With<EditablePlatform>, Without<Camera2d>)>
) {
    if buttons.just_released(MouseButton::Left) {
        let cam_tf = camera.get_single().expect("failed to get a single camera");
        if let Some(c_pos) = cursors.get_single_mut().ok().and_then(|(style, _, _)| {
            ui_to_world_pos(style.position, cam_tf.translation.truncate())
        }) {
            // check if cursor "touches" any existing platforms
            println!("mouse pos: {:?}", c_pos);
            for plfm in &platforms {

            }
        }

        info!("editing platform");
    } else if buttons.just_released(MouseButton::Right) {
        let cam_tf = camera.get_single().expect("failed to get a single camera");
        if let Ok((cursor, _, _)) = cursors.get_single_mut() {
            if let Some(pos) = ui_to_world_pos(cursor.position, cam_tf.translation.truncate()) {
                info!("creating a new platform at position: {:?}", pos);
                spawn_platform(
                    &mut cmd,
                    tileset.handle.clone(),
                    game_font.main.clone(),
                    pos,
                    3.0,
                    "Text",
                );
            }
        }
    }
}

fn spawn_platform(
    cmd: &mut Commands,
    atlas: Handle<TextureAtlas>,
    font: Handle<Font>,
    pos: Vec2,
    length: f32,
    label: &str,
) {
    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 1,
            ..Default::default()
        },
        transform: Transform::from_xyz(pos.x, pos.y, Layer::Platforms.into())
            // TODO: spawn multiple block instead of stretching one
            .with_scale(Vec3 {
                x: BG_SCALE.x * length,
                y: BG_SCALE.y,
                z: 1.,
            }),
        texture_atlas: atlas,
        ..Default::default()
    })
    .with_children(|p| {
        p.spawn_bundle(Text2dBundle {
            text: Text::from_section(
                label,
                TextStyle {
                    font,
                    font_size: 9.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform::from_xyz(0., 0., Layer::Platforms.into()).with_scale(Vec3::new(
                1.0 / length,
                1.0,
                1.0,
            )),
            ..Default::default()
        });
    })
    .insert(Collider::cuboid(TILE_SIZE_X / 2., TILE_SIZE_Y / 2.))
    .insert(EditablePlatform);
}

fn f32_val_to_f32(val: Val) -> Option<f32> {
    if let Val::Px(v) = val {
        Some(v)
    } else {
        None
    }
}

// ui position starts from bottom left, but the transform position starts from
// screen center, which is mildly inconvenient!!!
fn ui_to_world_pos(ui_pos: UiRect<Val>, camera_pos: Vec2) -> Option<Vec2> {
    let x_pos = f32_val_to_f32(ui_pos.left).map(|f| f - DEFAULT_WIDTH / 2. + camera_pos.x)?;
    let y_pos = f32_val_to_f32(ui_pos.bottom).map(|f| f - DEFAULT_HEIGHT / 2. + camera_pos.y)?;

    Some(Vec2::new(x_pos, y_pos))
}

fn check_overlaps(a: &Transform, b: &Transform) {
    
}
