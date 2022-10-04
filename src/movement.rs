use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::perspective::PerspectiveCamera;

const MOVE_SPEED:f32 = 4.0;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(handle_movement)
            .add_system(handle_aim);
    }
}

fn handle_movement(
    mut cam_query: Query<&mut Transform, With<PerspectiveCamera>>,
    key: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut cam_transform = cam_query.get_single_mut().expect("No camera?");

    let mut movement_agg = Vec3::splat(0.0);

    if key.pressed(KeyCode::D) {
        movement_agg += cam_transform.right();
    }

    if key.pressed(KeyCode::A) {
        movement_agg += cam_transform.left();
    }

    if key.pressed(KeyCode::W) {
        movement_agg += cam_transform.forward();
    }

    if key.pressed(KeyCode::S) {
        movement_agg += cam_transform.back();
    }

    let movement = movement_agg.normalize() * MOVE_SPEED * time.delta_seconds();

    if movement.length() > 0.0 {
        cam_transform.translation += movement;
    }
}

fn handle_aim(
    mut cam_query: Query<&mut Transform, With<PerspectiveCamera>>,
    time: Res<Time>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let mut camera_transform = cam_query.get_single_mut().expect("No camera?");

    for ev in motion_evr.iter() {
        camera_transform.rotate_local_y(-ev.delta.x * time.delta_seconds());
    }
}
