@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

@group(1) @binding(0)
var<uniform> settings: SettingsUniform;

@group(1) @binding(1)
var<storage, read> attractionTable: array<f32>;


struct SettingsUniform {
    deltaTime: f32,
    time: f32,
    invAspectRatio: f32,

    nTypes: u32,
    nParticles: u32,

    minR: f32,
    maxR: f32,
    friction: f32,
    speed: f32,
    flatForce: i32,
    wrap: i32,
// #ifdef SIXTEEN_BYTE_ALIGNMENT
//     _padding: vec3<f32>,
// #endif
}


struct Particle {
    pos: vec2<f32>,
    vel: vec2<f32>,
    color: vec3<f32>,
    typeIdx: u32,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

fn getAttractionFactor(sourceIdx: u32, targetIdx: u32) -> f32 {
    return attractionTable[sourceIdx * settings.nTypes + targetIdx];
}

fn attraction(dst: f32, a: f32) -> f32 {
    let r = dst / settings.maxR;

    if (r < settings.minR) {
        return r / settings.minR - 1.0;
    } else if (settings.minR < r && r < 1.0) {
        return a * (1.0 - abs(2.0 * r - 1.0 - settings.minR) / (1.0 - settings.minR));
    }
    return 0.0;
}


@compute @workgroup_size(64, 1, 1)
fn update(@builtin(global_invocation_id) id: vec3<u32>) {
    let p = particles[id.x];

    var accel = vec2<f32>(0.0);
    for (var i = 0u; i < settings.nParticles; i++) {
        let targetPart = particles[i];
        var dir = targetPart.pos - p.pos;

        if (settings.wrap == 1) {
            dir -= vec2(settings.invAspectRatio, 1.0) * round(dir / vec2(settings.invAspectRatio, 1.0));
        }

        let dst = length(dir);

        if (dst > 0.0 && dst < settings.maxR) {
            let normDir = dir / dst;

            let attractionFactor = getAttractionFactor(p.typeIdx, targetPart.typeIdx);
            let attractionAmount = attraction(dst, attractionFactor);

            accel += normDir * attractionAmount;
        }
    }
    accel *= settings.maxR * settings.speed;

    let newVel = settings.friction * p.vel + accel * settings.deltaTime;
    var newPos = p.pos + newVel * settings.deltaTime;

    if (settings.wrap == 1) {
        if (newPos.x >= settings.invAspectRatio || newPos.x < 0.0) {
            newPos.x = abs(newPos.x - settings.invAspectRatio);
        }
        if (newPos.y >= 1.0 || newPos.y < 0.0) {
            newPos.y = abs(newPos.y - 1.0);
        }
    }

    storageBarrier();
    particles[id.x].vel = newVel;
    particles[id.x].pos = newPos;
}