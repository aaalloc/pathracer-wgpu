struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

const EPSILON = 0.0001f;
const PI = 3.1415927f;
const FRAC_1_PI = 0.31830987f;
const FRAC_PI_2 = 1.5707964f;

const MIN_T = 0.001f;
const MAX_T = 1000f;

@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<uniform> frame_data: Frame;
@group(0) @binding(2) var<uniform> render_param: RenderParam;
@group(0) @binding(3) var<storage, read_write> image_buffer: array<array<f32, 3>>;


@group(1) @binding(0) var<storage, read> spheres: array<Sphere>;
@group(1) @binding(1) var<storage, read> materials: array<Material>;
@group(1) @binding(2) var<storage, read> textures: array<array<f32, 3>>;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    return VertexOutput(
        vec4<f32>(model.position, 0.0, 1.0),
        model.tex_coords,
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let u = in.tex_coords.x;
    let v = in.tex_coords.y;

    let x = u32(u * f32(frame_data.width));
    let y = u32(v * f32(frame_data.height));
    let i = y * frame_data.width + x;

    var rngState: u32 = init_rng(
        vec2<u32>(u32(x), u32(y)), 
        vec2<u32>(frame_data.width, frame_data.height),
        frame_data.frame_idx
    );

    var pixel = vec3(image_buffer[i][0], image_buffer[i][1], image_buffer[i][2]);

    let rgb = sample_pixel(&rngState, f32(x), f32(y));
    pixel += rgb;

    image_buffer[i] = array<f32, 3>(pixel.r, pixel.g, pixel.b);


    return vec4<f32>(
        pixel / f32(render_param.total_samples),
        1.0
    );

    // var noiseState: u32 = init_rng(vec2<u32>(u32(u), u32(v)), vec2<u32>(512u, 512u), 0u);
    // return vec4<f32>(rng_next_float(&rngState), rng_next_float(&rngState), rng_next_float(&rngState), 1.0);
}

struct RenderParam {
    samples_max_per_pixel: u32,
    samples_per_pixel: u32,
    total_samples: u32,
    max_depth: u32,
};

struct Frame {
    width: u32,
    height: u32,
    frame_idx: u32,
};

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

const MAT_LAMBERTIAN = 0u;
const MAT_METAL = 1u;
const MAT_DIELECTRIC = 2u;

struct Material {
    id: u32,
    desc: TextureDescriptor,
    fuzz: f32,
};

struct TextureDescriptor {
    width: u32,
    height: u32,
    offset: u32,
}

struct HitRecord {
    p: vec3<f32>,
    normal: vec3<f32>,
    t: f32,
    material_index: u32,
    front_face: bool,
};


struct Scatter {
    scattered: Ray,
    attenuation: vec3<f32>,
}

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
    let b = dot(ray.direction, oc);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - a * c;
    
    if discriminant < 0.0 {
        return false;
    }

    let sqrtd = sqrt(discriminant);

    var root = (-b - sqrtd) / a;
    if root < ray_min || root > ray_max 
    {
        root = (-b + sqrtd) / a;
        if root < ray_min || root > ray_max {
            return false;
        }
    }


    *hit = sphereIntersection(ray, sphere, root, sphere_index);
    return true;
}

fn sphereIntersection(ray: Ray, sphere: Sphere, t: f32, material_index: u32) -> HitRecord {
    let p = ray.origin + t * ray.direction;
    var normal = (p - sphere.center.xyz) / sphere.radius;
    var front_face = true;
    if dot(ray.direction, normal) > 0.0 {
        normal = -normal;
        front_face = false;
    }
    return HitRecord(p, normal, t, material_index, front_face);
}


fn check_intersection(ray: Ray, intersection: ptr<function, HitRecord>) -> bool {
    var closest_so_far = MAX_T;
    var hit_anything = false;
    var tmp_rec = HitRecord();

    for (var i = 0u; i < arrayLength(&spheres); i += 1u) 
    {
        if hit_sphere(i, ray, MIN_T, MAX_T, &tmp_rec) 
        {
            hit_anything = true;
            closest_so_far = tmp_rec.t;
            *intersection = tmp_rec;
        }
    }

    return hit_anything;
}

fn sample_pixel(rngState: ptr<function, u32>, x: f32, y: f32) -> vec3<f32> {
    var color = vec3(0.0);
    for (var i = 0u; i < render_param.samples_per_pixel; i += 1u) 
    {
        let ray = get_ray(rngState, x, y);
        color += ray_color(ray, rngState);
    }
    return color;
}

fn get_ray(rngState: ptr<function, u32>, x: f32, y: f32) -> Ray {
    let u = f32(x + rng_next_float(rngState)) / f32(frame_data.width);
    let v = f32(y + rng_next_float(rngState)) / f32(frame_data.height);

    let origin = camera.eye;
    let direction = camera.lowerLeftCorner + u * camera.horizontal + v * camera.vertical - origin;

    return Ray(origin, direction);
}



