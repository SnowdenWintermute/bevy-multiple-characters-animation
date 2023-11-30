use bevy::gltf::Gltf;
use bevy::prelude::*;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum AssetLoaderState {
    #[default]
    Loading,
    Done,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AssetLoaderState>()
            .add_systems(OnEnter(AssetLoaderState::Loading), load_assets)
            .add_systems(
                Update,
                check_for_load_complete.run_if(in_state(AssetLoaderState::Loading)),
            );
    }
}

#[derive(Resource, Debug)]
pub struct AssetPack(pub Handle<Gltf>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("loading assets...");
    let handle = asset_server.load("Cleric.gltf");
    commands.insert_resource(AssetPack(handle));
}

fn check_for_load_complete(
    asset_pack: Res<AssetPack>,
    mut next_state: ResMut<NextState<AssetLoaderState>>,
    mut asset_events: EventReader<AssetEvent<Gltf>>,
) {
    for event in asset_events.read() {
        debug!("seen {:?}", event);
        if event.is_loaded_with_dependencies(asset_pack.0.clone()) {
            info!("all assets loaded");
            println!("asset pack: {:#?}", asset_pack);
            next_state.set(AssetLoaderState::Done)
        }
    }
}
