use bevy::prelude::*;
use crate::camera::MainCamera;

const SPRITE_STACK_SPACE: f32 = 3.;

#[derive(Component)]
pub struct VolumeObject {
    pub virtual_position: Vec3
}

#[derive(Component)]
pub struct StackedSprite {
    pub height: f32
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_volume_object_translation,
                             update_stacked_sprite_translation,
                             update_stacked_sprite_depth));
}

fn update_volume_object_translation(
    camera_query: Query<&Transform, With<MainCamera>>,
    mut volume_object_query: Query<(&mut Transform, &VolumeObject), Without<MainCamera>>
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    let camera_direction = camera_transform.up();

    for (mut transform, volume_object) in &mut volume_object_query {
        let offset = camera_direction * volume_object.virtual_position.z;
        transform.translation = volume_object.virtual_position + offset;
    }
}

fn update_stacked_sprite_translation(
    camera_query: Query<&Transform, With<MainCamera>>,
    mut stacked_sprite_query: Query<(&mut Transform, &StackedSprite), Without<MainCamera>>
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    let camera_direction = camera_transform.up();

    for (mut transform, stacked_sprite) in &mut stacked_sprite_query {
        transform.translation = camera_direction * stacked_sprite.height * SPRITE_STACK_SPACE;
    }
}

fn update_stacked_sprite_depth(
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

        transform.translation.z = -y_position;
    }
}
