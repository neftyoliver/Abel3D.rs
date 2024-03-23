//struct AbelPipeline {}

use std::sync::Arc;
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer};
use vulkano::device::Device;
use crate::citadel::AbelVulkanoRenderer;

enum AbelInstructions {
    Copy(),

}

pub struct AbelInstructionBuilder {
    allocator: StandardCommandBufferAllocator,
    pub cmd_buff_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>
}

impl AbelInstructionBuilder {
    fn new(dev: Arc<Device>, renderer: &AbelVulkanoRenderer) -> AbelInstructionBuilder { //todo: should make it concurrent in future
        let allocator = StandardCommandBufferAllocator::new(dev.clone(), 
            StandardCommandBufferAllocatorCreateInfo::default()
        );

        let mut cmd_builder = AutoCommandBufferBuilder::primary(
            &allocator,
            renderer.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();
        
        AbelInstructionBuilder {
            allocator,
            cmd_buff_builder: cmd_builder
        }
    }

    //command function starts with cmd

    //cmd_open_window
    //cmd_copy_from_buffer
    //cmd_copy_to_buffer
    //cmd_copy_from_img
    //cmd_copy_to_img
    //cmd_set_pipeline
    //cmd_render
    //cmd_represent_to_window

}

#[cfg(test)]
mod test {
    use crate::citadel::AbelVulkanoRenderer;

    #[test]
    fn test_command_buffer() {
        let inst = AbelVulkanoRenderer::new("Hello?");


    }
}