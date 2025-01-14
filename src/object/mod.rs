mod sphere;
pub use sphere::Sphere;

mod mesh;
pub use mesh::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Object {
    pub id: u32,
    pub obj_type: u32,
    pub count: u32,
    pub offset: u32,
}

pub struct ObjectList {
    pub objects: Vec<Object>,
    pub counter: u32,
    pub offset_counter: u32,
}

impl ObjectList {
    pub fn new() -> Self {
        ObjectList {
            objects: Vec::new(),
            counter: 0,
            offset_counter: 0,
        }
    }

    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj);
        self.counter += 1;
        self.offset_counter += obj.count;
    }

    pub fn add_sphere(&mut self, count: Option<usize>) {
        self.add(Object::new(
            self.counter,
            ObjectType::Sphere,
            count,
            Some(self.offset_counter),
        ));
    }

    pub fn add_mesh(&mut self, count: Option<usize>) {
        self.add(Object::new(
            self.counter,
            ObjectType::Mesh,
            count,
            Some(self.offset_counter),
        ));
    }
}

impl Object {
    pub fn new(id: u32, obj_type: ObjectType, count: Option<usize>, offset: Option<u32>) -> Self {
        Object {
            id,
            obj_type: obj_type as u32,
            count: count.unwrap_or(1) as u32,
            offset: offset.unwrap_or(0),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum ObjectType {
    Sphere = 0,
    Mesh = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Light {
    pub id: u32,
    pub light_type: u32,
}

impl Light {
    pub fn new(id: u32, light_type: ObjectType) -> Self {
        Light {
            id,
            light_type: light_type as u32,
        }
    }

    pub fn empty() -> Self {
        Light {
            id: 0xFFFFFFFF,
            light_type: 0,
        }
    }
}