fn ray_color(first_ray: Ray, rngState: ptr<function, u32>) -> vec3<f32> {
    var ray = first_ray;
    var sky_color = vec3(0.0);
    var color = vec3(1.0);
    var intersection = HitRecord();

    for (var i = 0u; i < render_param.max_depth; i += 1u)
    {
        if check_intersection(ray, &intersection)
        {
            let material = materials[intersection.material_index];
            let scattered = scatter(ray, intersection, material, rngState);
            color *= scattered.attenuation;
            ray = scattered.scattered;
        } 
        else 
        {
            let direction = normalize(ray.direction);
            let a = 0.5 * (direction.y + 1.0);
            var sky = (1.0 - a) * vec3<f32>(1.0, 1.0, 1.0) + a * vec3<f32>(0.5, 0.7, 1);
            sky_color = sky;
            // break;
        }
    }
    return color * sky_color;
}



fn scatter(ray: Ray, hit: HitRecord, material: Material, rngState: ptr<function, u32>) -> Scatter {
    switch (material.id) 
    {
        case MAT_LAMBERTIAN: 
        {
            let t = hit.p + hit.normal + random_on_hemisphere(rngState, hit.normal);
            
            if vec3_near_zero(t - hit.p) {
                return Scatter(Ray(hit.p, hit.normal), texture_look_up(material.desc, 0.5, 0.5));
            }

            return Scatter(Ray(hit.p, t - hit.p), texture_look_up(material.desc, 0.5, 0.5));
        }
        case MAT_METAL: 
        {
            let reflected = reflect(normalize(ray.direction), hit.normal);
            let fuzz = material.fuzz;
            let direction = reflected + fuzz * random_in_unit_sphere(rngState);
            return Scatter(Ray(hit.p, direction), texture_look_up(material.desc, 0.5, 0.5));
        }
        case MAT_DIELECTRIC: 
        {
            var ri: f32 = material.fuzz;
            if hit.front_face {
                ri = 1.0 / material.fuzz;
            }

            let unit_direction = normalize(ray.direction);
            let cos_theta = min(dot(-unit_direction, hit.normal), 1.0);
            let sin_theta = sqrt(1.0 - cos_theta * cos_theta);

            var direction = vec3(0.0);
            let rnd_float = rng_next_float(rngState);
            if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > rnd_float {
                direction = reflect(unit_direction, hit.normal);
            } else {
                direction = refract(unit_direction, hit.normal, ri);
            }


            return Scatter(Ray(hit.p, direction), vec3(1.0));
        }
        default: {
            return Scatter(Ray(vec3(0.0), vec3(0.0)), vec3(0.0));
        }
    }
}

fn vec3_near_zero(v: vec3<f32>) -> bool {
    let s = 1e-8;
    return (abs(v.x) < s) && (abs(v.y) < s) && (abs(v.z) < s);
}

fn reflect(v: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    return v - 2.0 * dot(v, n) * n;
}

fn refract(uv: vec3<f32>, n: vec3<f32>, etai_over_etat: f32) -> vec3<f32> {
    let cos_theta = dot(-uv, n);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -sqrt(abs(1.0 - length(r_out_perp) * length(r_out_perp))) * n;
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    var r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * pow(1.0 - cosine, 5.0);
}


fn jenkin_hash(input: u32) -> u32 {
    var x = input;
    x += x << 10u;
    x ^= x >> 6u;
    x += x << 3u;
    x ^= x >> 11u;
    x += x << 15u;
    return x;
}

fn init_rng(pixel: vec2<u32>, resolution: vec2<u32>, frame: u32) -> u32 {
    // Adapted from https://github.com/boksajak/referencePT
    let seed = dot(pixel, vec2<u32>(1u, resolution.x)) ^ jenkin_hash(frame);
    return jenkin_hash(seed);
}


fn random_on_hemisphere(rngState: ptr<function, u32>, normal: vec3<f32>) -> vec3<f32> {
    let on_unit_sphere = random_in_unit_sphere(rngState);
    if dot(on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}


fn random_in_unit_sphere(state: ptr<function, u32>) -> vec3<f32> {
    // Generate three random numbers x,y,z using Gaussian distribution
    var x = rng_next_float_gauss(state);
    var y = rng_next_float_gauss(state);
    var z = rng_next_float_gauss(state);

    // Multiply each number by 1/sqrt(x²+y²+z²) (a.k.a. Normalise) .
    // case x=y=z=0 ?

    let length = sqrt(x * x + y * y + z * z);
    return vec3(x, y, z) / length;
}

fn rng_next_int(state: ptr<function, u32>) -> u32 {
    // PCG random number generator
    // Based on https://www.shadertoy.com/view/XlGcRh
    let newState = *state * 747796405u + 2891336453u;
    *state = newState;
    let word = ((newState >> ((newState >> 28u) + 4u)) ^ newState) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rng_next_float_gauss(state: ptr<function, u32>) -> f32 {
    let x1 = rng_next_float(state);
    let x2 = rng_next_float(state);
    return sqrt(-2.0 * log(x1)) * cos(2.0 * PI * x2);
}

fn rng_next_float(state: ptr<function, u32>) -> f32 {
    let x = rng_next_int(state);
    return f32(*state) / f32(0xffffffffu);
}

fn texture_look_up(desc: TextureDescriptor, x: f32, y: f32) -> vec3<f32> {
    var u = clamp(x, 0f, 1f);
    var v = 1f - clamp(y, 0f, 1f);

    let j = u32(u * f32(desc.width));
    let i = u32(v * f32(desc.height));
    let idx = i * desc.width + j;

    let elem = textures[desc.offset + idx];
    return vec3(elem[0u], elem[1u], elem[2u]);
}
