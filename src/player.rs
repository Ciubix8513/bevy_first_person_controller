#![allow(clippy::module_name_repetitions, clippy::needless_pass_by_value)]
use bevy::{input::mouse::MouseMotion, math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::{Collider, RigidBody};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerCamera;
#[derive(Component)]
pub struct FirstPersonController {
    pub speed: f32,
    pub jump_force: f32,
    pub movement_vec: Vec2,
}

impl Default for FirstPersonController {
    fn default() -> Self {
        Self {
            speed: 0.1,
            jump_force: 1.0,
            movement_vec: Vec2::ZERO,
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player)
            .add_system(keyboard_event)
            .add_system(mouse_controls)
            .add_system(move_player);
    }
}

fn add_player(mut commands: Commands) {
    //Camera
    commands
        //Use transform bundle instead bc it has global transform. Which is need behind the scenes
        .spawn((
            FirstPersonController::default(),
            TransformBundle {
                local: Transform::from_xyz(0.0, 1.0, 0.0),
                ..Default::default()
            },
            Collider::capsule_y(1.0, 0.5),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 1.8, 0.0).looking_to(Vec3::Z, Vec3::Y),
                    projection: PerspectiveProjection {
                        fov: 90.0,
                        ..Default::default()
                    }
                    .into(),
                    camera_3d: Camera3d {
                        clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                            Color::rgb(0.33, 0.8, 0.98),
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                PlayerCamera,
            ));
        });
}

fn keyboard_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut FirstPersonController, &Transform)>,
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
    let mut movement_vec = movement_vec.normalize_or_zero();

    if keyboard_input.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        movement_vec *= 2.0;
    }

    let mut query = query.single_mut();
    let rot = query.1.rotation;
    query.0.movement_vec = rot.mul_vec3(movement_vec * -0.1).xz();
}

fn move_player(mut query: Query<(&mut RigidBody, &mut FirstPersonController)>) {
    let query = query.single_mut();
    let mut controller = query.1;

    if controller.movement_vec.length_squared() == 0.0 {
        return;
    }

    controller.movement_vec = Vec2::ZERO;
}

fn mouse_controls(
    mut player_transform: Query<&mut Transform, With<FirstPersonController>>,
    //You can have only one mutable reference to a value, so without specifying without it might
    //try to get the player transform, which isn't allowed
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<FirstPersonController>)>,
    mut motion: EventReader<MouseMotion>,
) {
    if let Some(motion) = motion.iter().collect::<Vec<_>>().first() {
        let mut rotation = motion.delta * 0.1;
        rotation.x *= -1.0;

        let mut player_transform = player_transform.single_mut();
        let mut camera_transform = camera.single_mut();
        let camera_rotation_euler = camera_transform.rotation.to_euler(EulerRot::XYZ);

        //A lot of degree/radians conversions, but it's fine
        let vertical_rotation = camera_rotation_euler.0.to_degrees() - rotation.y;

        camera_transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            if vertical_rotation > 180.0 {
                vertical_rotation - 360.0
            } else {
                vertical_rotation
            }
            .clamp(-90.0, 90.0)
            .to_radians(),
            0.0,
            0.0,
        );
        player_transform.rotate_y(rotation.x.to_radians());
    }
}
