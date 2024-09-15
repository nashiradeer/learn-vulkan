use std::{ffi::CStr, rc::Rc};

use ash::{
    prelude::VkResult,
    vk::{
        DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDeviceFeatures, Queue, KHR_SWAPCHAIN_NAME,
    },
    Device,
};

use crate::physical_device::PhysicalDevice;

pub static REQUIRED_EXTENSIONS: [&CStr; 1] = [KHR_SWAPCHAIN_NAME];

#[derive(Clone)]
#[allow(dead_code)]
pub struct LogicalDevice(Rc<InnerLogicalDevice>);

impl LogicalDevice {
    pub fn new(physical_device: PhysicalDevice) -> VkResult<Self> {
        let queue_priority = [1.0];
        let queue_family_indices = [
            physical_device.graphics_family_u32(),
            physical_device.present_family_u32(),
        ];

        let queue_create_infos = create_queue_create_infos(&queue_family_indices, &queue_priority);

        let device_features = PhysicalDeviceFeatures::default();

        let extensions = REQUIRED_EXTENSIONS.map(|s| s.as_ptr());

        let create_info = DeviceCreateInfo::default()
            .queue_create_infos(queue_create_infos.as_slice())
            .enabled_features(&device_features)
            .enabled_extension_names(&extensions);

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
}

fn create_queue_create_infos<'a>(
    indices: &'a [u32],
    queue_priority: &'a [f32],
) -> Vec<DeviceQueueCreateInfo<'a>> {
    let mut queue_create_infos = Vec::new();
    let mut processed_indices = Vec::new();

    for &index in indices {
        if !processed_indices.contains(&index) {
            let queue_create_info = DeviceQueueCreateInfo::default()
                .queue_family_index(index)
                .queue_priorities(&queue_priority);

            queue_create_infos.push(queue_create_info);
            processed_indices.push(index);
        }
    }

    queue_create_infos
}

struct InnerLogicalDevice {
    device: Device,

    #[allow(dead_code)]
    physical_device: PhysicalDevice,

    #[allow(dead_code)]
    queue: Queue,
}

impl Drop for InnerLogicalDevice {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
