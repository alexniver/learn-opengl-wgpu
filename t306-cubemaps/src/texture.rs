use anyhow::Result;
use image::GenericImageView;

pub fn gen_texture_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Texture Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    })
}

pub fn gen_texture_sampler_skybox(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Texture Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    })
}

pub fn gen_texture_view(
    img_bytes: Vec<u8>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<wgpu::TextureView> {
    let img = image::load_from_memory(&img_bytes)?;
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

    Ok(texture.create_view(&wgpu::TextureViewDescriptor::default()))
}

pub fn gen_texture_skybox(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::TextureView {
    let img_src_arr = [
        "assets/texture/cubemap/posx.jpg",
        "assets/texture/cubemap/negx.jpg",
        "assets/texture/cubemap/posy.jpg",
        "assets/texture/cubemap/negy.jpg",
        "assets/texture/cubemap/posz.jpg",
        "assets/texture/cubemap/negz.jpg",
    ];

    let img_arr = img_src_arr
        .iter()
        .map(|path| image::load_from_memory(&std::fs::read(path).unwrap()).unwrap())
        .collect::<Vec<image::DynamicImage>>();

    let img_dim = img_arr[0].dimensions();

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Texture Skybox"),
        size: wgpu::Extent3d {
            width: img_dim.0,
            height: img_dim.1,
            depth_or_array_layers: img_arr.len() as _,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    for (idx, img) in img_arr.iter().enumerate() {
        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: idx as _,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &img.to_rgba8(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * img_dim.0),
                rows_per_image: Some(img_dim.1),
            },
            wgpu::Extent3d {
                width: img_dim.0,
                height: img_dim.1,
                depth_or_array_layers: 1,
            },
        );
    }

    texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("Texture View Skybox"),
        dimension: Some(wgpu::TextureViewDimension::Cube),
        ..Default::default()
    })
}

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub fn gen_texture_depth(
    device: &wgpu::Device,
    surface_config: &wgpu::SurfaceConfiguration,
) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: wgpu::Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
}
