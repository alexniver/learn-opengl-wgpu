struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) pos_world: vec3<f32>,
}

struct TransformIT {
    @location(5) t0: vec4<f32>,
    @location(6) t1: vec4<f32>,
    @location(7) t2: vec4<f32>,
    @location(8) t3: vec4<f32>,
    @location(9) t4: vec4<f32>,
    @location(10) t5: vec4<f32>,
    @location(11) t6: vec4<f32>,
    @location(12) t7: vec4<f32>,
}

@group(0)@binding(0)
var<uniform> view_proj: mat4x4<f32>;
@group(0)@binding(1)
var<uniform> light_point_pos: vec3<f32>;
@group(0)@binding(2)
var<uniform> near_far: vec2<f32>;

@vertex
fn vs_main(in: VertexIn, transform: TransformIT) -> VertexOut {
    let model = mat4x4<f32>(transform.t0, transform.t1, transform.t2, transform.t3);
    //let it_model = mat3x3<f32>(transform.t4.xyz, transform.t5.xyz, transform.t6.xyz); // inverse transpose model
    var out: VertexOut;
    out.clip_pos = view_proj * model * vec4<f32>(in.pos, 1.0);
    out.pos_world = (model * vec4<f32>(in.pos, 1.0)).xyz;

    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let len = length(in.pos_world - light_point_pos);
    let v = (len - near_far.x) / (near_far.y - near_far.x);
    return vec4<f32>(v);
}
