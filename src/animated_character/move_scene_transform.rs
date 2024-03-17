use super::spawn_scenes::SceneName;
use bevy::prelude::*;

pub fn move_scene_transform(
    scene_query: Query<(Entity, &SceneName), With<SceneName>>,
    mut transforms: Query<&mut Transform>,
    mut global_transforms: Query<&mut GlobalTransform>,
) {
    for (scene_entity, character_name) in &scene_query {
        if character_name.0 == "SciFi Torso" {
            if let Ok(mut transform) = transforms.get_mut(scene_entity) {
                println!("transform: {:#?}", transform);
                transform.translation.x += 0.001;
            }
            // if let Ok(mut global_transform) = global_transforms.get_mut(scene_entity) {
            //     // println!("transform: {:#?}", transform);
            // }
        }
    }
}
