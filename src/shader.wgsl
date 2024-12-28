struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_position: vec4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;


fn jenkinsHash(input: u32) -> u32 {
    var x = input;
    x += x << 10u;
    x ^= x >> 6u;
    x += x << 3u;
    x ^= x >> 11u;
    x += x << 15u;
    return x;
}

fn initRng(pixel: vec2<u32>, resolution: vec2<u32>, frame: u32) -> u32 {
    // Adapted from https://github.com/boksajak/referencePT
    let seed = dot(pixel, vec2<u32>(1u, resolution.x)) ^ jenkinsHash(frame);
    return jenkinsHash(seed);
}


fn rngNextInt(state: ptr<function, u32>) -> u32 {
    // PCG random number generator
    // Based on https://www.shadertoy.com/view/XlGcRh
    let newState = *state * 747796405u + 2891336453u;
    *state = newState;
    let word = ((newState >> ((newState >> 28u) + 4u)) ^ newState) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rngNextFloat(state: ptr<function, u32>) -> f32 {
    let x = rngNextInt(state);
    return f32(*state) / f32(0xffffffffu);
}


@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let resoltion = u32(512u);
    let scaleFactor = 1.0;
    let u = in.tex_coords.x * scaleFactor * f32(resoltion);
    let v = in.tex_coords.y * scaleFactor * f32(resoltion);

    var noiseState: u32 = initRng(
        vec2<u32>(u32(u), u32(v)), 
        vec2<u32>(resoltion, resoltion), 
        0u
    );
    
    return vec4<f32>(
        rngNextFloat(&noiseState), 
        rngNextFloat(&noiseState), 
        rngNextFloat(&noiseState),
        1.0
    );
}
