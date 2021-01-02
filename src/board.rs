use bevy::prelude::*;
use bevy_mod_picking::*;

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

pub fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materiales
    // meshes.add() returns a Handle<Mesh>
    let mesh = meshes.add(Mesh::from(shape::Plane {size:1.0}));
    // Spawn 64 squares
    for i in 0..8{
        for j in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                // Alternate material
                material: if (i + j + 1) % 2 == 0 {
                    materials.add(Color::rgb(1.0,0.9,0.9).into())
                } else {
                    materials.add(Color::rgb(0.0,0.1,0.1).into())
                },
                transform: Transform::from_translation( Vec3::new(i as f32, 0.0, j as f32)),
                ..Default::default()
            })
            // Add component to make this mesh pickable
            .with(PickableMesh::default())
            .with(Square {
                x: i,
                y: j,
            });
        }
    }
}