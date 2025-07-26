use bevy::{asset::RenderAssetUsages, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_resource::{AsBindGroup, ShaderRef}}, sprite::{Material2d, Material2dPlugin}};

use crate::GameState;

const SHADER_ASSET_PATH: &str = "shaders/unified_animate.wgsl";
const MAX_CIRCLES: usize = 1; // Only 1 circle to match single Vec4 uniform

pub struct AnimatedShaderPlugin;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct AnimatedShaderMaterial {
    #[uniform(0)]
    data: Vec4, // x = time, y = circle_x, z = circle_y, w = circle_radius
}

#[derive(Component)]
struct LiquidCanvas;

#[derive(Resource, Default)]
struct CircleData {
    circles: Vec<LiquidCircle>,
}

#[derive(Clone)]
struct LiquidCircle {
    position: Vec2,
    creation_time: f32,
    max_radius: f32,
}

impl Material2d for AnimatedShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

/// This plugin handles liquid circle rendering with mouse interaction
impl Plugin for AnimatedShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<AnimatedShaderMaterial>::default())
           .init_resource::<CircleData>()
           .add_systems(OnEnter(GameState::Playing), spawn_animated_mesh)
           .add_systems(Update, (
               handle_mouse_clicks,
               update_liquid_circles,
               update_shader_uniforms,
           ).run_if(in_state(GameState::Playing)));
    }
}

fn handle_mouse_clicks(
    mut circle_data: ResMut<CircleData>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    // Auto-add circles for testing every 2 seconds
    if time.elapsed_secs() % 2.0 < 0.016 { // Roughly every 2 seconds (with frame tolerance)
        println!("� AUTO-ADDING CIRCLE for testing");
        let auto_pos = Vec2::new(
            0.3 + (time.elapsed_secs() * 0.5).sin() * 0.4,
            0.3 + (time.elapsed_secs() * 0.7).cos() * 0.4,
        );
        
        let new_circle = LiquidCircle {
            position: auto_pos,
            creation_time: time.elapsed_secs(),
            max_radius: 0.08,
        };
        
        circle_data.circles.push(new_circle);
        
        if circle_data.circles.len() > MAX_CIRCLES {
            circle_data.circles.remove(0);
        }
        
        println!("🟢 Auto-added circle at {:?}, total: {}", auto_pos, circle_data.circles.len());
    }
    
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("🖱️ MOUSE CLICKED!");
        let random_pos = Vec2::new(
            0.3 + (time.elapsed_secs() * 2.7).sin() * 0.4,
            0.3 + (time.elapsed_secs() * 1.9).cos() * 0.4,
        );
        
        let new_circle = LiquidCircle {
            position: random_pos,
            creation_time: time.elapsed_secs(),
            max_radius: 0.05 + (circle_data.circles.len() as f32 * 0.01) % 0.03,
        };
        
        circle_data.circles.push(new_circle);
        
        if circle_data.circles.len() > MAX_CIRCLES {
            circle_data.circles.remove(0);
        }
        
        println!("🟢 Added liquid circle at {:?}, total circles: {}", random_pos, circle_data.circles.len());
    }
}

fn update_liquid_circles(
    mut circle_data: ResMut<CircleData>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    // Remove circles that are too old (after 10 seconds)
    circle_data.circles.retain(|circle| current_time - circle.creation_time < 10.0);
}

fn update_shader_uniforms(
    time: Res<Time>,
    circle_data: Res<CircleData>,
    mut materials: ResMut<Assets<AnimatedShaderMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.data.x = time.elapsed_secs();
        
        // Update with the most recent circle (if any)
        if let Some(latest_circle) = circle_data.circles.last() {
            material.data.y = latest_circle.position.x;
            material.data.z = latest_circle.position.y;
            material.data.w = latest_circle.max_radius;
        } else {
            // No circles - clear circle data
            material.data.y = 0.0;
            material.data.z = 0.0;
            material.data.w = 0.0;
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
    println!("🔥 SPAWNING LIQUID CIRCLE CANVAS!");
    let mesh = meshes.add(make_quad_mesh());
    let material = materials.add(AnimatedShaderMaterial { 
        data: Vec4::new(0.0, 0.0, 0.0, 0.0), // time, circle_x, circle_y, circle_radius
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.4))
            .with_scale(Vec3::splat(800.0)), // Made larger and centered
        GlobalTransform::default(),
        LiquidCanvas,
    ));
    println!("✅ Liquid canvas spawned successfully!");
}
