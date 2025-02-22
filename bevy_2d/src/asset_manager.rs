use bevy::prelude::{
    AssetServer, Assets, Commands, Handle, Res, ResMut, Resource, TextureAtlas, Vec2,
};

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub fn load_asset(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut mut_assets: ResMut<Assets<TextureAtlas>>,
) {
    let image: Handle<_> = assets.load("ascii.png");
    let atlas: TextureAtlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Some(Vec2::splat(2.0)),
        Some(Vec2::splat(0.0)),
    );
    let atlas_handle: Handle<TextureAtlas> = mut_assets.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}
