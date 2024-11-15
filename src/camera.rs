use bevy::prelude::*;
use crate::player::Player;

const CAMERA_POSITION:f32 = 180.;
const CAMERA_SMOOTHNESS: f32 = 5.;

#[derive(Component)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera);
    app.add_systems(Update, follow_player);
}

fn setup_camera(
    mut commands: Commands
) {
    commands.spawn((
        MainCamera,
        Camera2dBundle::default()
    ));
}

fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    let target_position = player_transform.translation + player_transform.up() * CAMERA_POSITION;

    camera_transform.translation = camera_transform.translation.lerp(
        target_position,
        time.delta_seconds() * CAMERA_SMOOTHNESS
    );

    camera_transform.rotation = camera_transform.rotation.slerp(
        player_transform.rotation,
        time.delta_seconds() * CAMERA_SMOOTHNESS
    );
}

