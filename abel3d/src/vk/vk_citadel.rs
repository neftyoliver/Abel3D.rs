use std::default::Default;
use std::sync::Arc;
use crate::citadel::render::{Renderer};

use vulkano::VulkanLibrary;
use vulkano::{instance::Instance, device::physical::PhysicalDevice, device::Device};
use vulkano::device::{DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::InstanceCreateInfo;

#[derive(Debug)]
struct VulkanoRenderer {
    vulkano: Arc<Instance>,
    physical_devices: [Arc<PhysicalDevice>; 1],
    device: Arc<Device>,
    queue: Arc<Queue>
}

impl Renderer for VulkanoRenderer {
    fn create(name: &str) -> VulkanoRenderer {
        let lib = VulkanLibrary::new().expect("ERROR: No vulkan library.");
        let mut instance_create_info = InstanceCreateInfo::default();
        instance_create_info.application_name = Option::from(name.to_string());
        let instance = Instance::new(lib, instance_create_info).expect("ERROR: Unable to create instance");


        //Right now, just one simple physical device is all we need.
        let physical_devices = Instance::enumerate_physical_devices(&instance)
            .expect("ERROR: No physical device available").next().unwrap();

        let loc_queue_family_index = &physical_devices
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(x, y)| {
                print!("Queue family index[{}]: with features ", x);
                let result = y.queue_flags.contains(QueueFlags::GRAPHICS) && y.queue_flags.contains(QueueFlags::TRANSFER);
                if result {
                    println!("{:?} -> Used!", y.queue_flags);
                }
                result
            }).expect("ERROR: No queue family to use for rendering with transfer.");

        //We need queue create info before creating logical device since the queue is PART OF the device.

        let queue_create_info = QueueCreateInfo {
            flags: Default::default(),
            queue_family_index: *loc_queue_family_index as u32,
            queues: vec![1.0],
            ..Default::default()
        };

        //Logical device doesn't need to be more than just one.
        let device_create_info = DeviceCreateInfo {
            queue_create_infos: vec![queue_create_info],
            enabled_extensions: Default::default(),
            enabled_features: Default::default(),
            physical_devices: Default::default(),
            private_data_slot_request_count: 0,
            ..Default::default()
        };

        let cp_physical_device = physical_devices.clone();

        let (logical_device, mut queues) = Device::new(physical_devices, device_create_info)
            .unwrap();

        VulkanoRenderer {
            vulkano: instance,
            physical_devices: [cp_physical_device; 1],
            device: logical_device,
            queue:  queues.next().unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::citadel::render::Renderer;
    use crate::vk::vk_citadel::VulkanoRenderer;

    #[test]
    fn test_instance_creation() {
        let a = VulkanoRenderer::create("Hello?");
        println!("{:?}", a);
    }
}