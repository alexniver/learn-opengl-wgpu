use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("build window error");
    pollster::block_on(run(event_loop, window));
}

async fn run(even_loop: EventLoop<()>, window: Window) {
    even_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            //redraw
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
