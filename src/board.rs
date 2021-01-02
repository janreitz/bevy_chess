use bevy::prelude::*;
use bevy_mod_picking::*;

pub fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materiales
    // meshes.add() returns a Handle<Mesh>
    let mesh = meshes.add(Mesh::from(shape::Plane {size:1.0}));
    let white_material = materials.add(Color::rgb(1.0,0.9,0.9).into());
    let black_material = materials.add(Color::rgb(0.0,0.1,0.1).into());

    // Spawn 64 squares
    for i in 0..8{
        for j in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                // Alternate material
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation( Vec3::new(i as f32, 0.0, j as f32)),
                ..Default::default()
            })
            // Add component to make this mesh pickable
            .with(PickableMesh::default());
        }
    }
}