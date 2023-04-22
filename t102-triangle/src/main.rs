use std::fs;

use t102_triangle::{runner::run, vertex::Vertex};
use wgpu::{
    util::DeviceExt, Backends, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, Features, FragmentState, Instance, PrimitiveState, TextureUsages,
    VertexState,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("create window error");
    pollster::block_on(run(event_loop, window));
}
