use bevy::app;
use bevy::DefaultPlugins;
use bevy::prelude::{
    ClearColor,
    StartupStage,
    WindowDescriptor,
};
use bevy::window::PresentMode;
mod entities;
mod window;
mod camera;
mod asset_manager;
mod player;

fn main() {
    app::App::new()
        .insert_resource(ClearColor(window::CLEAR))
        .insert_resource(WindowDescriptor {
                title: "WebWorld".to_string(),
                present_mode: PresentMode::Fifo,
                ..Default::default()
            }
        )
        .add_startup_system(camera::spawn_camera)
        .add_startup_system(player::spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_manager::load_asset)
        .add_plugins(DefaultPlugins)
        .run();
}
