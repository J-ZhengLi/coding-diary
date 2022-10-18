pub mod layers;
mod license;
mod platform_config;
mod plugins;
mod states;

pub use platform_config::PlatformCfg;
pub use states::GameState;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::close_on_esc;
use plugins::*;

pub const DEFAULT_WIDTH: f32 = 607.5;
pub const DEFAULT_HEIGHT: f32 = 1080.0;

const CURSOR_SIZE: Vec2 = Vec2::new(16.0, 16.0);

#[derive(Default)]
pub(crate) struct CursorSprites {
    pub normal: Handle<Image>,
    pub pointing: Handle<Image>,
}

#[derive(Default)]
pub(crate) struct GameFont {
    pub main: Handle<Font>,
}

#[derive(Component)]
pub(crate) struct Cursors(CursorType);

pub(crate) enum CursorType {
    Normal,
    Pointing,
}

fn main() {
    let df_win_des = WindowDescriptor {
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        resizable: false,
        title: "Jump Ferris".to_string(),
        // hide default cursor
        cursor_visible: false,
        ..Default::default()
    };

    App::new()
        .insert_resource(df_win_des)
        .insert_resource(ImageSettings::default_nearest())
        .add_state(GameState::Started)
        .add_plugins(DefaultPlugins)
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlatformPlugin)
        .add_plugin(PlayerPlugin)
        .init_resource::<CursorSprites>()
        .init_resource::<GameFont>()
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system(custom_cursors)
        .run();
}

fn setup(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut cursor_sprites: ResMut<CursorSprites>,
    mut game_font: ResMut<GameFont>,
) {
    // camera
    cmd.spawn_bundle(Camera2dBundle::default());

    // Font(s)
    let main_font: Handle<Font> = assets.load("fonts/GnuUnifontFull.ttf");
    game_font.main = main_font;

    // cursors
    let cursor_n_handle: Handle<Image> = assets.load("J-ZhengLi/cursor_normal.png");
    let cursor_p_handle: Handle<Image> = assets.load("J-ZhengLi/cursor_pointing.png");
    cursor_sprites.normal = cursor_n_handle.clone();
    cursor_sprites.pointing = cursor_p_handle.clone();
    // spawn as ui element
    cmd.spawn_bundle(ImageBundle {
        style: Style {
            size: Size {
                width: Val::Px(CURSOR_SIZE.x),
                height: Val::Px(CURSOR_SIZE.y),
            },
            ..Default::default()
        },
        image: UiImage(cursor_n_handle),
        ..Default::default()
    })
    .insert(Cursors(CursorType::Normal));
}

fn custom_cursors(
    mut cursor_event_reader: EventReader<CursorMoved>,
    mut cursors: Query<&mut Style, With<Cursors>>,
) {
    if let Ok(mut c_style) = cursors.get_single_mut() {
        for c_event in cursor_event_reader.iter() {
            c_style.position.bottom = Val::Px(c_event.position.y - CURSOR_SIZE.y);
            c_style.position.left = Val::Px(c_event.position.x);
        }
    }
}
