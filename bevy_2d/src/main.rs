use bevy::app;
use bevy::prelude::ClearColor;
use bevy::prelude::PluginGroup;
use bevy::prelude::PreStartup;
use bevy::prelude::Startup;
use bevy::window::Window;
use bevy::window::WindowPlugin;
use bevy::window::WindowTheme;
use bevy::DefaultPlugins;
mod asset_manager;
mod camera;
mod entities;
mod player;
mod window;

fn main() {
    app::App::new()
        .insert_resource(ClearColor(window::DARK))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Webworld".into(),
                    resolution: (1800., 790.).into(),
                    window_theme: Some(WindowTheme::Dark),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            entities::PeoplePlugin,
        ))
        .add_systems(Startup, (camera::spawn_camera, player::spawn_player))
        .add_systems(PreStartup, asset_manager::load_asset)
        // .add_startup_system(camera::spawn_camera)
        // .add_startup_system(player::spawn_player)
        // .add_startup_system_to_stage(StartupStage::PreStartup, asset_manager::load_asset)
        // .add_plugin(entities::PeoplePlugin)
        .run();
}
