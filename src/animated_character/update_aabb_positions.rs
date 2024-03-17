use super::spawn_characters::SceneName;
use bevy::{prelude::*, render::primitives::Aabb};

pub fn update_aabb_positions(
    scene_query: Query<(Entity, &SceneName), With<SceneName>>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    mut axis_aligned_bounding_boxes: Query<(Entity, &mut Aabb), With<Aabb>>,
) {
    for (scene_entity, character_name) in &scene_query {
        if character_name.0 == "SciFi Torso" {
            for (entity, mut aabb) in &mut axis_aligned_bounding_boxes {
                if let Ok(global_transform) = global_transforms.get(entity) {
                    aabb.center.x = global_transform.translation().x;
                    aabb.center.y = global_transform.translation().y;
                    aabb.center.z = global_transform.translation().z;
                }
            }
        }
    }
}
