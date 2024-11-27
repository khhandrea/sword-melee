use bevy::prelude::*;

mod camera;
mod player;
mod scene;
mod sprite_sheet;
mod volume_object;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                DefaultPlugins,
                camera::plugin,
                player::plugin,
                scene::plugin,
                volume_object::plugin
            ))
            .init_resource::<sprite_sheet::PlayerSpriteSheet>()
            .init_resource::<sprite_sheet::BoxSpriteSheet>();
    }
}
