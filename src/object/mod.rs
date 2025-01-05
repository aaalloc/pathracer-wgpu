mod sphere;
pub use sphere::Sphere;

mod mesh;
pub use mesh::Mesh;

mod aabb;
pub use aabb::AABB;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Object {
    pub id: u32,
    pub obj_type: u32,
}

impl Object {
    pub fn new(id: u32, obj_type: ObjectType) -> Self {
        Object {
            id,
            obj_type: obj_type as u32,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum ObjectType {
    Sphere = 0,
    Mesh = 1,
}
