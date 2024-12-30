#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder,
};

mod render_context;
use render_context::{RenderContext, Scene, Sphere};

mod vertex;
mod gpu_buffer;

mod camera;
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
    let (window, event_loop) = init(900, 450);
    let scenes = Scene {
        camera: camera::Camera {
            eye_pos: glm::vec3(0.0, 0.0, 1.0),
            eye_dir: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            vfov: 45.0,
            aperture: 0.1,
            focus_distance: 1.0,
        },
        spheres: vec![
            Sphere::new(
                glm::vec3(0.0, -100.5, -1.0),
                100.0,
            ),
            Sphere::new(
                glm::vec3(0.0, 0.0, -1.2),
                0.5,
            ),
            Sphere::new(
                glm::vec3(-1.0, 0.0, -1.0),
                0.5,
            ),
        ],
    };

    let mut context = RenderContext::new(&window, &scenes).await;
    let mut surface_configured = false;
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
                        if !surface_configured {
                            log::info!("Surface not configured yet");
                            return;
                        }
                        
                        context.update();
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
                        surface_configured = true;
                        context.resize(*physical_size);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }).unwrap();


}
