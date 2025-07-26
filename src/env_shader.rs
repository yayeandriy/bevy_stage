use bevy::{asset::RenderAssetUsages, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_resource::{AsBindGroup, ShaderRef}}, sprite::{ColorMaterial, Material2d, Material2dPlugin}};

use crate::GameState;

const SHADER_ASSET_PATH: &str = "shaders/unified_animate.wgsl";

pub struct EnvShaderPlugin;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct AnimatedShaderMaterial {
    #[uniform(0)]
    data: Vec4, // x = time, y,z,w = padding for 16-byte alignment
}

impl Material2d for AnimatedShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

/// This plugin handles animated shader rendering
/// Works on both native and web platforms
impl Plugin for EnvShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<AnimatedShaderMaterial>::default())
           .add_systems(OnEnter(GameState::Playing), spawn_animated_mesh)
           .add_systems(Update, update_shader_animation.run_if(in_state(GameState::Playing)));
    }
}

fn update_shader_animation(
    time: Res<Time>,
    mut materials: ResMut<Assets<AnimatedShaderMaterial>>,
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

fn spawn_animated_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<AnimatedShaderMaterial>>,
) {
    println!("Spawning animated shader quad!");
    let mesh = meshes.add(make_quad_mesh());
    let material = materials.add(AnimatedShaderMaterial { data: Vec4::new(0.0, 0.0, 0.0, 0.0) });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(200.0, 200.0, 2.0))
            .with_scale(Vec3::splat(500.0)),
        GlobalTransform::default(),
    ));
}
