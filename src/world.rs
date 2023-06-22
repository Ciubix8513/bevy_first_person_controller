use bevy::prelude::*;

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
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(100.0, 0.1, 100.0).into()),
        material: materials.add(Color::GRAY.into()),
        ..Default::default()
    });

    for i in -5..5 {
        //Test sphere
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(
                    shape::UVSphere {
                        radius: 0.4,
                        sectors: 20,
                        stacks: 20,
                    }
                    .into(),
                ),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_xyz(i as f32, 1.0, 5.0),
                ..Default::default()
            },
            BouncingSphere { time: i as f32 },
        ));
    }
    //Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        visibility: Visibility::Visible,
        transform: Transform::default().looking_to(Vec3::NEG_Y, Vec3::Z),
        ..Default::default()
    });
}
