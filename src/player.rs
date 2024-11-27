use bevy::prelude::*;
use crate::sprite_sheet::PlayerSpriteSheet;

const PLAYER_SPEED: f32 = 100.;
const ROTATION_SPEED: f32 = 2.;

#[derive(Component)]
pub struct Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_player);
    app.add_systems(Update, move_player);
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_atlas: Res<PlayerSpriteSheet>
) {
    let sprite: Handle<Image> = asset_server.load("test.png");

    commands.spawn((
        Player,
        SpriteBundle {
            transform: Transform::from_xyz(-1., 0., 2.),
            texture: sprite,
            ..default()
        },
        TextureAtlas {
            layout: sprite_atlas.0.clone(),
            index: 0
        }
    ));
}

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>
) {
    let Ok(mut player_transform) = player.get_single_mut() else {
        return;
    };

    let mut movement_factor = Vec2::ZERO;
    let mut rotation_factor = 0.0;

    if kb_input.pressed(KeyCode::KeyW) {
        movement_factor.y += 1.;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        movement_factor.y -= 1.;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        movement_factor.x -= 1.;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        movement_factor.x += 1.;
    }
    if kb_input.pressed(KeyCode::KeyY) {
        rotation_factor += 1.;
    }
    if kb_input.pressed(KeyCode::KeyO) {
        rotation_factor -= 1.;
    }

    // Rotate
    let rotation_amount = rotation_factor * ROTATION_SPEED * time.delta_seconds();
    player_transform.rotate_z(rotation_amount);

    // Move
    let movement_distance = movement_factor.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    let movement_direction = player_transform.rotation;
    let translation_delta = movement_direction * movement_distance.extend(0.);
    player_transform.translation += translation_delta;
}

