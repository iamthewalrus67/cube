mod player;
mod block;

use bevy::{input::mouse::MouseMotion, prelude::*, pbr::wireframe::{WireframePlugin, WireframeConfig}};

use block::spawn_block;
use player::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Cube".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        // .add_plugin(WireframePlugin) // For debug
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system(cursor_grab_system)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    // cube
    let texture_handle = server.load("minecraft_atlas.png");
    let material_handle = materials.add(StandardMaterial { 
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for i in 0..10 {
        for j in 0..10 {
            spawn_block(&mut commands, material_handle.clone(), &mut meshes, Vec3::new(0.0 + j as f32, -1.0, -10.0 - i as f32));
        }
    }
    
    // light
    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, -8.0),
    //     ..default()
    // });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(0.0, -1.0, -1.0).normalize(), Vec3::Y),
        ..default()
    });
}
