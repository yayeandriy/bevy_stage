use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};

pub struct MetaballsPlugin;

impl Plugin for MetaballsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<MetaballsMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, update_material);
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct MetaballsMaterial {
    #[uniform(0)]
    params: MetaballsParams,
}

#[derive(ShaderType, Clone, Copy)]
struct MetaballsParams {
    time: f32,
    _pad: f32,
    mouse: Vec2,
}

impl Default for MetaballsParams {
    fn default() -> Self {
        Self {
            time: 0.0,
            _pad: 0.0,
            mouse: Vec2::ZERO,
        }
    }
}

impl Material2d for MetaballsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/metaballs.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MetaballsMaterial>>,
    windows: Query<&Window>,
) {
    commands.spawn(Camera2d);
    let window = windows.single();
    let mesh = meshes.add(Rectangle::new(window.width(), window.height()));
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh.into(),
        material: materials.add(MetaballsMaterial {
            params: MetaballsParams::default(),
        }),
        ..default()
    });
}

fn update_material(
    time: Res<Time>,
    windows: Query<&Window>,
    mut materials: ResMut<Assets<MetaballsMaterial>>,
    query: Query<&Handle<MetaballsMaterial>>,
) {
    let window = windows.single();
    let cursor = window
        .cursor_position()
        .map(|p| p / Vec2::new(window.width(), window.height()))
        .unwrap_or(Vec2::new(0.5, 0.5));
    for handle in &query {
        if let Some(mat) = materials.get_mut(handle) {
            mat.params.time = time.elapsed_seconds();
            mat.params.mouse = cursor;
        }
    }
}

