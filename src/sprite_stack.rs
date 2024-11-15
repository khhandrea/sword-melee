use bevy::prelude::*;
use crate::scene::StackedSprite;
use crate::camera::MainCamera;

const SPRITE_STACK_SPACE: f32= 3.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_sprite_stack);
}

fn update_sprite_stack(
    mut transform_query: Query<(&mut Transform, &StackedSprite), Without<MainCamera>>,
    camera_query: Query<&Transform, With<MainCamera>>
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    let camera_direction = camera_transform.up();

    for (mut transform, stacked_sprite) in &mut transform_query {
        transform.translation = camera_direction * stacked_sprite.height as f32 * SPRITE_STACK_SPACE;
    }
}
