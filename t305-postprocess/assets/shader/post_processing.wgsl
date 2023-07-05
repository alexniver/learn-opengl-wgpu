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

@group(0)@binding(0)
var texture_post_processing_sampler: sampler;
@group(0)@binding(1)
var texture_post_processing: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let t = textureSample(texture_post_processing, texture_post_processing_sampler, in.tex_coord);
    //return vec4<f32>(1.0, 1.0, 0.0, 1.0);

    // Inversion
    //return vec4<f32>(1.0 - t.rgb, 1.0);

    // Grayscale
    //let average = 0.2126 * t.r + 0.7152 * t.g + 0.0722 * t.b;
    //return vec4<f32>(average, average, average, 1.0);
    return t;
}
