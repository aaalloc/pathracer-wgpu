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

@group(1) @binding(0) var<storage, read> objects: array<Object>;
@group(1) @binding(1) var<storage, read> spheres: array<Sphere>;
@group(1) @binding(2) var<storage, read> materials: array<Material>;
@group(1) @binding(3) var<storage, read> textures: array<array<f32, 3>>;
@group(1) @binding(4) var<storage, read> surfaces: array<Surface>;


@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    return VertexOutput(
        vec4<f32>(model.position, 0.0, 1.0),
        model.tex_coords,
    );
}

fn apply_transfer_function(x: f32) -> u32 {
    let a = 0.055;
    var y: f32;
    if x > 0.0031308 {
        y = (1.0 + a) * pow(x, 1.0 / 2.4) - a;
    } else {
        y = 12.92 * x;
    }
    return u32(round(y * 255.0));
}
fn from_linear_rgb(c: vec3<f32>) -> vec3<f32> {

    let r = apply_transfer_function(c.x);
    let g = apply_transfer_function(c.y);
    let b = apply_transfer_function(c.z);

    return vec3<f32>(f32(r), f32(g), f32(b)) / 255.0;
}

// for webgpu
@fragment
fn fs_main_rgb(in: VertexOutput) -> @location(0) vec4<f32> {
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

    if render_param.clear_samples == 1u {
        pixel = vec3(0.0);
    }

    var rgb = sample_pixel(&rngState, f32(x), f32(y));
    rgb = from_linear_rgb(rgb);

    pixel += rgb;
    image_buffer[i] = array<f32, 3>(pixel.r, pixel.g, pixel.b);

    return vec4<f32>(
        pixel / f32(render_param.total_samples),
        1.0
    );

    // var noiseState: u32 = init_rng(vec2<u32>(u32(u), u32(v)), vec2<u32>(512u, 512u), 0u);
    // return vec4<f32>(rng_next_float(&rngState), rng_next_float(&rngState), rng_next_float(&rngState), 1.0);
}

@fragment
fn fs_main_srgb(in: VertexOutput) -> @location(0) vec4<f32> {
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

    if render_param.clear_samples == 1u {
        pixel = vec3(0.0);
    }

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
    clear_samples: u32,
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

struct Object {
    id: u32,
    obj_type: u32,
    // for when object is has multiple meshes
    count: u32,
};

const OBJECT_SPHERE = 0u;
const OBJECT_MESHES = 1u;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

struct Sphere {
    center: vec4<f32>,
    radius: f32,
    material_index: u32,
};

struct Surface {
    vertices: array<vec4<f32>, 3>,
    normals: array<vec4<f32>, 3>,
};

const MAT_LAMBERTIAN = 0u;
const MAT_METAL = 1u;
const MAT_DIELECTRIC = 2u;
const MAT_DIFFUSE_LIGHT = 3u;

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
    ray: Ray,
    attenuation: vec3<f32>,
    type_pdf: u32,
}

const PDF_NONE = 0u;
const PDF_COSINE = 1u;

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
    if root < ray_min || root > ray_max {
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

fn hit_triangle(
    triangle_index: u32,
    material_index: u32,
    ray: Ray,
    ray_min: f32,
    ray_max: f32,
    hit: ptr<function, HitRecord>,
) -> bool {
    let surface = surfaces[triangle_index];

    let e1 = surface.vertices[1].xyz - surface.vertices[0].xyz;
    let e2 = surface.vertices[2].xyz - surface.vertices[0].xyz;
    let h = cross(ray.direction, e2);
    let a = dot(e1, h);

    if a > -EPSILON && a < EPSILON {
        return false;
    }

    let f = 1.0 / a;
    let s = ray.origin - surface.vertices[0].xyz;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return false;
    }

    let q = cross(s, e1);
    let v = f * dot(ray.direction, q);

    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    let t = f * dot(e2, q);
    if t > ray_min && t < ray_max {
        let b = vec3(1.0 - u - v, u, v);
        let n = b.x * surface.normals[0].xyz + b.y * surface.normals[1].xyz + b.z * surface.normals[2].xyz;
        let front_face = dot(ray.direction, n) < 0.0;
        *hit = HitRecord(ray.origin + t * ray.direction, normalize(n), t, material_index, front_face);
        return true;
    }

    return false;
}

fn hit_object(
    object_index: u32,
    ray: Ray,
    ray_min: f32,
    ray_max: f32,
    hit: ptr<function, HitRecord>,
) -> bool {
    switch objects[object_index].obj_type {
        case OBJECT_SPHERE: {
            return hit_sphere(object_index, ray, ray_min, ray_max, hit);
        }
        case OBJECT_MESHES: {
            return hit_triangle(object_index, objects[object_index].id, ray, ray_min, ray_max, hit);
        }
        default: {
            return false;
        }
    }
}

