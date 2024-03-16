use std::default::Default;
use std::sync::Arc;
use crate::citadel::render::{Renderer};

use vulkano::VulkanLibrary;
use vulkano::{instance::Instance, device::physical::PhysicalDevice};
use vulkano::instance::InstanceCreateInfo;

#[derive(Debug)]
struct VulkanoRenderer {
    vulkano: Arc<Instance>,

    physical_devices: [Arc<PhysicalDevice>; 1],

}

impl Renderer for VulkanoRenderer {
    fn create(name: &str) -> VulkanoRenderer {
        let lib = VulkanLibrary::new().expect("ERROR: No vulkan library.");
        let mut instance_create_info = InstanceCreateInfo::default();
        instance_create_info.application_name = Option::from(name.to_string());
        let instance = Instance::new(lib, instance_create_info).expect("ERROR: Unable to create instance");

        let mut physical_devices = Instance::enumerate_physical_devices(&instance)
            .expect("ERROR: No physical device available");

        VulkanoRenderer {
            vulkano: instance,
            physical_devices: [physical_devices.next().unwrap(); 1]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::citadel::render::Renderer;
    use crate::vk::vk_citadel::VulkanoRenderer;

    #[test]
    fn test_instance_creation() {
        let a: dyn Renderer = VulkanoRenderer::create("Hello?");
        println!("{:?}", a);
    }
}