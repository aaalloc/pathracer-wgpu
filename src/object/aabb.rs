use glm::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
    pub left_child: u32,
    pub right_child: u32,
}
