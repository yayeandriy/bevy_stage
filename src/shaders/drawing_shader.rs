use bevy::{
    asset::RenderAssetUsages,
    input::{mouse::MouseButton, ButtonInput},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin},
    window::PrimaryWindow,
};

use crate::GameState;

const DRAWING_SHADER_ASSET_PATH: &str = "shaders/drawing_shader.wgsl";

pub struct DrawingShaderPlugin;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DrawingShaderMaterial {
    #[uniform(0)]
    data1: Vec4, // x = time, y = mouse_pressed, z,w = mouse_pos
    #[uniform(1)]
    data2: Vec4, // x,y = resolution, z,w = padding
}

impl Material2d for DrawingShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        DRAWING_SHADER_ASSET_PATH.into()
    }
}

#[derive(Component)]
struct DrawingQuad;

/// This plugin handles interactive drawing shader rendering
/// Works on both native and web platforms
impl Plugin for DrawingShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DrawingShaderMaterial>::default())
            .add_systems(OnEnter(GameState::Playing), spawn_drawing_mesh)
            .add_systems(
                Update,
                update_drawing_shader.run_if(in_state(GameState::Playing)),
            );
    }
}

fn update_drawing_shader(
    time: Res<Time>,
    mut materials: ResMut<Assets<DrawingShaderMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    _drawing_query: Query<&Transform, With<DrawingQuad>>,
) {
    if let Ok(window) = windows.single() {
        // Get mouse position in window coordinates
        let mouse_pos = window.cursor_position().unwrap_or(Vec2::ZERO);
        
        // Convert to normalized coordinates (0.0 to 1.0)
        let normalized_mouse = Vec2::new(
            mouse_pos.x / window.width(),
            1.0 - (mouse_pos.y / window.height()), // Flip Y coordinate
        );

        let mouse_pressed = if mouse_button_input.pressed(MouseButton::Left) {
            1.0
        } else {
            0.0
        };

        let resolution = Vec2::new(window.width(), window.height());

        for (_, material) in materials.iter_mut() {
            material.data1.x = time.elapsed_secs();
            material.data1.y = mouse_pressed;
            material.data1.z = normalized_mouse.x;
            material.data1.w = normalized_mouse.y;
            material.data2.x = resolution.x;
            material.data2.y = resolution.y;
            material.data2.z = 0.0; // padding
            material.data2.w = 0.0; // padding
        }
    }
}

fn make_drawing_quad_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::new(0.5, 0.5, 0.0),
            Vec3::new(-0.5, 0.5, 0.0),
        ],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ],
    );

    mesh.insert_indices(Indices::U32(vec![0, 1, 2, 2, 3, 0]));
    mesh
}

fn spawn_drawing_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DrawingShaderMaterial>>,
) {
    println!("Spawning interactive drawing shader quad!");
    let mesh = meshes.add(make_drawing_quad_mesh());
    let material = materials.add(DrawingShaderMaterial {
        data1: Vec4::new(0.0, 0.0, 0.0, 0.0), // time, mouse_pressed, mouse_pos
        data2: Vec4::new(800.0, 600.0, 0.0, 0.0), // resolution + padding
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(-200.0, 200.0, 2.0))
            .with_scale(Vec3::splat(500.0)),
        GlobalTransform::default(),
        DrawingQuad,
    ));
}
