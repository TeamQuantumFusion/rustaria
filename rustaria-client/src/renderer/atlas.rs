use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

use image::{EncodableLayout, RgbaImage};
use rectangle_pack::{contains_smallest_box, GroupedRectsToPlace, pack_rects, RectanglePackError, RectanglePackOk, RectToInsert, TargetBin, volume_heuristic};
use tracing::{debug, info};
use wgpu::{BindGroup, BindGroupLayout, Device, Extent3d, Queue, Sampler, Texture, TextureView};

pub struct Atlas<I: Debug + Hash + Ord + Clone> {
    image_locations: HashMap<I, AtlasLocation>,
    texture: Texture,
    sampler: Sampler,
    texture_view: TextureView,
    pub layout: BindGroupLayout,
    pub group: BindGroup,
}

impl<I: Debug + Hash + Ord + Clone> Atlas<I> {
    pub fn new(queue: &Queue, device: &Device, images: Vec<(I, RgbaImage)>) -> Atlas<I> {
        let (atlas_images, width, height) = Self::pack_images(&images);

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("tile_texture"),
            }
        );

        let mut image_locations = HashMap::new();
        for (pos, (_, image_size)) in atlas_images.packed_locations() {
            info!("yuor mom");
            let (identifier, image) = images.get(*pos).unwrap();

            image_locations.insert(identifier.clone(), AtlasLocation {
                x: image_size.x() as f32 / width as f32,
                y: image_size.y() as f32 / height as f32,
                width: image_size.width() as f32 / width as f32,
                height: image_size.height() as f32 / height as f32,
            });


            queue.write_texture(
                // Tells wgpu where to copy the pixel data
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                // The actual pixel data
                image.as_bytes(),
                // The layout of the texture
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * image_size.width()),
                    rows_per_image: std::num::NonZeroU32::new(image_size.height()),
                },
                Extent3d {
                    width: image_size.width(),
                    height: image_size.height(),
                    depth_or_array_layers: 1,
                },
            );
        }



        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        );

        let group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );


        Atlas {
            image_locations,
            texture,
            sampler,
            texture_view,
            layout,
            group,
        }
    }

    fn pack_images(images: &[(I, RgbaImage)]) -> (RectanglePackOk<usize, i32>, u32, u32) {
        debug!(target: "client.render.atlas", "Packing {} images.", images.len());
        let mut rects_to_place: GroupedRectsToPlace<usize, ()> = GroupedRectsToPlace::new();

        for (id, (_, image)) in images.iter().enumerate() {
            rects_to_place.push_rect(
                id,
                None,
                RectToInsert::new(image.width(), image.height(), 1),
            );
        }

        let mut atlas_w = 32u32;
        let mut atlas_h = 32u32;
        loop {
            let mut target_bins = BTreeMap::new();
            target_bins.insert(1, TargetBin::new(atlas_w, atlas_h, 1));
            match pack_rects(
                &rects_to_place,
                &mut target_bins,
                &volume_heuristic,
                &contains_smallest_box,
            ) {
                Ok(placement) => {
                    return (placement, atlas_w, atlas_h);
                }
                Err(err) => {
                    match err {
                        RectanglePackError::NotEnoughBinSpace => {
                            if atlas_h > atlas_w {
                                atlas_w <<= 1;
                            } else {
                                atlas_h <<= 1;
                            }
                            info!(target: "client.render.atlas", "Resized Atlas to {}x{}", atlas_w, atlas_h);
                        }
                    }
                }
            };
        };
    }
}

pub struct AtlasLocation {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}