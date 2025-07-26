use bevy::{asset::RenderAssetUsages, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_resource::{AsBindGroup, ShaderRef}}, sprite::{ColorMaterial, Material2d, Material2dPlugin}};

use crate::GameState;


const SHADER_ASSET_PATH: &str = "shaders/animate_shader.wgsl";

pub struct EnvShaderPlugin;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomShaderMaterial {
    #[uniform(0)]
    data: Vec4, // x = time, y,z,w = padding for 16-byte alignment
}

#[cfg(not(target_arch = "wasm32"))]
impl Material2d for CustomShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WebShaderMaterial {
    #[uniform(0)]
    data: Vec4, // x = time, y,z,w = padding for 16-byte alignment
}

#[cfg(target_arch = "wasm32")]
impl Material2d for WebShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/web_animate.wgsl".into()
    }
}

/// This plugin handles boid related stuff like movement
/// Boid logic is only active during the State `GameState::Playing`
impl Plugin for EnvShaderPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            app.add_plugins(Material2dPlugin::<CustomShaderMaterial>::default())
               .add_systems(OnEnter(GameState::Playing), spawn_custom_mesh)
               .add_systems(Update, update_shader_animation.run_if(in_state(GameState::Playing)));
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            app.add_plugins(Material2dPlugin::<WebShaderMaterial>::default())
               .add_systems(OnEnter(GameState::Playing), spawn_web_mesh)
               .add_systems(Update, update_web_shader_animation.run_if(in_state(GameState::Playing)));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn update_shader_animation(
    time: Res<Time>,
    mut materials: ResMut<Assets<CustomShaderMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.data.x = time.elapsed_secs();
    }
}

#[cfg(target_arch = "wasm32")]
fn update_web_shader_animation(
    time: Res<Time>,
    mut materials: ResMut<Assets<WebShaderMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.data.x = time.elapsed_secs();
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

#[cfg(not(target_arch = "wasm32"))]
fn spawn_custom_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomShaderMaterial>>,
) {
    println!("Spawning environment shader quad!");
    let mesh = meshes.add(make_quad_mesh());
    let material = materials.add(CustomShaderMaterial { data: Vec4::new(0.0, 0.0, 0.0, 0.0) });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
       Transform::from_translation(Vec3::new(200.0, 200.0, 2.0)) // Moved to a visible position and higher Z
            .with_scale(Vec3::splat(500.0)), // Made it smaller
        GlobalTransform::default(),
    ));
}

#[cfg(target_arch = "wasm32")]
fn spawn_web_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WebShaderMaterial>>,
) {
    println!("Spawning web animated shader quad!");
    let mesh = meshes.add(make_quad_mesh());
    let material = materials.add(WebShaderMaterial { data: Vec4::new(0.0, 0.0, 0.0, 0.0) });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(200.0, 200.0, 2.0))
            .with_scale(Vec3::splat(500.0)),
        GlobalTransform::default(),
    ));
}
