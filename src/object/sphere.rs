use super::aabb;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Sphere {
    pub center: glm::Vec4,  // 0 byte offset
    pub radius: f32,        // 16 byte offset
    pub material_idx: u32,  // 20 byte offset
    pub _padding: [u32; 2], // 24 byte offset, 8 bytes size
}

impl Sphere {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            center: glm::Vec4::new(0.0, 0.0, 0.0, 0.0),
            radius: 0.0,
            material_idx: 0,
            _padding: [0; 2],
        }
    }

    #[allow(dead_code)]
    pub fn new(center: glm::Vec3, radius: f32) -> Self {
        Self {
            center: glm::vec3_to_vec4(&center),
            radius,
            material_idx: 0,
            _padding: [0; 2],
        }
    }

    pub fn get_aabb(&self) -> aabb::AABB {
        let radius = glm::vec3(self.radius, self.radius, self.radius);
        let center = self.center.xyz();

        aabb::AABB {
            bounds: aabb::Bounds {
                min: center - radius,
                max: center + radius,
            },
            centroid: center,
            type_: 0,
        }
    }
}
