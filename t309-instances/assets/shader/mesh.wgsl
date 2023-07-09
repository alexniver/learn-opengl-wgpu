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

struct LightDirection {
    dir: vec3<f32>,
    color: vec4<f32>,
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    metallic: vec3<f32>,
}

struct LightDirectionArray {
    arr: array<LightDirection>,
}

struct LightPoint {
    pos: vec3<f32>,
    color: vec4<f32>,
    ambient: vec3<f32>,
    constant: f32,
    diffuse: vec3<f32>,
    linear: f32,
    metallic: vec3<f32>,
    quadratic: f32,
}

struct LightPointArray {
    arr: array<LightPoint>,
}

struct LightSpot {
    pos: vec3<f32>,
    front: vec3<f32>,
    color: vec4<f32>,
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    in_cutoff: f32,
    metallic: vec3<f32>,
    out_cutoff: f32,
}

struct LightSpotArray {
    arr: array<LightSpot>,
}

@group(0)@binding(0)
var<uniform> view: mat4x4<f32>;
@group(0)@binding(1)
var<uniform> proj: mat4x4<f32>;
@group(0)@binding(2)
var<uniform> camera_pos: vec3<f32>;

@vertex
fn vs_main(in: VertexIn, transform: TransformIT) -> VertexOut {
    let model = mat4x4<f32>(transform.t0, transform.t1, transform.t2, transform.t3);
    let it_model = mat3x3<f32>(transform.t4.xyz, transform.t5.xyz, transform.t6.xyz); // inverse transpose model

    var out: VertexOut;
    out.clip_pos = proj * view * model * vec4<f32>(in.pos, 1.0);
    //out.clip_pos = vec4<f32>(in.pos, 1.0);
    out.frag_pos = (model * vec4<f32>(in.pos, 1.0)).xyz;
    out.normal = it_model * in.normal;
    out.tex_coord = in.tex_coord;

    return out;
}


@group(1)@binding(0)
var<storage> light_direction_arr: LightDirectionArray;
@group(1)@binding(1)
var<storage> light_point_arr: LightPointArray;
@group(1)@binding(2)
var<storage> light_spot_arr: LightSpotArray;


@group(2)@binding(0)
var texture_sampler: sampler;
@group(2)@binding(1)
var texture_diffuse: texture_2d<f32>;
@group(2)@binding(2)
var<uniform> color: vec3<f32>;
@group(2)@binding(3)
var<uniform> shininess: f32;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    var l = vec3<f32>(0.0, 0.0, 0.0);
    let normal = normalize(in.normal);
    let view_dir = normalize(camera_pos - in.frag_pos);

    let tex_diffuse = textureSample(texture_diffuse, texture_sampler, in.tex_coord).rgb;

    for (var i: u32 = 0u; i < arrayLength(&light_direction_arr.arr); i = i + 1u) {
        l += do_light_direction(light_direction_arr.arr[i], normal, view_dir, tex_diffuse);
    }

    for (var i: u32 = 0u; i < arrayLength(&light_point_arr.arr); i = i + 1u) {
        l += do_light_point(light_point_arr.arr[i], normal, view_dir, tex_diffuse, in.frag_pos);
    }

    for (var i: u32 = 0u; i < arrayLength(&light_spot_arr.arr); i = i + 1u) {
        l += do_light_spot(light_spot_arr.arr[i], normal, tex_diffuse, in.frag_pos);
    }
    return vec4<f32>(l, 1.0);
}

fn do_light_direction(light_direction: LightDirection, normal: vec3<f32>, view_dir: vec3<f32>, tex_diffuse: vec3<f32>) -> vec3<f32> {
    if light_direction.color.a == 0.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let light_color = light_direction.color.rgb;

    let ambient = light_color * light_direction.ambient * tex_diffuse;

    let light_dir = normalize(-light_direction.dir);
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = light_color * light_direction.diffuse * (diff * tex_diffuse);

    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);

    return ambient + diffuse;
}

fn do_light_point(light_point: LightPoint, normal: vec3<f32>, view_dir: vec3<f32>, tex_diffuse: vec3<f32>, frag_pos: vec3<f32>) -> vec3<f32> {
    if light_point.color.a == 0.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let light_color = light_point.color.rgb;

    var ambient = light_color * light_point.ambient * tex_diffuse;

    let light_dir = normalize(light_point.pos - frag_pos);
    let diff = max(dot(normal, light_dir), 0.0);
    var diffuse = light_color * light_point.diffuse * (diff * tex_diffuse);

    let view_dir = normalize(camera_pos - frag_pos);
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);

    let len = length(light_point.pos - frag_pos);
    let attenuation = 1.0 / (light_point.constant + light_point.linear * len + light_point.quadratic * (len * len));

    ambient *= attenuation;
    diffuse *= attenuation;

    return ambient + diffuse;
}

fn do_light_spot(light_spot: LightSpot, normal: vec3<f32>, tex_diffuse: vec3<f32>, frag_pos: vec3<f32>) -> vec3<f32> {
    if light_spot.color.a == 0.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let light_dir = normalize(light_spot.pos - frag_pos);
    let light_color = light_spot.color.rgb;

    let theta = dot(light_dir, normalize(-light_spot.front));
    let epsilon = light_spot.in_cutoff - light_spot.out_cutoff;
    let intensity = clamp((theta - light_spot.out_cutoff) / epsilon, 0.0, 1.0);

    let ambient = light_color * light_spot.ambient * tex_diffuse;

    let diff = max(dot(normal, light_dir), 0.0);
    var diffuse = light_color * light_spot.diffuse * (diff * tex_diffuse);

    let view_dir = normalize(camera_pos - frag_pos);
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);

    diffuse *= intensity;

    return ambient + diffuse;
}
