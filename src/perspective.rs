use std::f32::consts::PI;

use bevy::prelude::*;

pub struct PerspectivePlugin;

impl Plugin for PerspectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_startup_system(spawn_light);
    }
}

#[derive(Component)]
pub struct PerspectiveCamera;

#[derive(Component)]
pub struct PerspectiveFlashlight;

fn spawn_camera(mut commands: Commands) {
    let start = Vec3::new(1.0, 0.5, 1.0);
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform {
                translation: start,
                ..Default::default()
            }
            .looking_at(Vec3::new(1.0, 0.5, 2.0), Vec3::Y),
            ..Default::default()
        })
        .insert(PerspectiveCamera)
        .insert_bundle(VisibilityBundle {
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(SpotLightBundle {
                    spot_light: SpotLight {
                        color: Color::YELLOW,
                        intensity: 200.0,
                        inner_angle: PI / 2.0,
                        outer_angle: PI / 1.7,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0,0.0,1.0),
                        ..Default::default()
                    },
                    // .looking_at(Vec3::new(1.0, 0.5, 2.0), Vec3::Y),
                    ..Default::default()
                })
                .insert(PerspectiveFlashlight);
        });
}

fn spawn_light(mut command: Commands) {
    command.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..Default::default()
    });
}

