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
var tex_sampler: sampler;
@group(0)@binding(1)
var tex_container: texture_2d<f32>;
@group(0)@binding(2)
var tex_huaji: texture_2d<f32>;

@group(1)@binding(0)
var<uniform> bias: f32;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let container = textureSample(tex_container, tex_sampler, in.tex_coord);
    let huaji = textureSample(tex_huaji, tex_sampler, in.tex_coord);
    return mix(container, huaji, bias);
}
