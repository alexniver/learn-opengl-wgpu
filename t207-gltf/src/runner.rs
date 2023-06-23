use glam::{Quat, Vec3};
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::{
    core::Core,
    model::{DrawMethod, Model},
    transform::Transform,
    vertex::Vertex,
};

pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    pollster::block_on(async {
        let mut core = Core::new(window).await;
        // let model = box_model();
        let mut model_arr = gltf_model_arr(&core);
        core.add_model_arr(&mut model_arr);
        // let rect_model = rect_model(&core);
        // core.add_model(rect_model);

        Core::block_loop(event_loop, core);
    });
}

fn gltf_model_arr(core: &Core) -> Vec<Model> {
    let mut result = vec![];

    let base_path = std::path::Path::new("assets/gltf/");
    let gltf_path = base_path.join("Cube.gltf");
    let gltf1 = gltf::Gltf::open(gltf_path).unwrap();
    let mut buffer_data = Vec::new();

    for buffer in gltf1.buffers() {
        match buffer.source() {
            gltf::buffer::Source::Bin => {
                if let Some(data) = gltf1.blob.as_deref() {
                    buffer_data.push(data.into());
                }
            }
            gltf::buffer::Source::Uri(path) => {
                let path = base_path.join(path);
                println!("path: {:?}", path);
                let data = std::fs::read(path).unwrap();
                buffer_data.push(data);
            }
        }
    }

    for scene in gltf1.scenes() {
        for node in scene.nodes() {
            let mesh = node.mesh().unwrap();
            let primitives = mesh.primitives();
            primitives.for_each(|primitive| {
                let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

                let mut vertices = vec![];
                if let Some(positions) = reader.read_positions() {
                    positions.for_each(|position| {
                        vertices.push(Vertex::new(
                            position,
                            Default::default(),
                            Default::default(),
                        ));
                    });
                }

                if let Some(normals) = reader.read_normals() {
                    let mut normal_idx = 0;
                    normals.for_each(|normal| {
                        vertices[normal_idx].normal = normal;
                        normal_idx += 1;
                    });
                }

                if let Some(tex_coords) = reader.read_tex_coords(0).map(|v| v.into_f32()) {
                    let mut tex_coord_idx = 0;
                    tex_coords.for_each(|tex_coord| {
                        vertices[tex_coord_idx].tex_coord = tex_coord;
                        tex_coord_idx += 1;
                    });
                }

                let mut indices = vec![];
                if let Some(indices_raw) = reader.read_indices() {
                    indices.append(&mut indices_raw.into_u32().collect::<Vec<u32>>());
                }

                let model = Model::new(
                    &core.device,
                    DrawMethod::Index,
                    vertices,
                    indices,
                    vec![Transform::new(
                        Vec3::new(0.0, 0.0, 0.0),
                        Quat::IDENTITY,
                        Vec3::ONE,
                    )],
                );
                result.push(model);
            });
        }
    }

    result
}

fn box_model(core: &Core) -> Model {
    let transform_arr = transforms();
    Model::new(
        &core.device,
        DrawMethod::Vertex,
        Vertex::cube().into(),
        vec![],
        transform_arr,
    )
}

fn rect_model(core: &Core) -> Model {
    let (vertices, indices) = Vertex::rect();
    Model::new(
        &core.device,
        DrawMethod::Index,
        vertices.into(),
        indices.into(),
        vec![Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)],
    )
}

fn triangle_model(core: &Core) -> Model {
    let vertices = Vertex::triangle();
    Model::new(
        &core.device,
        DrawMethod::Vertex,
        vertices.into(),
        vec![],
        vec![Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)],
    )
}

fn transforms() -> Vec<Transform> {
    let mut transform_arr = vec![];
    let axis = Vec3::new(1.0, 0.3, 0.5).normalize();
    let pos_arr = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 5.0, -15.0),
        Vec3::new(-1.5, -2.2, -2.5),
        Vec3::new(-3.8, -2.0, -12.3),
        Vec3::new(2.4, -0.4, -3.5),
        Vec3::new(-1.7, 3.0, -7.5),
        Vec3::new(1.3, -2.0, -2.5),
        Vec3::new(1.5, 2.0, -2.5),
        Vec3::new(1.5, 0.2, -1.5),
        Vec3::new(-1.3, 1.0, -1.5),
    ];
    for (i, pos) in pos_arr.into_iter().enumerate() {
        transform_arr.push(Transform::new(
            pos,
            Quat::from_axis_angle(axis, (20.0 * i as f32).to_radians()),
            Vec3::ONE,
        ));
    }
    transform_arr
}
