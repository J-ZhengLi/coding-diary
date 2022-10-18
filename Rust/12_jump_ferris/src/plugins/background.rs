use crate::layers::Layer;
use crate::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

const MAX_NUM_OF_CLOUDS: u8 = 8;
const BG_COLOR: Color = Color::rgb(0.4196, 0.7765, 1.0);
pub const BG_SCALE: Vec3 = Vec3::splat(DEFAULT_WIDTH / 320.0);

pub struct BackgroundPlugin;

#[derive(Component)]
struct StaticCloud;

#[derive(Component)]
struct Ground;

// TODO: remove this suppression
#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct AnimTimer(Timer);

#[derive(Component)]
pub struct AnimDirection(Direction);

#[derive(Component, Clone)]
pub struct Speed(f32);

#[derive(Component, Clone, Copy)]
pub struct RollingImageSize(Vec2);

#[derive(Component)]
struct Clouds;

#[derive(Bundle)]
pub struct RollingBundle {
    pub direction: AnimDirection,
    pub speed: Speed,
    pub size: RollingImageSize,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for RollingBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle::default(),
            direction: AnimDirection(Direction::Left),
            speed: Speed(1.0),
            size: RollingImageSize(Vec2::default()),
        }
    }
}

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            // lock the background cloud animation with 30 frame per second
            .insert_resource(AnimTimer(Timer::from_seconds(1. / 30., true)))
            .add_startup_system(init_background_image)
            .add_system(animate_bg_clouds)
            .add_system(random_floating_clouds);
    }
}

fn init_background_image(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("initializing game background...");

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_xyz(
            -DEFAULT_WIDTH / 2.,
            -DEFAULT_HEIGHT / 2.,
            Layer::BackgroundDeco.into(),
        )
        .with_scale(BG_SCALE),
        texture: assets.load("FreeCuteTileset/BG3.png"),
        ..Default::default()
    })
    .insert(Ground);

    let clouds_atlas = TextureAtlas::from_grid_with_padding(
        assets.load("cloud/Clouds V2.png"),
        Vec2::new(60.0, 38.),
        2,
        3,
        Vec2::new(4., 2.),
        Vec2::new(5., 8.),
    );
    let clouds_len = clouds_atlas.textures.len();
    let cloud_atlas_handle = texture_atlases.add(clouds_atlas);

    // This serves as an "object pool" for clouds which will be outside of current frame
    // and invisible on initialized
    let mut rng = thread_rng();
    for _ in 0..MAX_NUM_OF_CLOUDS {
        cmd.spawn_bundle(SpriteSheetBundle {
            texture_atlas: cloud_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                anchor: bevy::sprite::Anchor::BottomRight,
                index: rng.gen_range(0..clouds_len),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                rng.gen_range(-DEFAULT_WIDTH / 2.0..DEFAULT_WIDTH / 2.0),
                rng.gen_range(-DEFAULT_WIDTH / 2.0..DEFAULT_WIDTH / 2.0),
                Layer::BackgroundDeco.into(),
            )
            .with_scale(BG_SCALE),
            ..Default::default()
        })
        .insert(Clouds)
        .insert(Speed(rng.gen_range(0.2..0.6)))
        .insert(AnimDirection(Direction::Left));
    }
}

fn animate_bg_clouds(
    mut cmd: Commands,
    time: Res<Time>,
    mut timer: ResMut<AnimTimer>,
    mut clouds: Query<(&AnimDirection, &Speed, &RollingImageSize, &mut Transform)>,
    assets: Res<AssetServer>,
) {
    if clouds.is_empty() {
        let bg_cloud_size = RollingImageSize(Vec2 { x: 320.0, y: 192.0 });
        let sprite = Sprite {
            anchor: bevy::sprite::Anchor::BottomLeft,
            custom_size: Some(bg_cloud_size.0),
            ..Default::default()
        };

        cmd.spawn_bundle(RollingBundle {
            sprite_bundle: SpriteBundle {
                sprite: sprite.clone(),
                transform: Transform::from_xyz(
                    -DEFAULT_WIDTH / 2.,
                    -DEFAULT_HEIGHT / 2.1,
                    Layer::Background.into(),
                )
                .with_scale(BG_SCALE),
                texture: assets.load("FreeCuteTileset/BG2.png"),
                ..Default::default()
            },
            speed: Speed(0.2),
            size: bg_cloud_size,
            direction: AnimDirection(Direction::Left),
        });

        cmd.spawn_bundle(RollingBundle {
            sprite_bundle: SpriteBundle {
                sprite,
                transform: Transform::from_xyz(
                    DEFAULT_WIDTH / 2.,
                    -DEFAULT_HEIGHT / 2.1,
                    Layer::Background.into(),
                )
                .with_scale(BG_SCALE),
                texture: assets.load("FreeCuteTileset/BG2.png"),
                ..Default::default()
            },
            speed: Speed(0.2),
            size: bg_cloud_size,
            direction: AnimDirection(Direction::Left),
        });
    }

    if timer.0.tick(time.delta()).just_finished() {
        for (rd, rs, ris, mut tr) in &mut clouds {
            match rd.0 {
                Direction::Left => {
                    tr.translation.x -= rs.0;
                    if tr.translation.x < -DEFAULT_WIDTH / 2.0 - ris.0.x * tr.scale.x {
                        tr.translation.x += DEFAULT_WIDTH * 2.0;
                    }
                }
                _ => unimplemented!("scrolling in other directions are not implemented yet"),
            };
        }
    }
}

fn random_floating_clouds(
    mut timer: ResMut<AnimTimer>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut clouds: Query<
        (
            &mut Transform,
            &mut Visibility,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &Speed,
            &AnimDirection,
        ),
        With<Clouds>,
    >,
    camera: Query<&Transform, (With<Camera2d>, Without<Clouds>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut random = thread_rng();
        let cam_tf = camera.get_single().expect("unable to get a single camera");

        for (mut tr, mut vs, mut tas, h, s, _d) in &mut clouds {
            if vs.is_visible {
                tr.translation.x -= s.0;
            } else {
                // activates some clouds at random offsets
                tr.translation.x = random.gen_range(DEFAULT_WIDTH / 2.0..DEFAULT_WIDTH);
                tr.translation.y = random.gen_range(-DEFAULT_HEIGHT / 2.0..DEFAULT_HEIGHT / 2.0)
                    + cam_tf.translation.y;
                // randomly pick sprites for clouds
                let t_altas = texture_atlases
                    .get(h)
                    .expect("unable to get clouds sprite from spritesheet");
                tas.index = random.gen_range(0..t_altas.textures.len());
                vs.is_visible = true;
            }

            if tr.translation.x < -DEFAULT_WIDTH / 2. {
                vs.is_visible = false;
            }
        }
    }
}
