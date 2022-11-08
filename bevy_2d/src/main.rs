use bevy::app;
use bevy::DefaultPlugins;
use bevy::prelude::{
    ClearColor,
    StartupStage,
    WindowDescriptor,
};
mod entities;
mod window;
mod camera;
mod asset_manager;
mod player;

fn main() {
    let window_conf: WindowDescriptor = window::window_desc();
    app::App::new()
        .insert_resource(ClearColor(window::CLEAR))
        .insert_resource(window_conf)
        .add_startup_system(camera::spawn_camera)
        .add_startup_system(player::spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_manager::load_asset)
        .add_plugins(DefaultPlugins)
        // .add_plugin(entities::PeoplePlugin)
        .run();
}
