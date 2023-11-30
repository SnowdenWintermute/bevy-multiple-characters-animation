use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

use crate::asset_loader::{AssetLoaderState, AssetPack};

pub struct AnimatedCharacterPlugin;
impl Plugin for AnimatedCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoaderState::Done), spawn_characters)
            .add_systems(
                Update,
                run_animations.run_if(in_state(AssetLoaderState::Done)),
            );
    }
}

#[derive(Resource, Debug)]
struct Animations(HashMap<String, Handle<AnimationClip>>);

fn spawn_characters(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    println!("checking for gltf");
    if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
        commands.spawn(SceneBundle {
            scene: gltf.named_scenes["Scene"].clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
        let mut animations = HashMap::new();
        animations.insert(
            String::from("Death"),
            gltf.named_animations["Death"].clone(),
        );

        commands.insert_resource(Animations(animations));
    }
}

fn run_animations(
    mut animation_player_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
    animations: Res<Animations>,
) {
    for mut animation_player in &mut animation_player_query {
        animation_player
            .play(
                animations
                    .0
                    .get("Death")
                    .expect("named animation to be added")
                    .clone_weak(),
            )
            .repeat()
            .set_speed(0.5);
    }
}
