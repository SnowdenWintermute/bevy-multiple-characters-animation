mod alter_mesh_materials;
mod attach_part;
mod link_animations;
mod move_scene_transform;
mod paint_cubes_on_joints;
mod print_scene_tree;
mod remove_parts;
mod run_animations;
mod spawn_characters;
mod update_aabb_positions;
use self::move_scene_transform::move_scene_transform;
use self::remove_parts::remove_parts;
use self::spawn_characters::{SceneName, SpawnScenesState};
use self::update_aabb_positions::update_aabb_positions;
use crate::asset_loader::AssetLoaderState;
use bevy::{prelude::*, utils::HashMap};

pub struct AnimatedCharacterPlugin;
impl Plugin for AnimatedCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpawnScenesState>()
            .add_systems(
                OnEnter(AssetLoaderState::Done),
                spawn_characters::spawn_characters,
            )
            .add_systems(
                OnEnter(SpawnScenesState::Spawned),
                (
                    link_animations::link_animations,
                    // print_scene_tree::print_scene_tree,
                    alter_mesh_materials::alter_mesh_materials,
                    paint_cubes_on_joints::paint_cubes_on_joints,
                ),
            )
            .add_systems(
                OnEnter(SpawnScenesState::Done),
                run_animations::run_animations,
            )
            // .add_systems(Update, update_aabb_positions)
            .add_systems(OnEnter(SpawnScenesState::Done), attach_part::attach_part);
    }
}

#[derive(Resource, Debug)]
struct Animations(HashMap<String, Handle<AnimationClip>>);
