use bevy::{gltf::Gltf, prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum AssetLoaderState {
    #[default]
    Loading,
    Done,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoaderState>().add_loading_state(
            LoadingState::new(AssetLoaderState::Loading)
                .continue_to_state(AssetLoaderState::Done)
                .load_collection::<MyAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(
        paths(
            "main_skeleton.glb",
            "SciFi Torso.glb",
            "Witch Legs.glb",
            "Spacesuit Helmet.glb",
            "Sword Golden.glb"
        ),
        collection(typed, mapped)
    )]
    pub gltf_files: HashMap<String, Handle<Gltf>>,
    // #[asset(path = "main_skeleton.glb")]
    // pub main_skeleton: Handle<Gltf>,
    // #[asset(path = "SciFi Torso.glb")]
    // pub scifi_torso: Handle<Gltf>,
    // #[asset(path = "Witch Legs.glb")]
    // pub witch_legs: Handle<Gltf>,
    // #[asset(path = "Sword_Golden.glb")]
    // pub sword: Handle<Gltf>,
    #[asset(path = "FiraSans-Regular.ttf")]
    pub fira_sans_regular: Handle<Font>,
}
