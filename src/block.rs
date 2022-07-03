use bevy::prelude::*;

pub fn spawn_block(
    commands: &mut Commands,
    material_handle: Handle<StandardMaterial>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    position: Vec3,
) {

    let mesh = create_block_mesh();

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: material_handle.clone(),
        transform: Transform::from_translation(position),
        ..default()
    });
}

fn create_block_mesh() -> Mesh {
    let mut uvs = Vec::new();
    //
    uvs.push([0.0 / 16.0, 1.0 / 16.0]);
    uvs.push([0.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 1.0 / 16.0]);

    //
    uvs.push([0.0 / 16.0, 1.0 / 16.0]);
    uvs.push([0.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 1.0 / 16.0]);

    //
    uvs.push([0.0 / 16.0, 1.0 / 16.0]);
    uvs.push([0.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 1.0 / 16.0]);

    //
    uvs.push([0.0 / 16.0, 1.0 / 16.0]);
    uvs.push([0.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 1.0 / 16.0]);

    // top
    uvs.push([0.0 / 16.0, 1.0 / 16.0]);
    uvs.push([0.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 1.0 / 16.0]);

    // bottom
    uvs.push([0.0 / 16.0, 1.0 / 16.0]);
    uvs.push([0.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 0.0 / 16.0]);
    uvs.push([1.0 / 16.0, 1.0 / 16.0]);

    let mut mesh = Mesh::from(shape::Cube::new(1.0));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}