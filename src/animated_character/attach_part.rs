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
    // let mut main_bones = HashMap::new();
    // let mut part_bones = HashMap::new();
    for (player_character_entity, player_character_name) in &player_character_query {
        let depth = 0;
        println!("found player character name: {}", player_character_name.0);
        // if player_character_name.0 == "main_skeleton" {
        if player_character_name.0 == "SciFi Torso" {
            println!("dealing with: {}", player_character_name.0);
            let armature = find_child_with_name_containing(
                &all_child_entities,
                &names,
                &player_character_entity,
                "Root",
            );

            // walk_tree(
            //     &all_child_entities,
            //     &names,
            //     &player_character_entity,
            //     depth,
            // );
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

// pub fn get_bones_from_scene(
//     all_child_entities: &Query<&Children>,
//     names: &Query<&Name>,
//     scene_root_entity: &Entity,
// ) {
// }

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

pub fn find_bones(
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    entity: &Entity,
    depth: u32,
    main_bones: &mut HashMap<u32, Entity>,
    part_bones: &mut HashMap<u32, Entity>,
) -> () {
    let mut padding = String::from("");
    for _ in 0..depth {
        padding.push_str("-")
    }
    if let Ok(name) = names.get(*entity) {
        println!("{padding}{:#?}({:?})", name, entity);
        let as_string = format!("{}", name);
        let split = as_string.split("-").collect::<Vec<&str>>();
        if split[0] == "main" {
            println!("found main bone number {}", split[2]);
            let bone_number = split[2].parse::<u32>().expect("");
            main_bones.insert(bone_number, *entity);
        } else if split[0] == "part" {
            println!("found part bone number {}", split[2]);
            let bone_number = split[2].parse::<u32>().expect("");
            part_bones.insert(bone_number, *entity);
        }
    } else {
        println!("{padding}unnamed entity: {:#?}", entity)
    }

    if let Ok(children_of_curr_node) = all_entities_with_children.get(*entity) {
        for child_entity in children_of_curr_node {
            find_bones(
                all_entities_with_children,
                names,
                child_entity,
                depth + 1,
                main_bones,
                part_bones,
            )
        }
    }
}
