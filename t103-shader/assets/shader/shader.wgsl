struct VertexInput {
    @location(0) pos: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
};

struct TimeUniform {
    total_time: f32,
};

@group(0)@binding(0)
var<uniform> time: TimeUniform;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_pos = vec4<f32>(in.pos, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(sin(time.total_time), cos(time.total_time), sin(-time.total_time), 1.0);
}
