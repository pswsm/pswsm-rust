use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    let camera: Camera2dBundle = Camera2dBundle::default();
    commands.spawn_bundle(camera);
}
