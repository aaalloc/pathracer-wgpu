use scene::{Camera, CameraController, Material, Scene, Sphere, Texture};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowAttributes, WindowId}
};

mod render_context;
use render_context::RenderContext;

mod utils;

mod scene;
extern crate nalgebra_glm as glm;


struct MyUserEvent;

struct State<'a> {
    window: &'a Window,
    render_context: RenderContext<'a>,
    last_time: instant::Instant,
    mouse_pressed: bool,
    counter: i32,
}

impl ApplicationHandler<MyUserEvent> for State<'_> {
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _user_eventt: MyUserEvent) {
        // Handle user event.
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // Your application got resumed.
    }

    fn window_event(
        &mut self, 
        event_loop: &ActiveEventLoop, 
        _window_id: WindowId, 
        event: WindowEvent
    ) {
        self.render_context.window_event(&event, &mut self.mouse_pressed);
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.window.request_redraw();
                let now = instant::Instant::now();
                let dt = now - self.last_time;
                self.last_time = now;

                self.render_context.update(dt);
                match self.render_context.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated)
                        =>  self.render_context.resize( self.render_context.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log::error!("Out of memory");
                        event_loop.exit();
                    }
                    // This happens when the a frame takes too long to present
                    Err(wgpu::SurfaceError::Timeout) => {
                        log::warn!("Surface timeout")
                    }
                }
            },
            WindowEvent::Resized(physical_size) => {
                self.render_context.resize(physical_size);
            },
            _ => {}
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        self.render_context.device_event(&event, self.mouse_pressed);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
        self.counter += 1;
    }
}

fn init(width: u32, height: u32) -> (winit::window::Window, winit::event_loop::EventLoop<MyUserEvent>) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
     
    log::info!("Starting up");
    
    
    
    
    let event_loop = EventLoop::<MyUserEvent>::with_user_event().build().unwrap();
    #[allow(deprecated)]
    let window = event_loop.create_window(
        WindowAttributes::default()
    .with_title("Raytracer")
    .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
    ).unwrap();
    

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(PhysicalSize::new(width, height));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    return (window, event_loop)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {    
    let width = 900 * 2;
    let height = 450 * 2;
    let (window, event_loop) = init(width, height);
    let scenes = Scene::new(
        Camera {
            eye_pos: glm::vec3(1.0, 0.0, 1.0),
            eye_dir: glm::vec3(-1.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            vfov: 45.0,
            aperture: 0.1,
            focus_distance: 1.0,
        },
        vec![
            (
                Sphere::new(
                    glm::vec3(0.0, 0.0, -1.0),
                    0.5,
                ),
                Material::Lambertian { 
                    albedo: Texture::new_from_color(glm::vec3(0.1, 0.2, 0.5)),
                }
            ),
            (
                Sphere::new(
                    glm::vec3(0.0, -100.5, -1.0),
                    100.0,
                ),
                Material::Lambertian { 
                    albedo: Texture::new_from_color(glm::vec3(0.8, 0.8, 0.0)),
                },
            ),
            (
                Sphere::new(
                    glm::vec3(-1.0, 0.0, -1.0),
                    0.5,
                ),
                Material::Metal { 
                    albedo: Texture::new_from_color(glm::vec3(0.8, 0.6, 0.2)),
                    fuzz: 0.0,
                },
            ),
            (
                Sphere::new(
                    glm::vec3(1.0, 0.0, -1.0),
                    0.5,
                ),
                Material::Dialectric { 
                    ref_idx: 1.5,
                },
            ),
            (
                Sphere::new(
                    glm::vec3(1.0, 0.0, -1.0),
                    0.4,
                ),
                Material::Dialectric { 
                    ref_idx: 1.0/1.5,
                },
            ),
        ],
        scene::RenderParam {
            samples_per_pixel: 1,
            max_depth: 10,
            samples_max_per_pixel: 1000,
            total_samples: 0,
            clear_samples: 0,
        },
        scene::FrameData {
            width,
            height,
            index: 0,
        },
        CameraController::new(
            4.0, 
            0.4
        )
    );


    let mut state = State {
        window: &window,
        mouse_pressed: false,
        last_time: instant::Instant::now(),
        render_context: RenderContext::new(
            &window,
            &scenes
        ).await,
        counter: 0,
    };
    
    let _ = event_loop.run_app(&mut state);
}
