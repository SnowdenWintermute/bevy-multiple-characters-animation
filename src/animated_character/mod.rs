mod link_animations;
use self::link_animations::{link_animations, AnimationEntityLink};
use crate::asset_loader::{AssetLoaderState, AssetPack};
use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

pub struct AnimatedCharacterPlugin;
impl Plugin for AnimatedCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoaderState::Done), spawn_characters)
            .add_systems(
                Update,
                (
                    run_animations.run_if(in_state(AssetLoaderState::Done)),
                    link_animations.run_if(in_state(AssetLoaderState::Done)),
                    list_scene_entities.run_if(in_state(AssetLoaderState::Done)),
                ),
            );
    }
}

#[derive(Resource, Debug)]
struct Animations(HashMap<String, Handle<AnimationClip>>);

#[derive(Component, Debug)]
struct PlayerCharacterName(String);

fn spawn_characters(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
        let scene_entity_1 = commands.spawn((
            SceneBundle {
                scene: gltf.named_scenes["Scene"].clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            PlayerCharacterName("Player 1".to_string()),
        ));
        println!("spawned entity: {:#?}", &scene_entity_1.id());

        let scene_entity_2 = commands.spawn((
            SceneBundle {
                scene: gltf.named_scenes["Scene"].clone(),
                transform: Transform::from_xyz(2.0, 0.0, 0.0),
                ..Default::default()
            },
            PlayerCharacterName("Player 2".to_string()),
        ));
        println!("spawned entity: {:#?}", &scene_entity_2.id());

        let mut animations = HashMap::new();
        animations.insert(
            String::from("Death"),
            gltf.named_animations["Death"].clone(),
        );

        animations.insert(String::from("Idle"), gltf.named_animations["Idle"].clone());

        commands.insert_resource(Animations(animations));
    }
}

fn run_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut player_character_query: Query<
        (&PlayerCharacterName, &AnimationEntityLink),
        Added<AnimationEntityLink>,
    >,
    animations: Res<Animations>,
) {
    for (name, animation_entity_link) in player_character_query.iter_mut() {
        if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity_link.0) {
            if name.0 == "Player 2".to_string() {
                animation_player
                    .play(animations.0["Death"].clone_weak())
                    .repeat();
            }
        }
    }
}

fn list_scene_entities(
    query: Query<(Entity, &PlayerCharacterName, &AnimationEntityLink), Added<AnimationEntityLink>>,
) {
    for (entity, character_name, animation_entity_link) in query.iter() {
        println!(
            "entity {:#?} has player_character_name of {} and player {:#?}",
            entity, character_name.0, animation_entity_link
        );
    }
}
