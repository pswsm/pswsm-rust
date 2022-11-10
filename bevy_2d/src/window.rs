use bevy::window::{
    WindowDescriptor,
    PresentMode,
};
use bevy::render::color::Color;

pub const DARK: Color = Color::rgb(0.0, 0.0, 0.0);
pub const CLEAR: Color = Color::rgb(1.0, 1.0, 1.0);

pub fn window_desc() -> WindowDescriptor {
    WindowDescriptor {
        title: "WebWorld".to_string(),
        present_mode: PresentMode::Fifo,
        ..Default::default()
    }
}
