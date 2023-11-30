use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

const CAMERA_DISTANCE: f32 = 3.;
const CAMERA_LOOKING_AT: Vec3 = Vec3::new(0.0, 0., 2.0);

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, CAMERA_DISTANCE, 3.0)
                .looking_at(CAMERA_LOOKING_AT, Vec3::Z),
            ..default()
        },
        PanOrbitCamera {
            ..Default::default()
        },
        Name::new("Pan Orbit Camera"),
    ));
}
