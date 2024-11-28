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
    mut stacked_sprite_query: Query<(&mut Transform, &GlobalTransform, &StackedSprite), Without<MainCamera>>
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    let camera_up = camera_transform.up().as_vec3();

    for (mut transform, global_transform, stacked_sprite) in &mut stacked_sprite_query {
        let sprite_up = global_transform.up().as_vec3();
        let sprite_to_camera = camera_up - sprite_up;
        let ccw = sprite_up.cross(sprite_to_camera).z;
        let angle = ccw * camera_up.angle_between(sprite_up);
        let rotation = Quat::from_axis_angle(Vec3::Z, angle);
        let direction = rotation * Vec3::Y;
        transform.translation = direction * stacked_sprite.height * SPRITE_STACK_SPACE;
    }
}

fn update_stacked_sprite_depth(
    camera_query: Query<&Transform, With<MainCamera>>,
    volume_object_query: Query<&VolumeObject>,
    mut sprite_query: Query<(&mut Transform, &Parent, &StackedSprite), Without<MainCamera>>
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    let camera_up = camera_transform.up();

    let mut sorted_sprites: Vec<_> = sprite_query.iter_mut().collect();
    sorted_sprites.sort_by(|a, b| {
        let a_parent = volume_object_query.get(a.1.get()).unwrap();
        let b_parent = volume_object_query.get(b.1.get()).unwrap();

        let a_pos = a_parent.virtual_position;
        let b_pos = b_parent.virtual_position;

        let a_proj = camera_up.as_vec3().dot(a_pos);
        let b_proj = camera_up.as_vec3().dot(b_pos);

        match a_proj.partial_cmp(&b_proj).unwrap() {
            std::cmp::Ordering::Equal => {
                // If x, y are equal, use height
                b.2.height.partial_cmp(&a.2.height).unwrap()
            },
            ord => ord
        }
    });

    for (i, (mut transform, _, _)) in sorted_sprites.into_iter().enumerate() {
        transform.translation.z = -0.001 * i as f32;
    }
}
