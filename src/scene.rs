use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::sprite_sheet::{BoxSpriteSheet};
use crate::volume_object::{StackedSprite, VolumeObject};

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
    let Some(texture_atlas_layout) = texture_atlas_layouts.get(&sprite_atlas.0) else {
        return;
    };
    let max_index = texture_atlas_layout.textures.len() - 1;

    let sprite: Handle<Image> = asset_server.load("box.png");

    commands.spawn((
        VolumeObject {
            virtual_position: Vec3::ZERO
        },
        SpatialBundle::default()
    )).with_children(|parent| {
        for i in 0..=max_index {
            parent.spawn((
                StackedSprite {
                    height: i as f32
                },
                SpriteBundle {
                    texture: sprite.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: sprite_atlas.0.clone(),
                    index: i
                }
            ));
        }
    });
}

