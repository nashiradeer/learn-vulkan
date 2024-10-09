use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{Fence, FenceCreateFlags, FenceCreateInfo, Semaphore, SemaphoreCreateInfo},
};

use crate::logical_device::LogicalDevice;

pub struct SyncObjects(Rc<InnerSyncObjects>);

impl SyncObjects {
    pub fn new(logical_device: LogicalDevice) -> VkResult<Self> {
        let semaphore_info = SemaphoreCreateInfo::default();

        let image_available_semaphore = unsafe {
            logical_device
                .device()
                .create_semaphore(&semaphore_info, None)?
        };

        let render_finished_semaphore = unsafe {
            logical_device
                .device()
                .create_semaphore(&semaphore_info, None)?
        };

        let fence_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);

        let in_flight_fence = unsafe { logical_device.device().create_fence(&fence_info, None)? };

        Ok(Self(Rc::new(InnerSyncObjects {
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fence,
            logical_device,
        })))
    }

    pub fn wait_in_flight_fence(&self) -> VkResult<()> {
        let fences = [self.0.in_flight_fence];

        unsafe {
            self.0
                .logical_device
                .device()
                .wait_for_fences(&fences, true, u64::MAX)
        }
    }

    pub fn reset_in_flight_fence(&self) -> VkResult<()> {
        let fences = [self.0.in_flight_fence];

        unsafe { self.0.logical_device.device().reset_fences(&fences) }
    }

    pub fn image_available_semaphore(&self) -> &Semaphore {
        &self.0.image_available_semaphore
    }

    pub fn render_finished_semaphore(&self) -> &Semaphore {
        &self.0.render_finished_semaphore
    }

    pub fn in_flight_fence(&self) -> &Fence {
        &self.0.in_flight_fence
    }
}

struct InnerSyncObjects {
    image_available_semaphore: Semaphore,
    render_finished_semaphore: Semaphore,
    in_flight_fence: Fence,
    logical_device: LogicalDevice,
}

impl Drop for InnerSyncObjects {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .device()
                .destroy_semaphore(self.image_available_semaphore, None);
            self.logical_device
                .device()
                .destroy_semaphore(self.render_finished_semaphore, None);
            self.logical_device
                .device()
                .destroy_fence(self.in_flight_fence, None);
        }
    }
}
