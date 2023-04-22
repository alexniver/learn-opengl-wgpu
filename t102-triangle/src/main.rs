use t102_triangle::runner::run;

use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("create window error");
    pollster::block_on(run(event_loop, window));
}
