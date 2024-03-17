mod attach_part_to_main_skeleton;
mod collect_bones;
mod find_child_with_name_containing;
mod get_main_skeleton_bones_and_armature;
use self::{
    attach_part_to_main_skeleton::attach_part_to_main_skeleton,
    get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature,
};
use super::spawn_characters::{SceneEntitiesByName, SceneName};
use bevy::prelude::*;

pub fn assemble_parts(
    mut commands: Commands,
    scene_query: Query<(Entity, &SceneName), With<SceneName>>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
    all_entities_with_children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    names: Query<&Name>,
) {
    let (main_skeleton_bones, main_armature_entity) = get_main_skeleton_bones_and_armature(
        scene_entities_by_name,
        &all_entities_with_children,
        &names,
    );

    for (part_scene_entity, part_scene_name) in &scene_query {
        if part_scene_name.0 == "Sword Golden.glb" {
            let mut entity_commands = commands.entity(part_scene_entity);
            if let Some(hand_bone) = main_skeleton_bones.get("EquipmentHandle.R") {
                entity_commands.set_parent(*hand_bone);
            }
        } else if part_scene_name.0 == "SciFi Torso.glb" || part_scene_name.0 == "Witch Legs.glb" {
            attach_part_to_main_skeleton(
                &mut commands,
                &all_entities_with_children,
                &mut transforms,
                &names,
                &part_scene_name.0,
                &part_scene_entity,
                &main_armature_entity,
                &main_skeleton_bones,
            )
        }
    }
}