use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDeviceFeatures, Queue},
    Device,
};

use crate::physical_device::PhysicalDevice;

#[derive(Clone)]
pub struct LogicalDevice(Rc<InnerLogicalDevice>);

impl LogicalDevice {
    pub fn new(physical_device: PhysicalDevice) -> VkResult<Self> {
        let queue_priority = [1.0];

        let queue_create_info = DeviceQueueCreateInfo::default()
            .queue_family_index(physical_device.graphics_family_u32())
            .queue_priorities(&queue_priority);

        let queue_create_infos = [queue_create_info];

        let device_features = PhysicalDeviceFeatures::default();

        let create_info = DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features);

        let device = unsafe {
            physical_device.instance().instance().create_device(
                physical_device.device().clone(),
                &create_info,
                None,
            )?
        };

        let queue = unsafe { device.get_device_queue(physical_device.graphics_family_u32(), 0) };

        Ok(Self(Rc::new(InnerLogicalDevice {
            device,
            physical_device,
            queue,
        })))
    }

    pub fn device(&self) -> &Device {
        &self.0.device
    }

    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.0.physical_device
    }

    pub fn queue(&self) -> &Queue {
        &self.0.queue
    }
}

struct InnerLogicalDevice {
    device: Device,
    physical_device: PhysicalDevice,
    queue: Queue,
}

impl Drop for InnerLogicalDevice {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
