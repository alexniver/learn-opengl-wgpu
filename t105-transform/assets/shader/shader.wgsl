struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
};

struct Transform {
    @location(5) t0: vec4<f32>,
    @location(6) t1: vec4<f32>,
    @location(7) t2: vec4<f32>,
    @location(8) t3: vec4<f32>,
};

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};


@vertex
fn vs_main(in: VertexIn, tran: Transform) -> VertexOut {
    let model = mat4x4<f32>(tran.t0, tran.t1, tran.t2, tran.t3);

    var out: VertexOut;
    out.clip_pos = model * vec4<f32>(in.pos, 1.0);
    out.tex_coord = in.tex_coord;
    return out;
}

@group(0)@binding(0)
var tex_sampler: sampler;
@group(0)@binding(1)
var tex_container: texture_2d<f32>;
@group(0)@binding(2)
var tex_huaji: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let container = textureSample(tex_container, tex_sampler, in.tex_coord);
    let huaji = textureSample(tex_huaji, tex_sampler, in.tex_coord);
    return mix(container, huaji, 0.5);
}
