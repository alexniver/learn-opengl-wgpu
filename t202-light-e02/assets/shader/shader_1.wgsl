struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) frag_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
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
    out.frag_pos = (model * vec4<f32>(in.pos, 1.0)).xyz;
    out.normal = in.normal;
    return out;
}

@group(1)@binding(0)
var<uniform> light_color: vec3<f32>;
@group(1)@binding(1)
var<uniform> model_color: vec3<f32>;
@group(1)@binding(2)
var<uniform> light_pos: vec3<f32>;
@group(1)@binding(3)
var<uniform> camera_pos: vec3<f32>;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let ambient_strength = 0.01;

    let normal = normalize(in.normal);
    let light_dir = normalize(light_pos - in.frag_pos);
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = diff * light_color;

    let specular_strength = 0.5;
    let view_dir = normalize(camera_pos - in.frag_pos);
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), 128.0);
    let specular = specular_strength * spec * light_color;

    return vec4<f32>((ambient_strength + diffuse + specular) * (light_color * model_color), 1.0);
}
