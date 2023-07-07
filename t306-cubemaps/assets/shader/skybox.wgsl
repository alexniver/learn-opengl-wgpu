struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
}

struct VertexOutSkybox {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coord: vec3<f32>,
}

@group(0)@binding(0)
var<uniform> view: mat4x4<f32>;
@group(0)@binding(1)
var<uniform> proj: mat4x4<f32>;
@group(0)@binding(2)
var<uniform> camera_pos: vec3<f32>;


@vertex
fn vs_main(in: VertexIn) -> VertexOutSkybox {
    var out: VertexOutSkybox;
    out.tex_coord = in.pos;
    out.clip_pos = (proj * view * vec4<f32>(in.pos, 1.0)).xyww;
    return out;
}

@group(1) @binding(0)
var sampler_skybox: sampler;
@group(1) @binding(1)
var texture_skybox: texture_cube<f32>;

@fragment
fn fs_main(in: VertexOutSkybox) -> @location(0) vec4<f32> {
    return textureSample(texture_skybox, sampler_skybox, in.tex_coord);
}
