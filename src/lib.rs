use bevy::prelude::*;

mod camera;
pub mod player;
pub mod scene;
mod sprite_stack;
mod sprite;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            camera::plugin,
            player::plugin,
            scene::plugin,
            sprite_stack::plugin,
            sprite::plugin
        ));
    }
}
