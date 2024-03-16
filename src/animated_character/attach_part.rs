use super::{
    print_scene_tree::walk_tree,
    spawn_characters::{PlayerCharacterName, SceneEntitiesByName},
};
use bevy::{
    prelude::*,
    render::{mesh::skinning::SkinnedMesh, primitives::Aabb},
    transform,
    utils::HashMap,
};
use std::{collections::VecDeque, f32::consts::PI};

pub fn attach_part(
    mut commands: Commands,
    scene_query: Query<Entity, With<PlayerCharacterName>>,
    skinned_entity_query: Query<Entity, With<SkinnedMesh>>,
    player_character_query: Query<(Entity, &PlayerCharacterName), With<PlayerCharacterName>>,
    all_entities_with_children: Query<&Children>,
    all_entities_with_parents: Query<&Parent>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut transforms: Query<&mut Transform>,
    mut global_transforms: Query<&mut GlobalTransform>,
    mut axis_aligned_bounding_boxes: Query<(Entity, &mut Aabb), With<Aabb>>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
    names: Query<&Name>,
) {
    let mut main_bones = HashMap::new();
    let mut main_armature_option = None;
    let main_skeleton_scene_entity_option = scene_entities_by_name.0.get("main_skeleton");
    if let Some(main_skeleton_scene_entity) = main_skeleton_scene_entity_option {
        let root_bone = find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_skeleton_scene_entity,
            "Root",
        )
        .expect("skeleton to have bones");

        main_armature_option = find_child_with_name_containing(
            &all_entities_with_children,
            &names,
            &main_skeleton_scene_entity,
            "CharacterArmature",
        );
        // walk_tree(&all_entities_with_children, &names, &player_character_entity, depth);

        collect_bones(
            &all_entities_with_children,
            &names,
            &root_bone,
            &mut main_bones,
        );

        println!("main bones: {:#?}", main_bones);
    }
    // for (player_character_entity, player_character_name) in &player_character_query {
    //     let depth = 0;
    //     println!("found player character name: {}", player_character_name.0);
    // if player_character_name.0 == "main_skeleton" {
    //     let root_bone = find_child_with_name_containing(
    //         &all_entities_with_children,
    //         &names,
    //         &player_character_entity,
    //         "Root",
    //     )
    //     .expect("skeleton to have bones");

    //     main_armature_option = find_child_with_name_containing(
    //         &all_entities_with_children,
    //         &names,
    //         &player_character_entity,
    //         "CharacterArmature",
    //     );
    //     // walk_tree(&all_entities_with_children, &names, &player_character_entity, depth);

    //     collect_bones(
    //         &all_entities_with_children,
    //         &names,
    //         &root_bone,
    //         &mut main_bones,
    //     );

    //     println!("main bones: {:#?}", main_bones);
    // }
    // }

    for (player_character_entity, player_character_name) in &player_character_query {
        if player_character_name.0 == "Sword" {
            let mut entity_commands = commands.entity(player_character_entity);
            // if let Ok(mut transform) = transforms.get_mut(player_character_entity) {
            //     println!("ROTATING");
            //     transform.rotate(Quat::from_rotation_x(1.0));

            //     transform.translation.x += 1.0;
            // }
            if let Some(hand_bone) = main_bones.get("EquipmentHandle.R") {
                entity_commands.set_parent(*hand_bone);
            }
        } else if player_character_name.0 == "SciFi Torso"
            || player_character_name.0 == "Witch Legs"
        {
            println!("dealing with: {}", player_character_name.0);
            let root_bone_option = find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &player_character_entity,
                "Root",
            );

            let part_armature_option = find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &player_character_entity,
                "CharacterArmature",
            );

            if let Some(main_armature) = main_armature_option {
                if let Some(part_armature) = part_armature_option {
                    let mut part_armature_entity_commands = commands.entity(part_armature);
                    if let Ok(mut transform) = transforms.get_mut(part_armature) {
                        transform.translation.x = 0.0;
                        transform.translation.y = 0.0;
                        transform.translation.z = 0.0;
                        transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
                    }
                    part_armature_entity_commands.set_parent(main_armature);
                }
            }

            if let Some(root_bone) = root_bone_option {
                let mut part_bones = HashMap::new();
                collect_bones(
                    &all_entities_with_children,
                    &names,
                    &root_bone,
                    &mut part_bones,
                );
                println!("part bones: {:#?}", part_bones);

                for (name, part_bone) in part_bones {
                    let mut entity_commands = commands.entity(part_bone);
                    let new_parent_option = main_bones.get(&name);
                    if let Some(new_parent) = new_parent_option {
                        if let Ok(mut transform) = transforms.get_mut(part_bone) {
                            transform.translation.x = 0.0;
                            transform.translation.y = 0.0;
                            transform.translation.z = 0.0;
                            transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
                        }
                        entity_commands.set_parent(*new_parent);
                    }
                }
            }
        }
    }
}

pub fn collect_bones(
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    root_bone: &Entity,
    collected: &mut HashMap<String, Entity>,
) {
    if let Ok(name) = names.get(*root_bone) {
        collected.insert(format!("{}", name), *root_bone);

        if let Ok(children) = all_entities_with_children.get(*root_bone) {
            for child in children {
                collect_bones(all_entities_with_children, names, child, collected)
            }
        }
    }
}

pub fn find_child_with_name_containing(
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    entity: &Entity,
    name_to_match: &str,
) -> Option<Entity> {
    let mut queue = VecDeque::new();
    queue.push_back(entity);

    while let Some(curr_entity) = queue.pop_front() {
        let name_result = names.get(*curr_entity);
        if let Ok(name) = name_result {
            if format!("{}", name).contains(name_to_match) {
                println!("found named entity {}", name);
                return Some(*curr_entity);
            }
        }

        let children_result = all_entities_with_children.get(*curr_entity);
        if let Ok(children) = children_result {
            for child in children {
                queue.push_back(child);
            }
        }
    }

    None
}
