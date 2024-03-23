use std::sync::Arc;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{StandardMemoryAllocator};
use vulkano::VulkanLibrary;
use crate::workflow::AbelInstructionBuilder;

#[derive(Debug)]
pub struct AbelVulkanoRenderer {
    pub vulkano: Arc<Instance>,
    pub device: Vec<(Arc<Device>, bool)>,
    pub queue: Arc<Queue>,
    pub allocator: Arc<StandardMemoryAllocator>

}

impl AbelVulkanoRenderer {
    pub fn new(name: &str) -> AbelVulkanoRenderer {
        let lib = VulkanLibrary::new().expect("ERROR: No vulkan library.");
        let mut instance_create_info = InstanceCreateInfo::default();
        instance_create_info.application_name = Option::from(name.to_string());
        let instance = Instance::new(lib, instance_create_info).expect("ERROR: Unable to create instance");


        //Right now, just one simple physical device is all we need.
        let physical_device = Instance::enumerate_physical_devices(&instance)
            .expect("ERROR: No physical device available").next().unwrap();

        let loc_queue_family_index = &physical_device
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

        let cp_physical_device = physical_device.clone();

        let (logical_device, mut queues) = Device::new(cp_physical_device.clone(), device_create_info)
            .unwrap();

        let memory_allocator = StandardMemoryAllocator::new_default(logical_device.clone());

        AbelVulkanoRenderer {
            vulkano: instance,
            device: vec!((logical_device, true)),
            queue: queues.next().unwrap(),
            allocator: Arc::new(memory_allocator)
        }
    }

    fn consume_commands(&self) {

    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_instance_creation() {
        //println!("{:?}", a);
    }
}