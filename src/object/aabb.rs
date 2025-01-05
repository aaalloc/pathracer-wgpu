use glm::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: Vec3::new(0.0, 0.0, 0.0),
            max: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn union(b1: Bounds, b2: Bounds) -> Bounds {
        let min = Vec3::new(
            b1.min.x.min(b2.min.x),
            b1.min.y.min(b2.min.y),
            b1.min.z.min(b2.min.z),
        );
        let max = Vec3::new(
            b1.max.x.max(b2.max.x),
            b1.max.y.max(b2.max.y),
            b1.max.z.max(b2.max.z),
        );
        Bounds { min, max }
    }

    pub fn maximum_extent(&self) -> usize {
        let diag = self.max - self.min;
        if diag.x > diag.y && diag.x > diag.z {
            0
        } else if diag.y > diag.z {
            1
        } else {
            2
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AABB {
    pub bounds: Bounds,
    pub centroid: Vec3,
    pub type_: u32,
}
