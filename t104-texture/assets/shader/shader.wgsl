struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_pos = vec4<f32>(in.pos, 1.0);
    out.tex_coord = in.tex_coord;
    return out;
}

@group(0)@binding(0)
var texture_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var sampler_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_diffuse, sampler_diffuse, in.tex_coord);
}
