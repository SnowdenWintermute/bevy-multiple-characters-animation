use super::Animations;
use crate::asset_loader::AssetPack;
use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct PlayerCharacterName(pub String);

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum SpawnCharacterState {
    #[default]
    Loading,
    Spawned,
    Done,
}

#[derive(Resource)]
pub struct SceneEntitiesByName(pub HashMap<String, Entity>);

pub fn spawn_characters(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<SpawnCharacterState>>,
) {
    let mut scene_entities_by_name = HashMap::new();
    for (name, gltf_handle_loading_tracker) in &asset_pack.0 {
        println!("loading asset pack {name}");
        if let Some(gltf) = assets_gltf.get(&gltf_handle_loading_tracker.gltf_handle) {
            if name == "Sword" {
                let entity_commands = commands.spawn((
                    SceneBundle {
                        scene: gltf.named_scenes["Scene"].clone(),
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 0.0),
                            rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
                            scale: Vec3::splat(0.1),
                        },
                        ..Default::default()
                    },
                    PlayerCharacterName(name.clone()),
                ));

                let entity = entity_commands.id();
                scene_entities_by_name.insert(name.clone(), entity);
            } else {
                let entity_commands = commands.spawn((
                    SceneBundle {
                        scene: gltf.named_scenes["Scene"].clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    PlayerCharacterName(name.clone()),
                ));
                let entity = entity_commands.id();
                scene_entities_by_name.insert(name.clone(), entity);
            }

            let mut animations = HashMap::new();
            for named_animation in gltf.named_animations.iter() {
                println!("inserting named animation: {}", named_animation.0);
                animations.insert(
                    named_animation.0.clone(),
                    gltf.named_animations[named_animation.0].clone(),
                );
            }
            if animations.len() > 0 {
                commands.insert_resource(Animations(animations));
            }
        }
    }

    commands.insert_resource(SceneEntitiesByName(scene_entities_by_name));

    next_state.set(SpawnCharacterState::Spawned);
}
