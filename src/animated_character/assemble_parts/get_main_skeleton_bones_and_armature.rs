use crate::animated_character::{
    assemble_parts::{
        collect_bones::collect_bones,
        find_child_with_name_containing::find_child_with_name_containing,
    },
    spawn_scenes::SceneEntitiesByName,
};
use bevy::{prelude::*, utils::HashMap};

pub fn get_main_skeleton_bones_and_armature(
    scene_entities_by_name: Res<SceneEntitiesByName>,
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
) -> (HashMap<String, Entity>, Entity) {
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

        collect_bones(
            &all_entities_with_children,
            &names,
            &root_bone,
            &mut main_bones,
        );

        println!("main bones: {:#?}", main_bones);
    }

    (
        main_bones,
        main_armature_option.expect("the main skeleton to have an armature"),
    )
}
