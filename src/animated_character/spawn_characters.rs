use super::Animations;
use crate::asset_loader::MyAssets;
use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct SceneName(pub String);

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum SpawnScenesState {
    #[default]
    Loading,
    Spawned,
    Done,
}

#[derive(Resource)]
pub struct SceneEntitiesByName(pub HashMap<String, Entity>);

pub fn spawn_characters(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<SpawnScenesState>>,
) {
    // SET UP RESORCES
    let mut scene_entities_by_name = HashMap::new();
    let mut animations = HashMap::new();
    // SPAWN SCENES
    for (name, gltf_handle) in &asset_pack.gltf_files {
        if let Some(gltf) = assets_gltf.get(&gltf_handle.clone()) {
            let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
            if name == "Sword Golden.glb" {
                transform.scale = Vec3::splat(0.1);
            }

            let entity_commands = commands.spawn((
                SceneBundle {
                    scene: gltf.named_scenes["Scene"].clone(),
                    transform,
                    ..Default::default()
                },
                SceneName(name.clone()),
            ));

            let entity = entity_commands.id();
            scene_entities_by_name.insert(name.clone(), entity);

            // EXTRACT ANIMATIONS TO RESOURCE
            for named_animation in gltf.named_animations.iter() {
                println!("inserting named animation: {}", named_animation.0);
                animations.insert(
                    named_animation.0.clone(),
                    gltf.named_animations[named_animation.0].clone(),
                );
            }
        }
    }

    commands.insert_resource(Animations(animations));
    commands.insert_resource(SceneEntitiesByName(scene_entities_by_name));

    next_state.set(SpawnScenesState::Spawned);
}
