use bevy::prelude::*;

pub struct SimpleScene;

impl Plugin for SimpleScene {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_plane)
            .add_startup_system(add_box)
            .add_startup_system(add_light);
    }
}

pub fn add_plane(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    command.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 7.0 })),
        material: materials.add(Color::SEA_GREEN.into()),
        ..Default::default()
    });
}

pub fn add_box(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    command.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::PINK.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    command.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::INDIGO.into()),
        transform: Transform::from_xyz(-2.0, 0.25, -3.0),
        ..Default::default()
    });
    command.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.2 })),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(1.0, 0.6, 2.0),
        ..Default::default()
    });
}

pub fn add_light(mut command: Commands) {
    command.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(3.0, 10.0, 5.0),
        ..Default::default()
    });
}
