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

pub const DEFAULT_WIDTH: f32 = 360.0;
pub const DEFAULT_HEIGHT: f32 = 640.0;

fn main() {
    let df_win_des = WindowDescriptor {
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        resizable: false,
        title: "Jump Ferris".to_string(),
        ..Default::default()
    };

    App::new()
        .insert_resource(df_win_des)
        .insert_resource(ImageSettings::default_nearest())
        .add_system(close_on_esc)
        .add_state(GameState::Started)
        .add_plugins(DefaultPlugins)
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlatformPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_camera)
        .run()
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}
