use std::sync::Arc;
use vulkano::format::Format;
use vulkano::image::{Image, ImageCreateInfo, ImageTiling, ImageType, ImageUsage, SampleCount};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::shader::spirv::{ ImageFormat};
use vulkano::sync::Sharing;
use crate::citadel::AbelVulkanoRenderer;

struct AbelImage {
    image_format: Format,
    image_size: (u32, u32, u32),
    image_name: String,
    image_use_for: ImageUsage
}

impl AbelImage {
    fn to_vulkano_image(self, renderer: AbelVulkanoRenderer) -> Arc<Image> {
        Image::new(
            Default::default(),
            ImageCreateInfo {
                flags: Default::default(),
                image_type: ImageType::Dim2d,
                format: self.image_format,
                view_formats: vec![],
                extent: [self.image_size.0, self.image_size.1, self.image_size.2],
                array_layers: 0,
                mip_levels: 0,
                samples: SampleCount::Sample1,
                tiling: ImageTiling::Optimal,
                usage: self.image_use_for,
                stencil_usage: None,
                sharing: Sharing::Exclusive,
                initial_layout: Default::default(),
                drm_format_modifiers: vec![],
                drm_format_modifier_plane_layouts: vec![],
                external_memory_handle_types: Default::default(),
                _ne: Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            }
        ).expect("Cannot create image.")
    }
}

#[cfg(test)]
mod test {
    use vulkano::format::Format;
    use vulkano::image::ImageUsage;
    use crate::citadel::AbelVulkanoRenderer;
    use crate::resource::AbelImage;

    #[test]
    fn test_image_creation() {
        let inst = AbelVulkanoRenderer::new("Hello?");

        let img = AbelImage {
            image_format: Format::R8G8B8A8_UINT,
            image_size: (1024, 1024, 1),
            image_name: "asdf".to_string(),
            image_use_for: ImageUsage::STORAGE,
        }.to_vulkano_image(inst);
    }
}