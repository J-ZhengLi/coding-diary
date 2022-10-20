//! This mod is only meant to be used by `Platform` plugin
//! to show an ui for platforms editing.

use bevy::prelude::*;

use crate::{GameState, layers::Layer};

use super::{background::BG_SCALE, platform::RunningStatus};

pub(crate) struct PlatformEditorUi;

impl Plugin for PlatformEditorUi {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Editing).with_system(show_edit_ui))
            .add_system(handle_keyboard_input);
    }
}

fn show_edit_ui(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let arrow: Handle<Image> = assets.load("J-ZhengLi/arrow.png");
    let rot_arrow: Handle<Image> = assets.load("J-ZhengLi/rot_arrow.png");

    let c_pos = if let Some(c_pos) = windows.get_primary().and_then(|w| w.cursor_position()) {
        c_pos
    } else {
        return;
    };

    cmd.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(-50., 0., Layer::UI.into()).with_scale(BG_SCALE),
        texture: arrow.clone(),
        ..Default::default()
    });

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            flip_x: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(50., 0., Layer::UI.into()).with_scale(BG_SCALE),
        texture: arrow.clone(),
        ..Default::default()
    });
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