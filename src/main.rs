use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
use pieces::*;

mod board;
use board::*;

fn setup(
    commands: &mut Commands,
) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_xyzw(-0.3, -0.5,-0.3,0.5).normalize(), Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
        })
        // Add the PickSource component to the camera to enable mesh 
        // mesh selection
        .with(PickSource::default())
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0,4.0)),
            ..Default::default()
        });
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4})
        .add_resource(WindowDescriptor {
            title: String::from("Chess!"),
            width: 1600.0,
            height: 1600.0,
            // This is struct update syntax, filling out the remainder 
            // of the struct with default values as provided by this struct
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(DebugPickingPlugin)
        // Startup systems are called only one, at startup
        // calling `system()` on a function turns it into a system
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .add_startup_system(create_pieces.system())
        .run();
}