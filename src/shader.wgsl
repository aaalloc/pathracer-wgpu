struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};


const MIN_T = 0.001f;
const MAX_T = 1000f;
const WIDTH = 900u;
const HEIGHT = 450u;
const SAMPLES_PER_PIXEL = 100u;

@group(0) @binding(0) var<uniform> camera: Camera;

@group(1) @binding(0) var<storage, read> spheres: array<Sphere>;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    // 1.0 to 0.0 to put in 2D space
    return VertexOutput(
        vec4<f32>(model.position, 0.0, 1.0),
        model.tex_coords,
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let u = in.tex_coords.x;
    let v = in.tex_coords.y;

    let origin = camera.eye;
    let direction = camera.lowerLeftCorner + u * camera.horizontal + v * camera.vertical - origin;

    let x = u32(u * f32(WIDTH));
    let y = u32(v * f32(HEIGHT));

    var rngState: u32 = initRng(
        vec2<u32>(u32(x), u32(y)), 
        vec2<u32>(WIDTH, HEIGHT), 
        0u
    );
    let color = sample_pixel(&rngState, f32(x), f32(y));
    
    return vec4<f32>(color, 1.0);

    // var noiseState: u32 = initRng(vec2<u32>(u32(u), u32(v)), vec2<u32>(512u, 512u), 0u);
    // return vec4<f32>(rngNextFloat(&noiseState), rngNextFloat(&noiseState), rngNextFloat(&noiseState), 1.0);
}

struct Camera {
    eye: vec3<f32>,
    horizontal: vec3<f32>,
    vertical: vec3<f32>,
    u: vec3<f32>,
    v: vec3<f32>,
    lensRadius: f32,
    lowerLeftCorner: vec3<f32>,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

struct Sphere {
    center: vec4<f32>,
    radius: f32,
    material_index: u32,
};

struct HitRecord {
    p: vec3<f32>,
    normal: vec3<f32>,
    t: f32,
    front_face: bool,
};


fn hit_sphere(
    sphere_index: u32,
    ray: Ray,
    ray_min: f32,
    ray_max: f32,
    hit: ptr<function, HitRecord>,
) -> bool {
    let sphere = spheres[sphere_index];

    let oc = ray.origin - sphere.center.xyz;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        return false;
    }

    let sqrtd = sqrt(discriminant);
    var root = (-b - sqrtd) / (2.0 * a);
    if root < ray_min || root > ray_max {
        root = (-b + sqrtd) / (2.0 * a);
        if root < ray_min || root > ray_max {
            return false;
        }
    }

    let t = root;
    let p = ray.origin + t * ray.direction;
    var normal = (p - sphere.center.xyz) / sphere.radius;
    let front_face = dot(ray.direction, normal) < 0.0;
    if !front_face {
        normal = -normal;
    }
    *hit = HitRecord(p, normal, t, front_face);
    return true;
}

fn check_intersection(ray: Ray, intersection: ptr<function, HitRecord>) -> bool{
    var closest_so_far = MAX_T;
    var  hit_anything = false;

    for (var i = 0u; i < arrayLength(&spheres); i = i + 1u) {
        var t = HitRecord();
        if hit_sphere(i, ray, MIN_T, MAX_T, &t) {
            hit_anything = true;
            closest_so_far = t.t;
            *intersection = t;
        }
    }

    return hit_anything;
}

fn sample_pixel(rngState: ptr<function, u32>, x: f32, y: f32) -> vec3<f32> {
    var color = vec3<f32>(0.0);
    for (var i = 0u; i < SAMPLES_PER_PIXEL; i = i + 1u) {
        let s = f32(x + rngNextFloat(rngState)) / f32(WIDTH);
        let t = f32(y + rngNextFloat(rngState)) / f32(HEIGHT);
        let ray = Ray(camera.eye, normalize(camera.lowerLeftCorner + s * camera.horizontal + t * camera.vertical - camera.eye));
        color += ray_color(ray);
    }
    return color / f32(SAMPLES_PER_PIXEL);
}

fn ray_color(ray: Ray) -> vec3<f32> {
    var color = vec3<f32>(0.0);
    var intersection = HitRecord();

    if check_intersection(ray, &intersection) {
        let N = normalize(intersection.normal);
        color += 0.5 * vec3<f32>(N.x + 1.0, N.y + 1.0, N.z + 1.0);
    } else {
        let a = 0.5 * (ray.direction.y + 1.0);
        color += (1.0 - a) * vec3<f32>(1.0, 1.0, 1.0) + a * vec3<f32>(0.5, 0.7, 1.0);
    }
    return color;

}


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
