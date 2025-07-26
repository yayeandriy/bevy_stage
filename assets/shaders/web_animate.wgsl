#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> material: WebShaderMaterial;

struct WebShaderMaterial {
    data: vec4<f32>, // x = time, y,z,w = padding
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time = material.data.x;
    let uv = mesh.uv;
    
    // Simple animated gradient that works reliably on web
    let center = vec2<f32>(0.5, 0.5);
    let distance_to_center = distance(uv, center);
    
    // Create animated colors using simple math
    let speed = 2.0;
    let r = sin(time * speed + distance_to_center * 8.0) * 0.5 + 0.5;
    let g = sin(time * speed + distance_to_center * 8.0 + 2.094) * 0.5 + 0.5; // 2π/3 offset
    let b = sin(time * speed + distance_to_center * 8.0 + 4.188) * 0.5 + 0.5; // 4π/3 offset
    
    // Add some wave animation
    let wave = sin(distance_to_center * 12.0 - time * 3.0) * 0.3 + 0.7;
    
    return vec4<f32>(r * wave, g * wave, b * wave, 1.0);
}
