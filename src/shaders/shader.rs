use bevy::{asset::RenderAssetUsages, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_resource::{AsBindGroup, ShaderRef}}, sprite::{Material2d, Material2dPlugin}};

use crate::GameState;

pub struct ShaderPlugin;


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub time: f32,
}

impl Material2d for CustomMaterial {
     fn fragment_shader() -> ShaderRef {
        "shaders/shader_1.frag.wgsl".into() // Use WGSL
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/shader_1.vert.wgsl".into()
    }
}

/// This plugin handles boid related stuff like movement
/// Boid logic is only active during the State `GameState::Playing`
impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(Material2dPlugin::<CustomMaterial>::default()) // Re-enabling custom material
        .add_systems(OnEnter(GameState::Playing), (setup_camera, spawn_custom_mesh).chain())
        .add_systems(Update, update_shader_time.run_if(in_state(GameState::Playing)));
    }
}

fn update_shader_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.time = time.elapsed_secs();
    }
}

fn setup_camera(mut commands: Commands, cameras: Query<Entity, With<Camera2d>>) {
    // Only spawn a camera if none exists
    if cameras.is_empty() {
        commands.spawn((Camera2d, Msaa::Off));
    }
}
fn make_quad_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(0.5, -0.5, 0.0),
        Vec3::new(0.5, 0.5, 0.0),
        Vec3::new(-0.5, 0.5, 0.0),
    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]);
    
    mesh.insert_indices(Indices::U32(vec![0, 1, 2, 2, 3, 0]));
    mesh
}

fn spawn_custom_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    println!("Spawning custom mesh!");
    let mesh = meshes.add(make_quad_mesh());
    let material = materials.add(CustomMaterial { time: 0.0 }); // Using custom material for gradient
    
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)) // Moved forward in Z
            .with_scale(Vec3::splat(100.0)), // Made it larger
        GlobalTransform::default(),
    ));
}