use bevy::prelude::*;
use rand::Rng;

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

#[derive(Resource)]
struct PointMarker(shape::Icosphere);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut trajectories: ResMut<Trajectories>
) {

    // material for sphere
    let material = materials.add(StandardMaterial {
        base_color: Color::INDIGO,
        // metallic: 5.0,
        // reflectance: 0.0,
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
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 8000.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 4.0, 0.0),
    //     ..default()
    // });
    
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let mut rng = rand::thread_rng();

    // looping through to create starting points
    for _ in 0..20 {

        let pos: Vec3 = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

        // spawn sphere from cloned mesh
        commands.spawn(
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                ..default()
            }
        );
        trajectories.0.push(vec![Vec3::new(pos.x, pos.y, pos.z)]); // Vec3 added to trajectory within Vec[Vec[]]

    }
}

fn get_next_point(input_point: &Vec3) -> Vec3 {
    let rho: f32  = 25.0;
    let sigma: f32 = 10.0;
    let beta: f32 = 8.0/3.0;

    let dx: f32 = sigma*(input_point.y - input_point.x);
    let dy: f32 = input_point.x*(rho - input_point.z) - input_point.y;
    let dz: f32 = input_point.x*input_point.y - beta*input_point.z;

    // println!("dx: {}, dy: {}, dz: {}", dx, dy, dz);

    let sf = 0.01;

    Vec3::new(input_point.x+sf*dx, input_point.y+sf*dy, input_point.z+sf*dz)
}

fn calculate_and_draw_trajectories(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut trajectories: ResMut<Trajectories>,
    mut camera: Query<&mut Transform, With<Camera>>
    // point_marker: ResMut<PointMarker>
) {

    // mesh for sphere
    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.1,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );

    // material for sphere
    let material = materials.add(StandardMaterial {
        base_color: Color::INDIGO,
        metallic: 5.0,
        reflectance: 0.0,
        ..default()
    });

    // println!("trajectories len: {:#?}", trajectories.0.len());

    /* calculate new point according to */
    let mut comx = 0.0;
    let mut comy = 0.0;
    let mut comz = 0.0;

    for trajectory in trajectories.0.iter_mut() {
        /* get latest point in this trajectory */
        match trajectory.last() {
            Some(point) => {

                /* update center of mass calculation */
                comx += point.x;
                comy += point.y;
                comz += point.z;

                /* get the new point for this trajectory */
                let new_point = get_next_point(point);

                // println!("new_point: {:#?}", new_point);

                /* spawn a sphere there */
                commands.spawn(
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform::from_xyz(new_point.x, new_point.y, new_point.z),
                        ..default()
                    }
                );

                trajectory.push(new_point);
            },
            None => {}
        }
    }

    let num_points = trajectories.0.len() as f32;
    comx /= num_points;
    comy /= num_points;
    comz /= num_points;

    for mut transform in camera.iter_mut() {
        *transform = Transform::from_xyz(-50.0, 50.0, 50.0).looking_at(Vec3::new(comx, comy, comz), Vec3::Y)
    }

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Trajectories(Vec::new()))
        .insert_resource(PointMarker(shape::Icosphere { radius: 0.1, subdivisions: 5 }.try_into().unwrap()))
        .add_startup_system(setup)
        .add_system(calculate_and_draw_trajectories)
        .run();
}