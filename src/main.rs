#![allow(clippy::needless_pass_by_value)]
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use player::PlayerPlugin;
use world::World;

mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,
            enabled: true,
            ..Default::default()
        })
        .add_plugin(PlayerPlugin)
        .add_plugin(World)
        .add_system(cursor_unlocker)
        .run();
}

fn cursor_unlocker(
    mut window: Query<&mut Window>,
    key: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut window = window.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
