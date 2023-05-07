struct VertexInput {
    @location(0)pos: vec3<f32>,
    @location(1)tex_coord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position)clip_pos: vec4<f32>,
    @location(0)tex_coord: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_pos = vec4<f32>(input.pos, 1.0);
    out.tex_coord = input.tex_coord;
    return out;
}

@group(0)@binding(0)
var texture_diffuse_container: texture_2d<f32>;
@group(0)@binding(1)
var sampler_diffuse_container: sampler;
@group(0)@binding(2)
var texture_diffuse_huaji: texture_2d<f32>;
@group(0)@binding(3)
var sampler_diffuse_huaji: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0)vec4<f32> {
    let v1 = textureSample(texture_diffuse_container, sampler_diffuse_container, input.tex_coord);
    let v2 = textureSample(texture_diffuse_huaji, sampler_diffuse_huaji, vec2<f32>(1.0 - input.tex_coord.x, input.tex_coord.y));
    return mix(v1, v2, 0.5);
}
