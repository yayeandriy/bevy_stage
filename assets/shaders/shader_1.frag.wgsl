struct FragmentInput {
    @location(0) local_pos: vec2<f32>,
};

@group(2) @binding(0)
var<uniform> material: CustomMaterial;

struct CustomMaterial {
    time: f32,
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    // Create a radial gradient from center with time-based animation
    let center = vec2<f32>(0.0, 0.0);
    let pos = input.local_pos;
    let distance_from_center = length(pos - center);
    
    // Animate the colors over time
    let time_factor = material.time * 0.5;
    
    // Create a colorful gradient based on position and distance with animation
    let r = sin(distance_from_center * 3.14159 * 2.0 + time_factor) * 0.5 + 0.5;
    let g = sin(pos.x * 3.14159 + time_factor + 1.0) * 0.5 + 0.5;
    let b = sin(pos.y * 3.14159 + time_factor + 2.0) * 0.5 + 0.5;
    
    // Add some animated wave effect
    let wave = sin(distance_from_center * 10.0 - time_factor * 3.0) * 0.1 + 0.9;
    
    return vec4<f32>(r * wave, g * wave, b * wave, 1.0);
}