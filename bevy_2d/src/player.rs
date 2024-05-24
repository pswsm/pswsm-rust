use crate::asset_manager::AsciiSheet;
use bevy::prelude::{
    Color,
    Commands,
    Component,
    DerefMut,
    Res,
    Sprite,
    //Transform,
    //Name,
    SpriteBundle,
    SpriteSheetBundle,
    TextureAtlasSprite,
    Transform,
    Vec2,
    Vec3,
};
use bevy::time::prelude::Timer;
use bevy::time::TimerMode;

#[derive(Component, DerefMut, Deref)]
struct AnimationTimer(Timer);

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite: TextureAtlasSprite = TextureAtlasSprite::new(12);
    sprite.color = Color::RED;
    sprite.custom_size = Some(Vec2::splat(1.0));

    commands.spawn_batch(vec![
        SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ])

    // commands.spawn_batch(vec![SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(1.0, 0.0, 0.0),
    //         custom_size: Some(Vec2::new(150.0, 150.0)),
    //         ..Default::default()
    //     },
    //     transform: Transform {
    //         scale: Vec3::new(0.10, 0.10, 0.10),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }])
}
