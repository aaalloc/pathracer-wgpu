mod camera;
pub use camera::{Camera, GpuCamera};

mod material;
pub use material::{Material, GpuMaterial, Texture};



#[derive(Clone)]
pub struct Scene {
    pub camera: Camera,
    pub materials: Vec<Material>,
    pub spheres: Vec<Sphere>,
    pub render_param: RenderParam,
    pub frame_data: FrameData,
}

impl Scene {
    pub fn new(camera: Camera, spheres: Vec<(Sphere, Material)>, render_param: RenderParam, frame_data: FrameData) -> Self {
        let mut materials = Vec::new();
        let mut s = Vec::new();

        for (sphere, material) in spheres {
            materials.push(material);
            s.push(Sphere {
                material_idx: materials.len() as u32 - 1,
                ..sphere
            });
        }

        Self {
            camera,
            materials,
            spheres: s,
            render_param,
            frame_data,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderParam {
    pub samples_max_per_pixel: u32,
    pub samples_per_pixel: u32,
    pub total_samples: u32,
    pub max_depth: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FrameData {
    pub width: u32,
    pub height: u32,
    pub index: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere {
    center: glm::Vec4,  // 0 byte offset
    radius: f32,        // 16 byte offset
    material_idx: u32,  // 20 byte offset
    _padding: [u32; 2], // 24 byte offset, 8 bytes size
}

impl Sphere {
    pub fn new(center: glm::Vec3, radius: f32) -> Self {
        Self {
            center: glm::vec3_to_vec4(&center),
            radius,
            material_idx: 0,
            _padding: [0; 2],
        }
    }
}