fn check_intersection(ray: Ray, intersection: ptr<function, HitRecord>) -> bool {
    var closest_so_far = MAX_T;
    var hit_anything = false;
    var tmp_rec = HitRecord();

    var mesh_offset = 0u;
    for (var i = 0u; i < arrayLength(&objects); i += 1u) {
        let obj = objects[i];
        if obj.count > 1u {
            for (var j = 0u; j < obj.count; j += 1u) {
                if hit_triangle(mesh_offset + i + j, obj.id, ray, MIN_T, closest_so_far, &tmp_rec) {
                    hit_anything = true;
                    closest_so_far = tmp_rec.t;
                    *intersection = tmp_rec;
                }
            }
            mesh_offset += obj.count - 1u;
        } else {
            if hit_object(i, ray, MIN_T, closest_so_far, &tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *intersection = tmp_rec;
            }
        }
    }

    return hit_anything;
}

fn sample_pixel(rngState: ptr<function, u32>, x: f32, y: f32) -> vec3<f32> {
    var color = vec3(0.0);
    for (var i = 0u; i < render_param.samples_per_pixel; i += 1u) {
        let ray = get_ray(rngState, x, y);
        color += ray_color(ray, rngState);
    }
    return color;
}

fn get_ray(rngState: ptr<function, u32>, x: f32, y: f32) -> Ray {
    let u = f32(x + rng_next_float(rngState)) / f32(frame_data.width);
    let v = f32(y + rng_next_float(rngState)) / f32(frame_data.height);

    let rd = camera.lensRadius * rng_in_unit_disk(rngState);

    let origin = camera.eye + rd.x * camera.u + rd.y * camera.v;
    let direction = camera.lowerLeftCorner + u * camera.horizontal + v * camera.vertical - origin;

    return Ray(origin, direction);
}


fn ray_color(first_ray: Ray, rngState: ptr<function, u32>) -> vec3<f32> {
    var ray = first_ray;
    var sky_color = vec3(0.0);
    var color_from_scatter = vec3(1.0);
    var color_from_emission = vec3(0.0);

    for (var i = 0u; i < render_param.max_depth; i += 1u) {
        var intersection = HitRecord();
        if !check_intersection(ray, &intersection) {
            let direction = normalize(ray.direction);
            let a = 0.5 * (direction.y + 1.0);
            // sky_color = (1.0 - a) * vec3<f32>(1.0, 1.0, 1.0) + a * vec3<f32>(0.5, 0.7, 1);
            break;
        }
        // for triangles only
        // if !intersection.front_face {
        //     continue;
        // }

        let material = materials[intersection.material_index];
        color_from_emission += color_from_scatter * emitted(material, 0.5, 0.5, intersection);

        var scattered = Scatter();
        if !scatter(&scattered, ray, intersection, material, rngState) {
            break;
        }
        if scattered.type_pdf == PDF_NONE {
            color_from_scatter *= scattered.attenuation;
            ray = scattered.ray;
            continue;
        }

        scattered.ray.direction = pdf_generate(rngState, intersection);

        let pdf = pdf_mixed_value(
            pdf_value(
                scattered.type_pdf,
                scattered.ray.direction,
                pixar_onb(intersection.normal)
            ),
            pdf_light_value(intersection.p, scattered.ray.direction)
        );


        let scattering_pdf = scattering_pdf_lambertian(intersection.normal, scattered.ray.direction);

        color_from_scatter *= (scattered.attenuation * scattering_pdf) / pdf;
        ray = scattered.ray;
    }
    return color_from_emission + color_from_scatter * sky_color;
}



struct ONB {
    u: vec3<f32>,
    v: vec3<f32>,
    w: vec3<f32>,
}

fn pixar_onb(n: vec3<f32>) -> ONB {
    // https://www.jcgt.org/published/0006/01/01/paper-lowres.pdf
    let s = select(-1f, 1f, n.z >= 0f);
    let a = -1f / (s + n.z);
    let b = n.x * n.y * a;
    let u = vec3<f32>(1f + s * n.x * n.x * a, s * b, -s * n.x);
    let v = vec3<f32>(b, s + n.y * n.y * a, -n.y);

    return ONB(u, v, n);
}

fn emitted(material: Material, u: f32, v: f32, hit: HitRecord) -> vec3<f32> {
    switch (material.id) {
        case MAT_DIFFUSE_LIGHT: {
            if hit.front_face {
                return texture_look_up(material.desc, u, v);
            } else {
                return vec3(0.0);
            }
        }
        default: {
            return vec3(0.0);
        }
    }
}

