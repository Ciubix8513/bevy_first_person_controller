#![allow(clippy::module_name_repetitions, clippy::needless_pass_by_value)]
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct GameCamera;
#[derive(Component)]
pub struct PlayerBundle;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player)
            .add_system(keyboard_event);
    }
}

fn keyboard_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<PlayerBundle>>,
) {
    let mut movement_vec = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        movement_vec.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement_vec.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement_vec.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement_vec.x -= 1.0;
    }
    let movement_vec = movement_vec.normalize_or_zero();

    let mut t = player_transform.single_mut();
    t.translation += movement_vec * 0.1;
}

fn add_player(mut commands: Commands) {
    //Camera
    commands
        //Use transform bundle instead bc it has global transform. Which is need behind the scenes
        .spawn((PlayerBundle, TransformBundle::default()))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 2.0, 0.0).looking_to(Vec3::Z, Vec3::Y),
                    projection: PerspectiveProjection {
                        fov: 90.0,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                },
                GameCamera,
            ));
        });
}
