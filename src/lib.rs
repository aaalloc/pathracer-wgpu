use scene::{Camera, CameraController, Material, Scene, Sphere, Texture};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder,
};

mod render_context;
use render_context::RenderContext;

mod utils;

mod scene;
extern crate nalgebra_glm as glm;


pub fn init(width: u32, height: u32) -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
     
    log::info!("Starting up");
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();




    let window = WindowBuilder::new()
        .with_title("Raytracer")
        .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();
    

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

    let mut context = RenderContext::new(&window, &scenes).await;
    let mut last_render_time = instant::Instant::now();
    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == context.window().id() => if !context.input(event) {
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
                    } => control_flow.exit(),
                    WindowEvent::RedrawRequested => {
                        context.window().request_redraw();
                        let now = instant::Instant::now();
                        let dt = now - last_render_time;
                        last_render_time = now;

                        context.update(dt);
                        match context.render() {
                            Ok(_) => {},
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated)
                                => context.resize(context.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                log::error!("Out of memory");
                                control_flow.exit();
                            }
                            // This happens when the a frame takes too long to present
                            Err(wgpu::SurfaceError::Timeout) => {
                                log::warn!("Surface timeout")
                            }
                        }
                    },
                    WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }).unwrap();


}
