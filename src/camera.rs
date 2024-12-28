#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuCamera {
    eye: glm::Vec3,
    _padding1: f32,
    horizontal: glm::Vec3,
    _padding2: f32,
    vertical: glm::Vec3,
    _padding3: f32,
    u: glm::Vec3,
    _padding4: f32,
    v: glm::Vec3,
    lens_radius: f32,
    lower_left_corner: glm::Vec3,
    _padding5: f32,
}

impl GpuCamera {
    // TODO: This is currently a big mess, everything is hardcoded for making things work.
    pub fn new(_viewport_size: (u32, u32)) -> Self {
        let eye = glm::vec3(0_f32, 0_f32, 0_f32);
        let horizontal = glm::vec3(4_f32, 0_f32, 0_f32);
        let vertical = glm::vec3(0_f32, 2_f32, 0_f32);
        let lower_left_corner = glm::vec3(-2_f32, -1_f32, -1_f32);
        let u = glm::vec3(0_f32, 0_f32, 0_f32);
        let v = glm::vec3(0_f32, 0_f32, 0_f32);
        let lens_radius = 0_f32;

        Self {
            eye,
            _padding1: 0_f32,
            horizontal,
            _padding2: 0_f32,
            vertical,
            _padding3: 0_f32,
            u,
            _padding4: 0_f32,
            v,
            lens_radius,
            lower_left_corner,
            _padding5: 0_f32,
        }
    }

    // pub fn new(camera: &Camera, viewport_size: (u32, u32)) -> Self {
    //     let lens_radius = 0.5_f32 * camera.aperture;
    //     let aspect = viewport_size.0 as f32 / viewport_size.1 as f32;
    //     let theta = 45_f32.to_radians();
    //     let half_height = camera.focus_distance * (0.5_f32 * theta).tan();
    //     let half_width = aspect * half_height;

    //     let w = glm::normalize(&camera.eye_dir);
    //     let v = glm::normalize(&camera.up);
    //     let u = glm::cross(&w, &v);

    //     let lower_left_corner =
    //         camera.eye_pos + camera.focus_distance * w - half_width * u - half_height * v;
    //     let horizontal = 2_f32 * half_width * u;
    //     let vertical = 2_f32 * half_height * v;

    //     Self {
    //         eye: camera.eye_pos,
    //         _padding1: 0_f32,
    //         horizontal,
    //         _padding2: 0_f32,
    //         vertical,
    //         _padding3: 0_f32,
    //         u,
    //         _padding4: 0_f32,
    //         v,
    //         lens_radius,
    //         lower_left_corner,
    //         _padding5: 0_f32,
    //     }
    // }
}