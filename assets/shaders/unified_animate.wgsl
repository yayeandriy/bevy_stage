#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> material: AnimatedShaderMaterial;

struct AnimatedShaderMaterial {
    data1: vec4<f32>, // x = time, y = mouse_influence, z,w = mouse_pos
    data2: vec4<f32>, // x,y = resolution, z,w = padding
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time = material.data1.x;
    let mouse_influence = material.data1.y;
    let mouse_pos = vec2<f32>(material.data1.z, material.data1.w);
    let resolution = vec2<f32>(material.data2.x, material.data2.y);
    let uv = mesh.uv;
    
    // Make the center directly follow the mouse position
    let center = mouse_pos; // Direct mouse following
    let distance_to_center = distance(uv, center);
    let distance_to_mouse = distance(uv, mouse_pos);
    
    // Create concentric circles from mouse position
    let circle_frequency = 15.0; // Number of circles
    let circle_speed = 3.0;
    let circles = sin(distance_to_center * circle_frequency - time * circle_speed) * 0.5 + 0.5;
    
    // Mouse influence affects intensity and colors
    let intensity = 1.0 + mouse_influence * 2.0;
    
    // Create animated colors that radiate from mouse position
    let angle = atan2(uv.y - center.y, uv.x - center.x);
    let radial_pattern = sin(angle * 6.0 + time * 2.0) * 0.5 + 0.5;
    
    // Color based on distance from mouse and time
    let r = sin(distance_to_center * 8.0 - time * 2.0) * 0.4 + 0.6;
    let g = sin(distance_to_center * 8.0 - time * 2.0 + 2.094) * 0.4 + 0.6; // 2π/3 offset
    let b = sin(distance_to_center * 8.0 - time * 2.0 + 4.188) * 0.4 + 0.6; // 4π/3 offset
    
    // Combine circular pattern with radial pattern
    let pattern_strength = circles * radial_pattern * intensity;
    
    // Add a bright center point at mouse position
    let center_glow = 1.0 / (1.0 + distance_to_center * 20.0);
    
    // BASE COLOR: Original animated pattern
    var final_color = vec3<f32>(r, g, b) * pattern_strength + vec3<f32>(1.0, 0.8, 0.6) * center_glow;
    
    // BLACK CIRCLE when mouse is clicked (high influence)
    if (mouse_influence > 2.5) { // When clicking
        let click_circle_size = 0.15; // Larger black circle
        if (distance_to_mouse < click_circle_size) {
            // Create a black circle with soft edges
            let edge_softness = 0.03;
            let circle_alpha = 1.0 - smoothstep(click_circle_size - edge_softness, click_circle_size, distance_to_mouse);
            final_color = mix(final_color, vec3<f32>(0.0, 0.0, 0.0), circle_alpha);
        }
    }
    
    // WHITE CIRCLE indicator when just hovering (lower influence)
    if (mouse_influence > 1.0 && mouse_influence <= 2.5) {
        let hover_circle_size = 0.10; // Larger circle for hovering
        let ring_thickness = 0.02;
        let ring_distance = abs(distance_to_mouse - hover_circle_size);
        if (ring_distance < ring_thickness) {
            let ring_alpha = 1.0 - (ring_distance / ring_thickness);
            final_color = mix(final_color, vec3<f32>(1.0, 1.0, 1.0), ring_alpha);
        }
    }
    
    return vec4<f32>(final_color, 1.0);
}
