mod sphere;
pub use sphere::Sphere;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Object {
    pub id: u32,
    pub obj_type: u32,
}

impl Object {
    pub fn new(id: u32, obj_type: ObjectType) -> Self {
        match obj_type {
            ObjectType::Sphere => Self {
                id,
                obj_type: ObjectType::Sphere as u32,
            },
            ObjectType::Mesh => Self {
                id,
                obj_type: ObjectType::Mesh as u32,
            },
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ObjectType {
    Sphere = 0,
    Mesh = 1,
}
