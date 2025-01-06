#[derive(Clone, Debug, PartialEq)]
pub struct Texture {
    dimensions: (u32, u32),
    data: Vec<[f32; 3]>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TextureDescriptor {
    width: u32,
    height: u32,
    offset: u32,
}

impl Texture {
    pub fn new_from_color(color: glm::Vec3) -> Self {
        Self {
            dimensions: (1, 1),
            data: vec![[color.x, color.y, color.z]],
        }
    }

    pub fn as_slice(&self) -> &[[f32; 3]] {
        &self.data
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Material {
    Lambertian { albedo: Texture },
    Metal { albedo: Texture, fuzz: f32 },
    Dialectric { ref_idx: f32 },
    DiffuseLight { emit: Texture },
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuMaterial {
    id: u32,
    descriptor: TextureDescriptor,
    x: f32,
}
impl GpuMaterial {
    fn append_to_global_texture_data(
        texture: &Texture,
        global_texture_data: &mut Vec<[f32; 3]>,
    ) -> TextureDescriptor {
        let dimensions = texture.dimensions();
        let offset = global_texture_data.len() as u32;
        global_texture_data.extend_from_slice(texture.as_slice());
        TextureDescriptor {
            width: dimensions.0,
            height: dimensions.1,
            offset,
        }
    }

    pub fn new(material: &Material, global_texture_data: &mut Vec<[f32; 3]>) -> Self {
        match material {
            Material::Lambertian { albedo } => Self {
                id: 0,
                descriptor: Self::append_to_global_texture_data(albedo, global_texture_data),
                x: 0.0,
            },
            Material::Metal { albedo, fuzz } => Self {
                id: 1,
                descriptor: Self::append_to_global_texture_data(albedo, global_texture_data),
                x: *fuzz,
            },
            Material::Dialectric { ref_idx } => Self {
                id: 2,
                descriptor: TextureDescriptor {
                    width: 0,
                    height: 0,
                    offset: 0xffffffff,
                },
                x: *ref_idx,
            },
            Material::DiffuseLight { emit } => Self {
                id: 3,
                descriptor: Self::append_to_global_texture_data(emit, global_texture_data),
                x: 0.0,
            },
        }
    }
}
