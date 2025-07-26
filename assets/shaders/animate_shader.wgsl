struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(2) @binding(0)
var<uniform> material: CustomShaderMaterial;

struct CustomShaderMaterial {
    data: vec4<f32>, // x = time, y,z,w = padding
}

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(input.position, 1.0);
    out.uv = input.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let time = material.data.x;
    let uv = in.uv;
    
    // Simple animated gradient that should work on web
    let center = vec2<f32>(0.5, 0.5);
    let distance_to_center = distance(uv, center);
    
    // Create animated colors using simple math
    let r = sin(time * 2.0 + distance_to_center * 10.0) * 0.5 + 0.5;
    let g = sin(time * 2.0 + distance_to_center * 10.0 + 2.094) * 0.5 + 0.5; // 2.094 ≈ 2π/3
    let b = sin(time * 2.0 + distance_to_center * 10.0 + 4.188) * 0.5 + 0.5; // 4.188 ≈ 4π/3
    
    // Add some wave animation
    let wave = sin(distance_to_center * 15.0 - time * 3.0) * 0.2 + 0.8;
    
    return vec4<f32>(r * wave, g * wave, b * wave, 1.0);
}