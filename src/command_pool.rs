use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{self, CommandPoolCreateFlags, CommandPoolCreateInfo},
};

use crate::{logical_device::LogicalDevice, physical_device::PhysicalDevice};

pub struct CommandPool(#[allow(dead_code)] Rc<InnerCommandPool>);

impl CommandPool {
    pub fn new(logical_device: LogicalDevice, physical_device: &PhysicalDevice) -> VkResult<Self> {
        let command_pool_create_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(physical_device.graphics_family_u32());

        let command_pool = unsafe {
            logical_device
                .device()
                .create_command_pool(&command_pool_create_info, None)?
        };

        Ok(Self(Rc::new(InnerCommandPool {
            command_pool,
            logical_device,
        })))
    }
}

struct InnerCommandPool {
    command_pool: vk::CommandPool,

    #[allow(dead_code)]
    logical_device: LogicalDevice,
}

impl Drop for InnerCommandPool {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .device()
                .destroy_command_pool(self.command_pool, None);
        }
    }
}
