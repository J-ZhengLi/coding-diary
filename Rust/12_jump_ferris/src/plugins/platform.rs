use bevy::prelude::*;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_platfrom);
    }
}

fn init_platfrom() {
    println!("initialize platforms here");
}