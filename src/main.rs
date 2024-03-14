mod animated_character;
mod asset_loader;
mod camera;
mod level;
use animated_character::AnimatedCharacterPlugin;
use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use camera::CameraPlugin;
// use level::PlanePlugin;

fn main() {
    App::new()
        // BEVY
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.95,
        })
        .add_plugins(DefaultPlugins)
        // SELF MADE
        // .add_plugins(PlanePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(AnimatedCharacterPlugin)
        // EXTERNAL DEPENDENCIES
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(BillboardPlugin)
        .run();
}
