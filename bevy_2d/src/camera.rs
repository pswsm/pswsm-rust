use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    let mut camera: Camera2dBundle = Camera2dBundle::default();
    camera.projection.top = 1.0;
    camera.projection.right = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.left = -1.0;

    commands.spawn_bundle(camera);
}
