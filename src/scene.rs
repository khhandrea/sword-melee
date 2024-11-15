use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
pub struct StackedSprite { 
    pub height: usize
}

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
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1000., 700.))),
            material: materials.add(Color::srgb(0.4, 0.7, 0.3)),
            ..default()
        }
    );
}

fn spawn_block(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let sprite: Handle<Image> = asset_server.load("box.png");
    let layout = TextureAtlasLayout::from_grid((32, 32).into(), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(
        SpatialBundle::from_transform(Transform::from_xyz(32., 32., 1.))
    ).with_children(|parent| {
        for i in 0..8 {
            parent.spawn((
                StackedSprite {
                    height: i
                },
                SpriteBundle {
                    transform: Transform::from_xyz(0., 0., i as f32),
                    texture: sprite.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: i
                }
            ));
        }
    });

}

