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
    
    // Start with a dark background
    var final_color = vec3<f32>(0.1, 0.1, 0.2);
    
    // Simple time-based animation to verify uniforms are updating
    let time_color = sin(time) * 0.2 + 0.8;
    final_color *= time_color;
    
    // Test: Show a red circle in the center to verify shader is working
    let center_dist = distance(uv, vec2<f32>(0.5, 0.5));
    if center_dist < 0.1 {
        final_color = vec3<f32>(1.0, 0.0, 0.0); // Red circle
    }
    
    // Draw the dynamic circle if radius > 0
    if circle_radius > 0.0 {
        let circle_pos = vec2<f32>(circle_x, circle_y);
        let dist_to_circle = distance(uv, circle_pos);
        
        // Create gradient effect
        if dist_to_circle < circle_radius {
            let gradient = 1.0 - (dist_to_circle / circle_radius);
            gradient = smoothstep(0.0, 1.0, gradient);
            
            // Color based on time and position
            let hue = time * 0.2 + circle_x + circle_y;
            let r = sin(hue) * 0.5 + 0.5;
            let g = sin(hue + 2.094) * 0.5 + 0.5; // 2π/3
            let b = sin(hue + 4.188) * 0.5 + 0.5; // 4π/3
            
            let circle_color = vec3<f32>(r, g, b) * gradient;
            final_color = mix(final_color, circle_color, gradient * 0.8);
        }
        
        // Add glow effect
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
