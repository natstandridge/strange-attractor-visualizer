use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let material_emissive = materials.add(StandardMaterial {
        base_color: Color::INDIGO,
        metallic: 5.0,
        reflectance: 0.0,
        ..default()
    });

    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );
    
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 12000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
    
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(
        PbrBundle {
            mesh: mesh.clone(),
            material: material_emissive.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    );

}