fn scatter(
    s: ptr<function, Scatter>,
    ray: Ray,
    hit: HitRecord,
    material: Material,
    rngState: ptr<function, u32>,
) -> bool {
    switch (material.id) 
    {
        case MAT_LAMBERTIAN:
        {
            let onb = pixar_onb(hit.normal);
            let direction = pdf_cosine_generate(rngState, onb);

            *s = Scatter(
                Ray(hit.p, direction),
                texture_look_up(material.desc, 0.5, 0.5),
                PDF_COSINE
            );
        }
        case MAT_METAL: 
        {
            let reflected = reflect(normalize(ray.direction), hit.normal);
            let fuzz = material.fuzz;
            let direction = reflected + fuzz * rng_in_unit_sphere(rngState);
            *s = Scatter(
                Ray(hit.p, direction),
                texture_look_up(material.desc, 0.5, 0.5), PDF_NONE
            );
        }
        case MAT_DIELECTRIC: 
        {
            var ri: f32 = material.fuzz;
            // use select here
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
            *s = Scatter(
                Ray(hit.p, direction),
                vec3(1.0), PDF_NONE
            );
        }
        case MAT_DIFFUSE_LIGHT: 
        {
            return false;
        }
        default: {
            return false;
        }
    }
    return true;
}

fn scattering_pdf_lambertian(normal: vec3<f32>, direction: vec3<f32>) -> f32 {
    let cos_theta = dot(normalize(direction), normal);
    return select(0.0, cos_theta / PI, cos_theta > 0.0);
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


fn rng_on_hemisphere(rngState: ptr<function, u32>, normal: vec3<f32>) -> vec3<f32> {
    let on_unit_sphere = rng_in_unit_sphere(rngState);
    if dot(on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

fn rng_in_cosine_hemisphere(rngState: ptr<function, u32>) -> vec3<f32> {
    let r1 = rng_next_float(rngState);
    var r2 = rng_next_float(rngState);

    let z = sqrt(1.0 - r2);
    let phi = 2.0 * PI * r1;
    r2 = sqrt(r2);
    let x = cos(phi) * r2;
    let y = sin(phi) * r2;
    return vec3(x, y, z);
}

fn rng_in_unit_sphere(state: ptr<function, u32>) -> vec3<f32> {
    // Generate three random numbers x,y,z using Gaussian distribution
    var x = rng_next_float_gauss(state);
    var y = rng_next_float_gauss(state);
    var z = rng_next_float_gauss(state);

    // Multiply each number by 1/sqrt(x²+y²+z²) (a.k.a. Normalise) .
    // case x=y=z=0 ?

    let length = sqrt(x * x + y * y + z * z);
    return vec3(x, y, z) / length;
}

fn rng_in_unit_disk(state: ptr<function, u32>) -> vec2<f32> {
    var x = rng_next_float(state);
    var y = rng_next_float(state);
    return vec2(2.0 * x - 1.0, 2.0 * y - 1.0);
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

fn rng_next_float_bounded(state: ptr<function, u32>, min: f32, max: f32) -> f32 {
    return min + rng_next_float(state) * (max - min);
}

fn rng_next_float(state: ptr<function, u32>) -> f32 {
    let x = rng_next_int(state);
    return f32(*state) / f32(0xffffffffu);
}

fn pdf_sphere_value() -> f32 {
    return 1 / (4 * PI);
}

fn pdf_sphere_generate(state: ptr<function, u32>) -> vec3<f32> {
    return rng_in_unit_sphere(state);
}

fn pdf_cosine_value(direction: vec3<f32>, onb: ONB) -> f32 {
    let cosine_theta = dot(normalize(direction), onb.w);
    return max(cosine_theta, 0.0) / PI;
}

fn pdf_cosine_generate(state: ptr<function, u32>, onb: ONB) -> vec3<f32> {
    let rnd_direction = rng_in_cosine_hemisphere(state);
    return onb.u * rnd_direction.x + onb.v * rnd_direction.y + onb.w * rnd_direction.z;
}

fn pdf_light_generate(state: ptr<function, u32>, origin: vec3<f32>) -> vec3<f32> {
    let p = vec3(
        rng_next_float_bounded(state, -0.2, 0.2),
        0.99,
        rng_next_float_bounded(state, -0.2, 0.2)
    );
    return p - origin;
}

fn pdf_light_value(origin: vec3<f32>, direction: vec3<f32>) -> f32 {
    let area = 0.26; // hard coded for now
    var hit = HitRecord();
    if !check_intersection(Ray(origin, direction), &hit) {
        return 0.0;
    }

    let distance_squared = hit.t * hit.t * length(direction * direction);
    let cosine = abs(dot(direction, hit.normal) / length(direction));

    return distance_squared / (cosine * area);
}

fn pdf_mixed_value(value1: f32, value2: f32) -> f32 {
    return max(EPSILON, (0.5 * value1) + (0.5 * value2));
}


fn pdf_generate(
    rngState: ptr<function, u32>,
    hit: HitRecord,
) -> vec3<f32> {
    if rng_next_float(rngState) < 0.5 {
        return pdf_cosine_generate(rngState, pixar_onb(hit.normal));
    } else {
        return pdf_light_generate(rngState, hit.p);
    }
}

fn pdf_value(pdf_type: u32, direction: vec3<f32>, onb: ONB) -> f32 {
    switch (pdf_type) {
        case PDF_NONE: {
            // should not happen
            return 0.0;
        }
        case PDF_COSINE: {
            return pdf_cosine_value(direction, onb);
        }
        default: {
            return 0.0;
        }
    }
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
