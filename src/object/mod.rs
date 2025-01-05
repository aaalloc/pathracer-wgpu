mod sphere;
pub use sphere::Sphere;

mod mesh;
pub use mesh::Mesh;

mod aabb;
pub use aabb::{Bounds, AABB};

mod bvh;
pub use bvh::get_bvh;

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

impl ObjectType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => ObjectType::Sphere,
            1 => ObjectType::Mesh,
            _ => panic!("Unknown object type"),
        }
    }
}
