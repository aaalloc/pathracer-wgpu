mod camera;
pub use camera::{Camera, CameraController, GpuCamera};

mod material;
pub use material::{GpuMaterial, Material, Texture};

#[derive(Clone, Debug)]
pub struct Scene {
    pub materials: Vec<Material>,
    pub spheres: Vec<Sphere>,
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

        Self {
            camera,
            materials,
            spheres,
            render_param,
            frame_data,
            camera_controller: CameraController::new(4.0, 0.4),
        }
    }

    #[allow(dead_code)]
    pub fn new(
        camera: Camera,
        spheres: Vec<(Sphere, Material)>,
        render_param: RenderParam,
        frame_data: FrameData,
        camera_controller: CameraController,
    ) -> Self {
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
            camera_controller,
        }
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

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
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
