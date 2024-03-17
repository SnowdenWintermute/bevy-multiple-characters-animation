use std::borrow::Cow;
use super::spawn_characters::SceneName;
use bevy::prelude::*;

pub fn walk_tree(
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    entity: &Entity,
    depth: u32,
) -> () {
    let mut padding = String::from("");
    for _ in 0..depth {
        padding.push_str("-")
    }
    if let Ok(mut name) = names.get(*entity) {
        println!("{padding}{:#?}({:?})", name, entity);
    } else {
        println!("{padding}unnamed entity: {:#?}", entity)
    }

    if let Ok(children_of_curr_node) = all_entities_with_children.get(*entity) {
        for child_entity in children_of_curr_node {
            walk_tree(all_entities_with_children, names, child_entity, depth + 1 )
        }
    }
}

pub fn print_scene_tree(
    scene_query: Query<Entity, With<SceneName>>,
    children: Query<&Children>,
    names: Query<&Name>,
) {
    for scene_entity in &scene_query {
        let mut num_non_meshes_seen = 0;
        let mut num_meshes_seen = 0;

        walk_tree(&children,&names, &scene_entity, 0);
        // println!("num meshes seen: {num_meshes_seen}");
        // println!("num non meshes seen: {num_non_meshes_seen}");
    }
}
