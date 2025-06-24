#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Params {
    time: f32,
    _pad: f32,
    mouse: vec2<f32>,
}

@group(2) @binding(0)
var<uniform> params: Params;

fn metaball(pos: vec2<f32>, center: vec2<f32>, radius: f32) -> f32 {
    let diff = pos - center;
    return radius / (dot(diff, diff) + 0.0001);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = in.uv;
    let t = params.time;
    let mouse = params.mouse;

    let c1 = vec2<f32>(0.3 + 0.2 * sin(t), 0.6 + 0.2 * cos(t));
    let c2 = vec2<f32>(0.7 + 0.2 * sin(t * 1.3), 0.4 + 0.2 * cos(t * 1.3));
    let c3 = vec2<f32>(0.5 + 0.2 * sin(t * 0.7 + 1.0), 0.5 + 0.2 * cos(t * 0.7 + 1.0));

    let field =
        metaball(pos, mouse, 0.12) +
        metaball(pos, c1, 0.12) +
        metaball(pos, c2, 0.12) +
        metaball(pos, c3, 0.12);

    let intensity = smoothstep(1.0, 2.5, field);
    return vec4<f32>(vec3<f32>(intensity), 1.0);
}
