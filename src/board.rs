use bevy::prelude::*;
use bevy_mod_picking::*;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system());
    }
}

fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Query provides me with an iterable over all entities
    // that have these components, in this case 
    // - Entity(which every entity as)
    // - Square
    // - Handle<StandardMaterial>
    // These components need to be references
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    // Get entity under the cursor if there is one
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, material_handle) in query.iter() {
        // Get the actual material
        let material = materials.get_mut(material_handle).unwrap();

        // Change the material color
        material.albedo = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1.0, 0.9, 0.9)
        } else {
            Color::rgb(0.0, 0.1, 0.1)
        };
    }
}

// Resource to keep track of which sqaure is currently selected
// - Default means 'entity' will be None when it is initialized
#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

fn create_board(
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