use bevy::prelude::*;
use movement::MovementPlugin;
use perspective::PerspectivePlugin;
use simple_scene::SimpleScene;

mod movement;
mod perspective;
mod simple_scene;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgba(0.1, 0.2, 0.3, 1.)))
        .insert_resource(WindowDescriptor {
            width: 1422.0,
            height: 800.0,
            title: "Pac-3d".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PerspectivePlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(SimpleScene)
        .run();
}
