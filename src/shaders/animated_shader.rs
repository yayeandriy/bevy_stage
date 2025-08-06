use bevy::{
    asset::RenderAssetUsages, 
    input::{mouse::MouseButton, ButtonInput},
    prelude::*, 
    render::{mesh::{Indices, PrimitiveTopology}, render_resource::{AsBindGroup, ShaderRef}}, 
    sprite::{Material2d, Material2dPlugin},
    window::PrimaryWindow,
};

use crate::GameState;

const SHADER_ASSET_PATH: &str = "shaders/unified_animate.wgsl";

pub struct AnimatedShaderPlugin;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct AnimatedShaderMaterial {
    #[uniform(0)]
    data1: Vec4, // x = time, y = mouse_influence, z,w = mouse_pos
    #[uniform(1)]
    data2: Vec4, // x,y = resolution, z,w = padding
}

impl Material2d for AnimatedShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

#[derive(Component)]
struct AnimatedQuad;

/// This plugin handles animated shader rendering
/// Works on both native and web platforms
impl Plugin for AnimatedShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<AnimatedShaderMaterial>::default())
           .add_systems(OnEnter(GameState::MeshGrid), spawn_animated_mesh)
           .add_systems(Update, update_shader_animation.run_if(in_state(GameState::MeshGrid)));
    }
}

fn update_shader_animation(
    time: Res<Time>,
    mut materials: ResMut<Assets<AnimatedShaderMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    animated_query: Query<&Transform, With<AnimatedQuad>>,
) {
    if let Ok(window) = windows.single() {
        // Get mouse position in window coordinates
        let mouse_pos = window.cursor_position().unwrap_or(Vec2::ZERO);
        
        // Convert window coordinates to world coordinates
        let window_size = Vec2::new(window.width(), window.height());
        let world_pos = Vec2::new(
            mouse_pos.x - window_size.x / 2.0,
            -(mouse_pos.y - window_size.y / 2.0), // Flip Y and center
        );
        
        // Get the animated quad's transform to calculate relative mouse position
        if let Ok(quad_transform) = animated_query.single() {
            let quad_pos = quad_transform.translation.truncate();
            let quad_scale = quad_transform.scale.truncate();
            
            // Calculate relative position within the quad (-0.5 to 0.5 range)
            let relative_pos = (world_pos - quad_pos) / quad_scale;
            
            // Convert to UV coordinates (0.0 to 1.0)
            let normalized_mouse = Vec2::new(
                relative_pos.x + 0.5,
                -relative_pos.y + 0.5, // Flip Y for UV coordinates
            );
            
            // Calculate mouse influence based on movement and clicks
            let mouse_influence = if mouse_button_input.pressed(MouseButton::Left) {
                3.0 // Very strong influence when clicking
            } else {
                1.5 // Strong influence from movement only to make it visible
            };

            let resolution = Vec2::new(window.width(), window.height());

            for (_, material) in materials.iter_mut() {
                material.data1.x = time.elapsed_secs();
                material.data1.y = mouse_influence;
                material.data1.z = normalized_mouse.x;
                material.data1.w = normalized_mouse.y;
                material.data2.x = resolution.x;
                material.data2.y = resolution.y;
                material.data2.z = 0.0; // padding
                material.data2.w = 0.0; // padding
            }
        }
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
    println!("Spawning interactive animated shader quad!");
    let mesh = meshes.add(make_quad_mesh());
    let material = materials.add(AnimatedShaderMaterial { 
        data1: Vec4::new(0.0, 0.5, 0.5, 0.5), // time, mouse_influence, mouse_pos
        data2: Vec4::new(800.0, 600.0, 0.0, 0.0), // resolution + padding
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(200.0, 200.0, 2.0))
            .with_scale(Vec3::splat(500.0)),
        GlobalTransform::default(),
        AnimatedQuad,
    ));
}
