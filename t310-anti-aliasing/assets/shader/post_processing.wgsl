struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.clip_pos = vec4<f32>(in.pos, 1.0);
    out.tex_coord = in.tex_coord;
    return out;
}

@group(0) @binding(0)
var texture_sampler: sampler;
@group(0) @binding(1)
var input_texture: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    var color: vec4<f32> = textureSample(input_texture, texture_sampler, in.tex_coord);

    var grayscale: f32 = (color.r + color.g + color.b) / 3.0;

    //return color;
    return vec4<f32>(grayscale, grayscale, grayscale, color.a);
    //return vec4<f32>(1.0);
}

