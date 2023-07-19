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
var texture_depth: texture_depth_multisampled_2d;
@group(0)@binding(1)
var<uniform> texture_size: vec2<u32>;
@group(0)@binding(2)
var<uniform> orth: u32;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let near = 0.1;
    let far = 100.0;
    let x = u32(in.tex_coord.x * f32(texture_size.x));
    let y = u32(in.tex_coord.y * f32(texture_size.y));
    let depth = textureLoad(texture_depth, vec2<u32>(x, y), 0);
    if orth != u32(0) {
        return vec4<f32>(vec3<f32>(depth), 1.0);
    } else {
        let r = (2.0 * near) / (far + near - depth * (far - near));
        return vec4<f32>(vec3<f32>(r), 1.0);
    }
}
