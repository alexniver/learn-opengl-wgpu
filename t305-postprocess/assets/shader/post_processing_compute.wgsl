@group(0) @binding(0)
var screen_texture_in: texture_2d<f32>;
@group(0) @binding(1)
var screen_texture_out: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2)
var<uniform> texture_size: vec2<u32>;


@compute
@workgroup_size(8, 8, 1)
fn cp_main(@builtin(global_invocation_id) idx: vec3<u32>) {
    if idx.x >= texture_size.x || idx.y >= texture_size.y {
        return;
    }
    let OFFSET: i32 = 1;

    var kernel: array<i32, 9> = array<i32, 9>(
        -1,
        -1,
        -1,
        -1,
        9,
        -1,
        -1,
        -1,
        -1,
    );

    var offset_arr: array<vec2<i32>, 9> = array<vec2<i32>, 9>(
        vec2<i32>(-OFFSET, OFFSET),
        vec2<i32>(0, OFFSET),
        vec2<i32>(OFFSET, OFFSET),
        vec2<i32>(-OFFSET, 0),
        vec2<i32>(0, 0),
        vec2<i32>(OFFSET, 0),
        vec2<i32>(-OFFSET, -OFFSET),
        vec2<i32>(0, -OFFSET),
        vec2<i32>(OFFSET, -OFFSET),
    );

    let tex_coord = vec2<i32>(i32(idx.x), i32(idx.y));

    var sample_tex_arr: array<vec3<f32>, 9>;
    for (var i = 0u; i < 9u; i ++) {
        sample_tex_arr[i] = textureLoad(screen_texture_in, tex_coord + offset_arr[i], 0).xyz;
    }

    var color = vec3<f32>(0.0);
    for (var i = 0u; i < 9u; i ++) {
        color += sample_tex_arr[i] * f32(kernel[i]);
    }

    textureStore(screen_texture_out, tex_coord, vec4<f32>(color, 1.0));
}
