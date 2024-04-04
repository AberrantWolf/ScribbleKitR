use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::render::{Renderer, VoidRenderer};

trait RendererTrait: Renderer + std::fmt::Debug {}

#[derive(Debug)]
pub struct App {
    name: String,
    window: winit::window::Window,
    event_loop: EventLoop<()>,
    renderer: Box<dyn Renderer>,
}

impl App {
    /// Creates a new [`App`] with a void/passthrough renderer.
    ///
    /// # Panics
    ///
    /// Panics if the window or event loop cannot be created for some reason.
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new().expect("Unable to create event loop");
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let window = winit::window::WindowBuilder::new()
            .with_title(name)
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)
            .expect("Error creating the window.");

        App {
            name: name.to_owned(),
            window,
            event_loop,
            renderer: Box::new(VoidRenderer::create(&name)),
        }
    }

    pub fn set_renderer(&mut self, renderer: Box<dyn Renderer>) {
        self.renderer = renderer;
    }

    pub fn run(self) -> Result<(), EventLoopError> {
        self.event_loop
            .run(move |event, window_target| match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("Close button pressed");
                            window_target.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            // just render?
                            self.renderer.render()
                        }
                        WindowEvent::KeyboardInput {
                            // TODO: make an input handler and pass this off to that
                            device_id: _,
                            event,
                            is_synthetic: _,
                        } => match (event.physical_key, event.state) {
                            (PhysicalKey::Code(KeyCode::Escape), ElementState::Pressed) => {
                                window_target.exit()
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                Event::AboutToWait => {
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw in
                    // applications which do not always need to. Applications that redraw continuously
                    // can render here instead.
                    self.window.request_redraw();
                }
                _ => (),
            })
    }
}
