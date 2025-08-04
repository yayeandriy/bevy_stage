#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> material: DrawingShaderMaterial;

struct DrawingShaderMaterial {
    data1: vec4<f32>, // x = time, y = mouse_pressed, z,w = mouse_pos
    data2: vec4<f32>, // x,y = resolution, z,w = padding
}

// Simple hash function for pseudo-random values
fn hash(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}

// Smooth noise function
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    
    let u = f * f * (3.0 - 2.0 * f);
    
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

// Distance field for drawing trails
fn paintBrush(uv: vec2<f32>, mouse_pos: vec2<f32>, radius: f32) -> f32 {
    let dist = distance(uv, mouse_pos);
    return smoothstep(radius, radius * 0.5, dist);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let time = material.data1.x;
    let mouse_pressed = material.data1.y;
    let mouse_pos = vec2<f32>(material.data1.z, material.data1.w);
    let resolution = vec2<f32>(material.data2.x, material.data2.y);
    
    // Background pattern
    let background_noise = noise(uv * 10.0 + time * 0.5) * 0.1;
    let grid = abs(fract(uv * 20.0) - 0.5);
    let grid_pattern = min(grid.x, grid.y) * 2.0;
    let background = vec3<f32>(0.05 + background_noise, 0.1 + background_noise * 0.5, 0.15 + grid_pattern * 0.1);
    
    // Drawing brush effect
    let brush_size = 0.05;
    let brush_intensity = paintBrush(uv, mouse_pos, brush_size);
    
    // Create ripple effect when mouse is pressed
    var paint_color = vec3<f32>(0.0);
    if (mouse_pressed > 0.5) {
        // Current brush stroke
        let current_brush = brush_intensity;
        
        // Ripple effect from mouse position
        let dist_to_mouse = distance(uv, mouse_pos);
        let ripple = sin(dist_to_mouse * 30.0 - time * 10.0) * 0.5 + 0.5;
        let ripple_falloff = smoothstep(0.3, 0.0, dist_to_mouse);
        
        // Color based on position and time
        let hue_shift = sin(time * 2.0 + dot(uv, vec2<f32>(5.0, 3.0))) * 0.5 + 0.5;
        paint_color = vec3<f32>(
            0.8 + sin(hue_shift * 6.28) * 0.2,
            0.6 + sin(hue_shift * 6.28 + 2.09) * 0.4,
            0.9 + sin(hue_shift * 6.28 + 4.18) * 0.1
        );
        
        // Combine brush and ripple effects
        let total_effect = current_brush + ripple * ripple_falloff * 0.3;
        paint_color *= total_effect;
    } else {
        // When not pressed, show a subtle cursor indicator
        let cursor_ring = smoothstep(brush_size + 0.01, brush_size, distance(uv, mouse_pos)) 
                         - smoothstep(brush_size - 0.01, brush_size - 0.02, distance(uv, mouse_pos));
        paint_color = vec3<f32>(1.0, 1.0, 1.0) * cursor_ring * 0.5;
    }
    
    // Persistent drawing effect (this is a simple approximation)
    // In a real implementation, you'd want to use a texture to store the drawing
    let persistent_drawing = smoothstep(0.02, 0.0, abs(sin(uv.x * 50.0 + time) * sin(uv.y * 30.0 + time * 0.7)) - 0.8) * 0.2;
    let persistent_color = vec3<f32>(0.3, 0.7, 1.0) * persistent_drawing;
    
    // Combine all effects
    let final_color = background + paint_color + persistent_color;
    
    return vec4<f32>(final_color, 1.0);
}
