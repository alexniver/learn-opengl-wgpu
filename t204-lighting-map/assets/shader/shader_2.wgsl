struct VertexIn {
    @location(0) pos: vec3<f32>,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
}

struct Transform {
    @location(5) t0: vec4<f32>,
    @location(6) t1: vec4<f32>,
    @location(7) t2: vec4<f32>,
    @location(8) t3: vec4<f32>,
}

@group(0)@binding(0)
var<uniform> view: mat4x4<f32>;
@group(0)@binding(1)
var<uniform> proj: mat4x4<f32>;

@vertex
fn vs_main(in: VertexIn, transform: Transform) -> VertexOut {
    let model = mat4x4<f32>(transform.t0, transform.t1, transform.t2, transform.t3);
    var out: VertexOut;
    out.clip_pos = proj * view * model * vec4<f32>(in.pos, 1.0);
    return out;
}


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0);
}
