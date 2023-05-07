use image::GenericImageView;

pub fn gen_texture(
    img_path: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (wgpu::TextureView, wgpu::Sampler) {
    let img_bytes = std::fs::read(img_path).expect("read huaji fail");
    let img = image::load_from_memory(&img_bytes).expect("load img from memory fail");

    let img_rgba = img.to_rgba8();
    let img_dim = img.dimensions();

    let texture_size = wgpu::Extent3d {
        width: img_dim.0,
        height: img_dim.1,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::ImageCopyTextureBase {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &img_rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * img_dim.0),
            rows_per_image: Some(img_dim.1),
        },
        texture_size,
    );

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Texture Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    (texture_view, texture_sampler)
}
