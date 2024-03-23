
use std::sync::{Arc};
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::format::Format;
use vulkano::image::{Image, ImageCreateInfo, ImageTiling, ImageType, ImageUsage, SampleCount};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::sync::Sharing;
use crate::citadel::AbelVulkanoRenderer;

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct AbelVertex3D {
    #[format(R64G64B64_SFLOAT)]
    position: [f64; 3]
}

impl AbelVertex3D {
    fn from(x: f64, y: f64, z: f64) -> AbelVertex3D {
        AbelVertex3D {
            position: [x, y, z]
        }
    }
}

struct AbelPolygon3DStatic {
    pub polygon_data: Vec<AbelVertex3D>
}

impl AbelPolygon3DStatic {
    fn to_vulkano_buffer(self, renderer: &AbelVulkanoRenderer) -> Subbuffer<[AbelVertex3D]> {

        Buffer::from_iter(
            renderer.allocator.clone(),

            BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            self.polygon_data
        ).expect("FATAL: error unwrapping vertex buffer")
    }
}


struct AbelImage {
    image_format: Format,
    image_size: (u32, u32, u32),
    image_use_for: ImageUsage
}

impl AbelImage {
    /*fn load_from()  {

    }*/

    fn to_vulkano_image(self, renderer: AbelVulkanoRenderer) -> Arc<Image> {
        Image::new(
            renderer.allocator.clone(),
            ImageCreateInfo {
                flags: Default::default(),
                image_type: ImageType::Dim2d,
                format: self.image_format,
                view_formats: vec![],
                extent: [self.image_size.0, self.image_size.1, self.image_size.2],
                array_layers: 1,
                mip_levels: 1,
                samples: SampleCount::Sample1,
                tiling: ImageTiling::Optimal,
                usage: self.image_use_for,
                stencil_usage: None,
                sharing: Sharing::Exclusive,
                initial_layout: Default::default(),
                drm_format_modifiers: vec![],
                drm_format_modifier_plane_layouts: vec![],
                external_memory_handle_types: Default::default(),
                ..Default::default()
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
    use crate::resource::{AbelImage, AbelPolygon3DStatic, AbelVertex3D};

    #[test]
    fn test_polygon_creation() {
        let inst = AbelVulkanoRenderer::new("Hello?");

        let _poly = AbelPolygon3DStatic {
            polygon_data: vec![
                AbelVertex3D::from(1.0,  1.0,  0.0),
                AbelVertex3D::from(1.0, -1.0,  0.0),
                AbelVertex3D::from(-1.0, -1.0,  0.0),

                AbelVertex3D::from(-1.0, -1.0,  0.0),
                AbelVertex3D::from(-1.0,  1.0,  0.0),
                AbelVertex3D::from(1.0,  1.0,  0.0)
            ],
        }.to_vulkano_buffer(&inst);
    }

    #[test]
    fn test_image_creation() {
        let inst = AbelVulkanoRenderer::new("Hello?");

        let _img = AbelImage {
            image_format: Format::R8G8B8A8_UINT,
            image_size: (1024, 1024, 1),
            image_use_for: ImageUsage::STORAGE,
        }.to_vulkano_image(inst);
    }
}