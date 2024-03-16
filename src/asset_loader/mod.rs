use bevy::{gltf::Gltf, prelude::*};
use std::collections::BTreeMap;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum AssetLoaderState {
    #[default]
    Loading,
    Done,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoaderState>()
            .add_systems(OnEnter(AssetLoaderState::Loading), load_assets)
            .add_systems(
                Update,
                check_for_load_complete.run_if(in_state(AssetLoaderState::Loading)),
            );
    }
}

#[derive(Debug)]
pub struct GltfHandleLoadingTracker {
    pub gltf_handle: Handle<Gltf>,
    pub is_loaded: bool,
}

#[derive(Resource, Debug)]
pub struct AssetPack(pub BTreeMap<String, GltfHandleLoadingTracker>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut gltf_handles = BTreeMap::new();

    let handle_0 = asset_server.load("main_skeleton.glb");
    gltf_handles.insert(
        String::from("main_skeleton"),
        GltfHandleLoadingTracker {
            gltf_handle: handle_0.clone(),
            is_loaded: false,
        },
    );

    let sci_fi_torso_handle = asset_server.load("SciFi Torso.glb");
    gltf_handles.insert(
        String::from("SciFi Torso"),
        GltfHandleLoadingTracker {
            gltf_handle: sci_fi_torso_handle.clone(),
            is_loaded: false,
        },
    );

    let witch_legs_handle = asset_server.load("Witch Legs.glb");
    gltf_handles.insert(
        String::from("Witch Legs"),
        GltfHandleLoadingTracker {
            gltf_handle: witch_legs_handle.clone(),
            is_loaded: false,
        },
    );

    let sword_handle = asset_server.load("Sword_Golden.glb");
    gltf_handles.insert(
        String::from("Sword"),
        GltfHandleLoadingTracker {
            gltf_handle: sword_handle.clone(),
            is_loaded: false,
        },
    );
    // let handle_1 = asset_server.load("Casual.gltf");
    // gltf_handles.insert(
    //     String::from("Casual"),
    //     GltfHandleLoadingTracker {
    //         gltf_handle: handle_1.clone(),
    //         is_loaded: false,
    //     },
    // );

    // let handle_2 = asset_server.load("Adventurer.gltf");
    // gltf_handles.insert(
    //     String::from("Adventurer"),
    //     GltfHandleLoadingTracker {
    //         gltf_handle: handle_2.clone(),
    //         is_loaded: false,
    //     },
    // );

    commands.insert_resource(AssetPack(gltf_handles));
    // let handle = asset_server.load("Adventurer.gltf");
    // commands.insert_resource(AssetPack(handle));
}

fn check_for_load_complete(
    mut asset_pack: ResMut<AssetPack>,
    mut next_state: ResMut<NextState<AssetLoaderState>>,
    mut asset_events: EventReader<AssetEvent<Gltf>>,
) {
    for event in asset_events.read() {
        println!(" event : {:#?}", event);
        for (name, gltf_handle_loading_tracker) in asset_pack.0.iter_mut() {
            if event.is_loaded_with_dependencies(&gltf_handle_loading_tracker.gltf_handle) {
                println!("loaded  : {name}");
                gltf_handle_loading_tracker.is_loaded = true;
            }
        }
        // println!("{:#?}", &gltf_handle_loading_tracker.gltf_handle);
        // if event.is_loaded_with_dependencies(&gltf_handle_loading_tracker.gltf_handle) {
        //     gltf_handle_loading_tracker.is_loaded = true
        // }
    }

    for (name, gltf_handle_loading_tracker) in asset_pack.0.iter_mut() {
        println!(
            "{name} is loaded : {}",
            gltf_handle_loading_tracker.is_loaded
        );
        if !gltf_handle_loading_tracker.is_loaded {
            return;
        }
    }
    next_state.set(AssetLoaderState::Done)
}
