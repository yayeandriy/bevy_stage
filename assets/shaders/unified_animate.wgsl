#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> material: AnimatedShaderMaterial;

struct AnimatedShaderMaterial {
    data: vec4<f32>, // x = time, y = circle_x, z = circle_y, w = circle_radius
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time = material.data.x;
    let circle_x = material.data.y;
    let circle_y = material.data.z;
    let circle_radius = material.data.w;
    let uv = mesh.uv;
    
    // Start with a dark blue background
    var final_color = vec3<f32>(0.1, 0.1, 0.3);
    
    // Test: Show a red circle in the center to verify shader is working
    let center_dist = distance(uv, vec2<f32>(0.5, 0.5));
    if center_dist < 0.05 {
        final_color = vec3<f32>(1.0, 0.0, 0.0); // Small red circle in center
    }
    
    // Simple time-based animation to verify uniforms are updating
    let time_pulse = sin(time * 2.0) * 0.1 + 0.9;
    final_color *= time_pulse;
    
    // Draw the dynamic circle if it has a radius
    if circle_radius > 0.0 {
        let circle_pos = vec2<f32>(circle_x, circle_y);
        let dist_to_circle = distance(uv, circle_pos);
        
        if dist_to_circle < circle_radius {
            // Create a gradient from center to edge
            let gradient = 1.0 - (dist_to_circle / circle_radius);
            gradient = smoothstep(0.0, 1.0, gradient);
            
            // Animated colors
            let hue_shift = time * 0.5;
            let r = sin(hue_shift) * 0.5 + 0.5;
            let g = sin(hue_shift + 2.094) * 0.5 + 0.5; // 2π/3 offset
            let b = sin(hue_shift + 4.188) * 0.5 + 0.5; // 4π/3 offset
            
            let circle_color = vec3<f32>(r, g, b) * gradient;
            final_color = mix(final_color, circle_color, gradient);
        }
        
        // Add a soft glow around the circle
        let glow_radius = circle_radius * 1.5;
        if dist_to_circle < glow_radius && dist_to_circle >= circle_radius {
            let glow_factor = 1.0 - ((dist_to_circle - circle_radius) / (glow_radius - circle_radius));
            glow_factor = smoothstep(0.0, 1.0, glow_factor);
            
            let glow_color = vec3<f32>(0.2, 0.4, 0.8) * glow_factor * 0.3;
            final_color += glow_color;
        }
    }
    
    return vec4<f32>(final_color, 1.0);
}
