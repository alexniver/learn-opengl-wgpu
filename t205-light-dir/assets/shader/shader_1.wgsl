struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) frag_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
}

struct Transform {
    @location(5) t0: vec4<f32>,
    @location(6) t1: vec4<f32>,
    @location(7) t2: vec4<f32>,
    @location(8) t3: vec4<f32>,
}

struct Light {
    dir: vec3<f32>,
    color: vec3<f32>,
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
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
    out.tex_coord = in.tex_coord;

    return out;
}


@group(1)@binding(0)
var<uniform> light: Light;
@group(2)@binding(0)
var<uniform> camera_pos: vec3<f32>;

@group(3)@binding(0)
var texture_sampler: sampler;
@group(3)@binding(1)
var texture_diffuse: texture_2d<f32>;
@group(3)@binding(2)
var texture_specular: texture_2d<f32>;
@group(3)@binding(3)
var<uniform> color: vec3<f32>;
@group(3)@binding(4)
var<uniform> shininess: f32;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let tex_diffuse = textureSample(texture_diffuse, texture_sampler, in.tex_coord).rgb;
    let tex_specular = textureSample(texture_specular, texture_sampler, in.tex_coord).rgb;

    let ambient = light.ambient * tex_diffuse;

    let normal = normalize(in.normal);
    let light_dir = normalize(-light.dir);
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = light.diffuse * (diff * tex_diffuse);

    let view_dir = normalize(camera_pos - in.frag_pos);
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    let specular = light.specular * (spec * tex_specular);

    return vec4<f32>(ambient + diffuse + specular, 1.0);
}
