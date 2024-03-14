use std::collections::VecDeque;

use super::{print_scene_tree::walk_tree, spawn_characters::PlayerCharacterName};
use bevy::{prelude::*, render::mesh::skinning::SkinnedMesh, utils::HashMap};

pub fn attach_part(
    mut commands: Commands,
    scene_query: Query<Entity, With<PlayerCharacterName>>,
    skinned_entity_query: Query<Entity, With<SkinnedMesh>>,
    player_character_query: Query<(Entity, &PlayerCharacterName), With<PlayerCharacterName>>,
    all_child_entities: Query<&Children>,
    all_entities_with_parents: Query<&Parent>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    names: Query<&Name>,
) {
    let mut main_bones = HashMap::new();
    for (player_character_entity, player_character_name) in &player_character_query {
        let depth = 0;
        println!("found player character name: {}", player_character_name.0);
        // if player_character_name.0 == "main_skeleton" {
        if player_character_name.0 == "main_skeleton" {
            let root_bone = find_child_with_name_containing(
                &all_child_entities,
                &names,
                &player_character_entity,
                "Root",
            )
            .expect("skeleton to have bones");
            // walk_tree(&all_child_entities, &names, &player_character_entity, depth);

            collect_bones(&all_child_entities, &names, &root_bone, &mut main_bones);

            println!("main bones: {:#?}", main_bones);
        }
    }

    for (player_character_entity, player_character_name) in &player_character_query {
        if player_character_name.0 == "SciFi Torso" {
            println!("dealing with: {}", player_character_name.0);
            let root_bone_option = find_child_with_name_containing(
                &all_child_entities,
                &names,
                &player_character_entity,
                "Root",
            );

            if let Some(root_bone) = root_bone_option {
                let mut part_bones = HashMap::new();
                collect_bones(&all_child_entities, &names, &root_bone, &mut part_bones);
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
                // walk_tree(&all_child_entities, &names, &root_bone, depth);
            }
        }
        // find_bones(
        //     &all_entities_with_children,
        //     &names,
        //     &armature,
        //     depth,
        //     &mut main_bones,
        //     &mut part_bones,
        // );
    }
    for skinned_entity in &skinned_entity_query {
        println!("skinned: {:?}", skinned_entity);
        // let parent_result = all_entities_with_parents.get(skinned_entity);
        // if let Ok(mesh_parent) = parent_result {
        //     let parent_entity = mesh_parent.get();
        //     let name_result = names.get(parent_entity);
        //     if let Ok(name) = name_result {
        //         println!("mesh parent name: {}", name);
        //     } else {
        //         println!("mesh parent is unnamed");
        //     }
        //     if let Ok(armature) = all_entities_with_parents.get(**mesh_parent) {
        //         let depth: u32 = 0;
        //         find_bones(
        //             &all_entities_with_children,
        //             &names,
        //             &armature,
        //             depth,
        //             &mut main_bones,
        //             &mut part_bones,
        //         );
        //         println!("all main bones: {:?}", main_bones);
        //         println!("all part bones: {:?}", part_bones)
        //     }
        // }
    }

    // for (number, part_bone) in part_bones {
    //     let mut entity_commands = commands.entity(part_bone);
    //     let new_parent_option = main_bones.get(&number);
    //     if let Some(new_parent) = new_parent_option {
    //         if let Ok(mut transform) = transforms.get_mut(part_bone) {
    //             transform.translation.x = 0.0;
    //             transform.translation.y = 0.0;
    //             transform.translation.z = 0.0;
    //         }
    //         entity_commands.set_parent(*new_parent);
    //     }
    // }
}

pub fn collect_bones(
    all_child_entities: &Query<&Children>,
    names: &Query<&Name>,
    root_bone: &Entity,
    collected: &mut HashMap<String, Entity>,
) {
    if let Ok(name) = names.get(*root_bone) {
        collected.insert(format!("{}", name), *root_bone);

        if let Ok(children) = all_child_entities.get(*root_bone) {
            for child in children {
                collect_bones(all_child_entities, names, child, collected)
            }
        }
    }
}

pub fn find_child_with_name_containing(
    all_child_entities: &Query<&Children>,
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

        let children_result = all_child_entities.get(*curr_entity);
        if let Ok(children) = children_result {
            for child in children {
                queue.push_back(child);
            }
        }
    }

    None
}
