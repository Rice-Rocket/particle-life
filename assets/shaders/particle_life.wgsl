@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

// @group(0) @binding(1)
// var<storage, read_write> particles: array<Particle>;

@group(1) @binding(0)
var<uniform> settings: SettingsUniform;


struct SettingsUniform {
    deltaTime: f32,
    time: f32,
    mapWidth: i32,
    mapHeight: i32,

    attract_mean: f32,
    attract_std: f32,
    min_r_lower: f32,
    min_r_upper: f32,
    max_r_lower: f32,
    max_r_upper: f32,
    friction: f32,
    speed: f32,
    flat_force: i32,
    wrap: i32,
// #ifdef SIXTEEN_BYTE_ALIGNMENT
//     _padding: vec3<f32>,
// #endif
}


struct Particle {
    pos: vec2<f32>,
    vel: vec2<f32>,
    typeIdx: f32,
    color: vec3<f32>,
}


@compute @workgroup_size(64, 1, 1)
fn update(@builtin(global_invocation_id) id: vec3<u32>) {
    let pos = particles[id.x].pos;
    var newPos = pos + vec2<f32>(0.0, 0.005);

    storageBarrier();
    particles[id.x].pos = newPos;
}