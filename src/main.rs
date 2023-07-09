
mod abort;
mod watcher;
mod window;
mod shaders;
mod timeline;
mod canvas;
mod glsl;
mod picture;
mod runtime;
mod interop;
mod app;

// ------------------------------------------------------------

use abort::*;
use winit::{event::*, event_loop::*};

// ------------------------------------------------------------

fn main() -> !
{
    let path = std::env::args().nth(1).unwrap_or_default();
    let event_loop = EventLoop::new();
    let mut app = app::App::new(&event_loop, path).aborts();
    event_loop.run
    (
        move |event, _, control| match event
        {
            Event::WindowEvent{event, ..} => match event
            {
                WindowEvent::MouseInput
                {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => app.drag_window().aborts(),
                WindowEvent::KeyboardInput
                {
                    input: KeyboardInput
                    {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => *control = ControlFlow::Exit,
                _ => {}
            }
            Event::MainEventsCleared
                => app.refresh().aborts(),
            _ => {}
        }
    )
}

