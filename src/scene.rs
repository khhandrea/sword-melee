use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::sprite_sheet::{BoxSpriteSheet};
use crate::volume_object::{spawn_volume_object};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_background, spawn_block));
}

fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        MaterialMesh2dBundle {
            transform: Transform::from_xyz(0., 0., -100.),
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1000., 700.))),
            material: materials.add(Color::srgb(0.4, 0.7, 0.3)),
            ..default()
        }
    );
}

fn spawn_block(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_atlas: Res<BoxSpriteSheet>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>
) {
    let sprite: Handle<Image> = asset_server.load("box.png");

    spawn_volume_object(
        &mut commands, 
        &texture_atlas_layouts,
        sprite.clone(),
        sprite_atlas.0.clone(),
        Vec3::new(64., 32., 0.),
        ()
    );
    spawn_volume_object(
        &mut commands, 
        &texture_atlas_layouts,
        sprite.clone(),
        sprite_atlas.0.clone(),
        Vec3::new(64., 64., 0.),
        ()
    );
    spawn_volume_object(
        &mut commands, 
        &texture_atlas_layouts,
        sprite.clone(),
        sprite_atlas.0.clone(),
        Vec3::new(32., 32., 0.),
        ()
    );
}

