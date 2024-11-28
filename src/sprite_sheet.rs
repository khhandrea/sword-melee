use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerSpriteSheet(pub Handle<TextureAtlasLayout>);

impl FromWorld for PlayerSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (32, 32).into(),
            8, 1,
            None, None
        );
        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlasLayout>>().unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}

#[derive(Resource)]
pub struct BoxSpriteSheet(pub Handle<TextureAtlasLayout>);

impl FromWorld for BoxSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (32, 32).into(),
            8, 1,
            None, None
        );
        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlasLayout>>().unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}
