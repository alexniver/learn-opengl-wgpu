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
var texture_diffuse1: texture_2d<f32>;
@group(0)@binding(1)
var sampler_diffuse1: sampler;

@group(1)@binding(0)
var texture_diffuse2: texture_2d<f32>;
@group(1)@binding(1)
var sampler_diffuse2: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let v1 = textureSample(texture_diffuse1, sampler_diffuse1, in.tex_coord);
    let v2 = textureSample(texture_diffuse2, sampler_diffuse2, in.tex_coord);
    return mix(v1, v2, 0.5);
    //return textureSample(texture_diffuse2, sampler_diffuse2, in.tex_coord);
}
