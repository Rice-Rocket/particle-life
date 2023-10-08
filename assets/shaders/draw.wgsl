@group(0) @binding(1)
var<uniform> aspectRatio: f32;

struct VertexOutput {
    @location(0) color: vec3<f32>,
    @builtin(position) position: vec4<f32>,
}

@vertex
fn main_vs(
    @location(0) particlePos: vec2<f32>,
    @location(1) particle_vel: vec2<f32>,
    @location(2) particle_col: vec3<f32>,
    @location(3) particle_type: f32,
    @location(4) position: vec2<f32>,
) -> VertexOutput {
    let aspectMul = vec2<f32>(aspectRatio, 1.0);
    let screenPartPos = (particlePos * aspectMul * 2.0 - 1.0);
    return VertexOutput(particle_col, vec4<f32>((position * aspectMul) + screenPartPos, 0.0, 1.0));
}

@fragment
fn main_fs(vert: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vert.color.x, vert.color.y, vert.color.z, 1.0);
}