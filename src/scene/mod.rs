mod camera;
pub use camera::{Camera, CameraController, GpuCamera};

mod material;
pub use material::{GpuMaterial, Material, Texture};

use crate::object::{self, Mesh, Object, ObjectType, Sphere, AABB};

#[derive(Clone, Debug)]
pub struct Scene {
    pub materials: Vec<Material>,
    pub objects: Vec<Object>,
    pub spheres: Vec<Sphere>,
    pub meshes: Vec<Mesh>,
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub render_param: RenderParam,
    pub frame_data: FrameData,
}

impl PartialEq for Scene {
    fn eq(&self, other: &Self) -> bool {
        self.materials == other.materials
            && self.spheres == other.spheres
            && self.camera == other.camera
            && self.frame_data == other.frame_data
            && self.camera_controller == other.camera_controller
    }
}

impl Scene {
    #[allow(dead_code)]
    pub fn raytracing_scene_oneweek(render_param: RenderParam, frame_data: FrameData) -> Self {
        let mut spheres = Vec::new();
        let mut materials = Vec::new();

        let ground_material = Material::Lambertian {
            albedo: Texture::new_from_color(glm::vec3(0.5, 0.5, 0.5)),
        };

        materials.push(ground_material);
        spheres.push(Sphere::new(glm::vec3(0.0, -1000.0, 0.0), 1000.0));

        for (a, b) in (-11..11).flat_map(|a| (-11..11).map(move |b| (a, b))) {
            let choose_mat = rand::random::<f32>();
            let center = glm::vec3(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - glm::vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    Material::Lambertian {
                        albedo: Texture::new_from_color(glm::vec3(
                            rand::random::<f32>() * rand::random::<f32>(),
                            rand::random::<f32>() * rand::random::<f32>(),
                            rand::random::<f32>() * rand::random::<f32>(),
                        )),
                    }
                } else if choose_mat < 0.95 {
                    Material::Metal {
                        albedo: Texture::new_from_color(glm::vec3(
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                        )),
                        fuzz: rand::random::<f32>() * 0.5,
                    }
                } else {
                    Material::Dialectric { ref_idx: 1.5 }
                };

                materials.push(sphere_material);
                spheres.push(Sphere::new(center, 0.2));
            }
        }

        spheres.push(Sphere::new(glm::vec3(0.0, 1.0, 0.0), 1.0));
        materials.push(Material::Dialectric { ref_idx: 1.5 });

        spheres.push(Sphere::new(glm::vec3(-4.0, 1.0, 0.0), 1.0));
        materials.push(Material::Lambertian {
            albedo: Texture::new_from_color(glm::vec3(0.4, 0.2, 0.1)),
        });

        spheres.push(Sphere::new(glm::vec3(4.0, 1.0, 0.0), 1.0));
        materials.push(Material::Metal {
            albedo: Texture::new_from_color(glm::vec3(0.7, 0.6, 0.5)),
            fuzz: 0.0,
        });

        let camera = Camera {
            eye_pos: glm::vec3(-10.5, 2.73, -5.83),
            eye_dir: glm::vec3(0.9086872, -0.15932521, 0.3858796),
            up: glm::vec3(0.0, 1.0, 0.0),
            vfov: 20.0,
            aperture: 0.6,
            focus_distance: 10.0,
        };

        let objects: Vec<Object> = spheres
            .iter()
            .enumerate()
            .map(|(i, _)| Object::new(i as u32, object::ObjectType::Sphere))
            .collect();

        Self {
            objects,
            camera,
            meshes: vec![Mesh::empty()],
            materials,
            spheres,
            render_param,
            frame_data,
            camera_controller: CameraController::new(4.0, 0.4),
        }
    }

    pub fn teapot_scene(render_param: RenderParam, frame_data: FrameData) -> Self {
        let mut materials = Vec::new();
        let mut objects = Vec::new();

        let ground_material = Material::Lambertian {
            albedo: Texture::new_from_color(glm::vec3(0.5, 0.5, 0.5)),
        };

        materials.push(ground_material);

        let path_str = "assets/mesh/teapot.obj";
        let options = tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        };
        println!("Current path: {:?}", std::env::current_dir().unwrap());

        let s = tobj::load_obj(path_str, &options).unwrap().0[0].clone();

        let meshes = Mesh::from_tobj(s);

        objects.push(Object::new(0, ObjectType::Mesh));

        let camera = Camera {
            eye_pos: glm::vec3(0.0, 0.0, 50.0),
            eye_dir: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            vfov: 45.0,
            aperture: 0.0,
            focus_distance: 1.0,
        };

        Self {
            objects,
            camera,
            meshes,
            materials,
            spheres: vec![Sphere::empty()],
            render_param,
            frame_data,
            camera_controller: CameraController::new(4.0, 0.4),
        }
    }

    pub fn get_bvh(&self) -> Vec<AABB> {
        let mut bvh = Vec::new();

        let axis = rand::random::<u32>() % 3;
        let mut spheres = self.spheres.clone();

        spheres.sort_by(|a, b| {
            let a = a.get_bounding_box();
            let b = b.get_bounding_box();

            a.min[axis as usize]
                .partial_cmp(&b.min[axis as usize])
                .unwrap()
        });

        let mut stack = Vec::new();
        stack.push((0, spheres.len(), 0));

        while let Some((start, end, _)) = stack.pop() {
            let mut min = glm::vec3(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
            let mut max = glm::vec3(
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
            );

            for i in start..end {
                let sphere = &spheres[i];
                let aabb = sphere.get_bounding_box();

                min = glm::vec3(
                    min.x.min(aabb.min.x),
                    min.y.min(aabb.min.y),
                    min.z.min(aabb.min.z),
                );

                max = glm::vec3(
                    max.x.max(aabb.max.x),
                    max.y.max(aabb.max.y),
                    max.z.max(aabb.max.z),
                );
            }

            let mid = (start + end) / 2;

            let left_child = if mid - start == 1 {
                start as u32
            } else {
                bvh.len() as u32 + 1
            };

            let right_child = if end - mid == 1 {
                mid as u32
            } else {
                bvh.len() as u32 + 2
            };

            bvh.push(AABB {
                min,
                max,
                left_child,
                right_child,
            });

            if mid - start > 1 {
                stack.push((start, mid, bvh.len() as u32 - 1));
            }

            if end - mid > 1 {
                stack.push((mid, end, bvh.len() as u32 - 1));
            }
        }

        // DEBUG: traverse bvh and print a tree
        for (i, aabb) in bvh.iter().enumerate() {
            println!(
                "Node: {} min: {:?} max: {:?} left: {} right: {}",
                i, aabb.min, aabb.max, aabb.left_child, aabb.right_child
            );
        }

        bvh
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct RenderParam {
    pub samples_max_per_pixel: u32,
    pub samples_per_pixel: u32,
    pub total_samples: u32,
    pub clear_samples: u32,
    pub max_depth: u32,
}

impl RenderParam {
    pub fn update(&mut self) {
        if self.total_samples == 0 {
            self.total_samples += self.samples_per_pixel;
            self.clear_samples = 1;
        } else if self.total_samples <= self.samples_max_per_pixel {
            self.total_samples += self.samples_per_pixel;
            self.clear_samples = 0;
        } else {
            self.samples_per_pixel = 0;
            self.clear_samples = 0;
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FrameData {
    pub width: u32,
    pub height: u32,
    pub index: u32,
}

impl PartialEq for FrameData {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
