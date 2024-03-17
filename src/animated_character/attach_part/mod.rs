mod collect_bones;
mod find_child_with_name_containing;
use self::collect_bones::collect_bones;
use self::find_child_with_name_containing::find_child_with_name_containing;
use super::spawn_characters::{SceneEntitiesByName, SceneName};
use bevy::{prelude::*, utils::HashMap};

pub fn attach_part(
    mut commands: Commands,
    scene_query: Query<(Entity, &SceneName), With<SceneName>>,
    all_entities_with_children: Query<&Children>,
    all_entities_with_parents: Query<&Parent>,
    mut transforms: Query<&mut Transform>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
    names: Query<&Name>,
) {
    let mut main_bones = HashMap::new();
    let mut main_armature_option = None;
    let main_skeleton_scene_entity_option = scene_entities_by_name.0.get("main_skeleton.glb");
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

    for (player_character_entity, player_character_name) in &scene_query {
        if player_character_name.0 == "Sword Golden.glb" {
            let mut entity_commands = commands.entity(player_character_entity);
            if let Some(hand_bone) = main_bones.get("EquipmentHandle.R") {
                entity_commands.set_parent(*hand_bone);
            }
        } else if player_character_name.0 == "SciFi Torso.glb"
            || player_character_name.0 == "Witch Legs.glb"
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
