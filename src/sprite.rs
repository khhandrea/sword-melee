use bevy::prelude::*;
use crate::camera::MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_sprite_depth);
}

fn update_sprite_depth(
    mut sprite_query: Query<(&mut Transform, &GlobalTransform), (Without<MainCamera>, With<Sprite>)>,
    camera_query: Query<&Transform, With<MainCamera>>
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    for (mut transform, global_transform) in &mut sprite_query {
        let camera_inverse_matrix = camera_transform.compute_matrix().inverse();
        let global_matrix = global_transform.compute_matrix();
        let relative_position_matrix = camera_inverse_matrix * global_matrix;
        let (_, _, relative_position) = relative_position_matrix.to_scale_rotation_translation();
        let y_position = relative_position.y;

        // transform.translation.z = -y_position;
    }
}
