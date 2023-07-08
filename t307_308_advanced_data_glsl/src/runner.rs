use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::core::Core;

// const BASE_GLTF_PATH: &str = "assets/gltf/";

pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    pollster::block_on(async {
        let core = Core::new(window).await;

        Core::block_loop(event_loop, core);
    });
}
