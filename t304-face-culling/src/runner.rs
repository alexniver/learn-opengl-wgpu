use glam::{Quat, Vec3};
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::{
    core::Core,
    material::Material,
    model::{DrawMethod, Model},
    texture::gen_texture_view,
    transform::Transform,
    vertex::Vertex,
};

const BASE_GLTF_PATH: &str = "assets/gltf/";

pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    pollster::block_on(async {
        let mut core = Core::new(window).await;
        load_box_model(&mut core);
        // load_rect_model(&mut core);
        // load_triangle_model(&mut core);
        // load_gltf_model(&mut core);

        Core::block_loop(event_loop, core);
    });
}

pub fn load_gltf_model(core: &mut Core) {
    let base_path = std::path::Path::new(BASE_GLTF_PATH);

    let gltf_path = base_path.join("box.gltf");
    let gltf_info = gltf::Gltf::open(gltf_path).unwrap();

    let mut buffer_data = Vec::new();
    for buffer in gltf_info.buffers() {
        match buffer.source() {
            gltf::buffer::Source::Bin => {}
            gltf::buffer::Source::Uri(path) => {
                let path = base_path.join(path);
                let data = std::fs::read(path).unwrap();
                buffer_data.push(data);
            }
        }
    }

    // let material_arr = vec![];
    for material in gltf_info.materials() {
        let info = material
            .pbr_metallic_roughness()
            .base_color_texture()
            .unwrap();
        let texture = info.texture();
        let source = texture.source().source();
        let texture_view;
        match source {
            gltf::image::Source::View { view, .. } => {
                texture_view =
                    gen_texture_view(buffer_data[view.index()].clone(), &core.device, &core.queue)
                        .unwrap();
            }
            gltf::image::Source::Uri { uri, .. } => {
                texture_view = gen_texture_view(
                    std::fs::read(base_path.join(url_escape::decode(uri).to_string())).unwrap(),
                    &core.device,
                    &core.queue,
                )
                .unwrap();
            }
        }

        let material = Material::new(texture_view, 32.0, core);
        core.material_arr.push(material);
    }

    for mesh in gltf_info.meshes() {
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

            core.material_arr[primitive.material().index().unwrap() as usize]
                .model_arr
                .push(model);
        });
    }
}

pub fn load_box_model(core: &mut Core) {
    let mut material = box_material(core);

    let transform_arr = transforms();
    let model = Model::new(
        &core.device,
        DrawMethod::Vertex,
        Vertex::cube().into(),
        vec![],
        transform_arr,
    );
    material.model_arr.push(model);
    core.material_arr.push(material);
}

pub fn load_rect_model(core: &mut Core) {
    let mut material = box_material(core);

    let (vertices, indices) = Vertex::rect();
    let model = Model::new(
        &core.device,
        DrawMethod::Index,
        vertices.into(),
        indices.into(),
        vec![Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)],
    );
    material.model_arr.push(model);
    core.material_arr.push(material);
}

pub fn load_triangle_model(core: &mut Core) {
    let mut material = box_material(core);

    let vertices = Vertex::triangle();
    let model = Model::new(
        &core.device,
        DrawMethod::Vertex,
        vertices.into(),
        vec![],
        vec![Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)],
    );

    material.model_arr.push(model);
    core.material_arr.push(material);
}

fn box_material(core: &Core) -> Material {
    let (device, queue) = (&core.device, &core.queue);
    let texture_diffuse_view = gen_texture_view(
        std::fs::read("assets/texture/container2.png").unwrap(),
        &device,
        &queue,
    )
    .unwrap();

    Material::new(texture_diffuse_view, 32.0, core)
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
