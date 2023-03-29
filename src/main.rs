use bevy::prelude::*;

#[derive(Resource)]
struct Trajectories(Vec<Vec<Vec3>>); // for storing starting points and their trajectories
/*  
    Vec[
        Vec[ // starting point 1
            Vec3[0.0, 0.0, 0.0],
            Vec3[0.0, 0.0, 0.0] // point added as the next point in the line based on starting point
            ]
        Vec[ // starting point 2
            Vec3[0.0, 0.0, 0.0]
            ]
        Vec[ // starting point 3, etc...

            ]
        ]
    */

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut trajectories: ResMut<Trajectories>
) {

    // material for sphere
    let material = materials.add(StandardMaterial {
        base_color: Color::INDIGO,
        metallic: 5.0,
        reflectance: 0.0,
        ..default()
    });

    // mesh for sphere
    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.1,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );
    
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..default()
    });
    
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // looping through to create starting points
    for z in vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5] {
        // spawn sphere from cloned mesh
        commands.spawn(
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, z),
                ..default()
            }
        );
        trajectories.0.push(vec![Vec3::new(0.0,0.0,z)]); // Vec3 added to trajectory within Vec[Vec[]]
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Trajectories(Vec::new()))
        .add_startup_system(setup)
        .run();
}