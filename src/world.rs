#![allow(clippy::cast_precision_loss)]
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub struct World;

#[derive(Component)]
pub struct BouncingSphere {
    pub time: f32,
}

impl Plugin for World {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(sphere_func);
    }
}

fn sphere_func(mut q: Query<(&mut Transform, &mut BouncingSphere)>) {
    q.for_each_mut(|mut i| {
        i.1.time += 0.1;
        i.0.translation.y = i.1.time.sin() + 2.0;
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Initialising world");
    //Ground plane
    commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            mesh: meshes.add(shape::Box::new(100.0, 2.0, 100.0).into()),
            material: materials.add(Color::GRAY.into()),
            ..Default::default()
        })
        .insert(Collider::cuboid(50.0, 1.0, 50.0));

    //Test sphere
    let sphere_mesh = meshes.add(
        shape::UVSphere {
            radius: 0.4,
            sectors: 20,
            stacks: 20,
        }
        .into(),
    );
    let sphere_material = materials.add(Color::RED.into());

    for i in -5..5 {
        commands
            .spawn(PbrBundle {
                mesh: sphere_mesh.clone(),
                material: sphere_material.clone(),
                transform: Transform::from_xyz(i as f32, 1.0, 5.0),
                ..Default::default()
            })
            // .insert(BouncingSphere { time: i as f32 })
            .insert(Collider::ball(0.4));
    }

    //Some boxes
    // let mut rng = rand::thread_rng();
    // let box_material = materials.add(Color::ANTIQUE_WHITE.into());
    // for _ in 0..0 {
    //     let x = rng.gen_range(1.0..10.0);
    //     let y = rng.gen_range(1.0..10.0);
    //     let z = rng.gen_range(1.0..10.0);

    //     let x_pos = rng.gen_range(-20.0..20.0) + 3.0;
    //     let z_pos = rng.gen_range(-20.0..20.0) + 3.0;
    //     commands
    //         .spawn(PbrBundle {
    //             mesh: meshes.add(shape::Box::new(x, y, z).into()),
    //             material: box_material.clone(),
    //             ..Default::default()
    //         })
    //         .insert(TransformBundle {
    //             local: Transform {
    //                 translation: Vec3 {
    //                     x: x_pos,
    //                     y: y / 2.0,
    //                     z: z_pos,
    //                 },
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         })
    //         .insert(Collider::cuboid(x / 2.0, y / 2.0, z / 2.0));
    // }

    commands
        .spawn(TransformBundle {
            local: Transform {
                translation: Vec3 {
                    x: 5.0,
                    y: 1.0,
                    z: 5.0,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::capsule_y(1.0, 0.5));

    //Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        visibility: Visibility::Visible,
        transform: Transform::default().looking_to(Vec3::NEG_Y, Vec3::Z),
        ..Default::default()
    });
}
