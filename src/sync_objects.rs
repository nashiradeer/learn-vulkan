use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{Fence, FenceCreateFlags, FenceCreateInfo, Semaphore, SemaphoreCreateInfo},
};

use crate::logical_device::LogicalDevice;

pub struct SyncObjects(Rc<InnerSyncObjects>);

impl SyncObjects {
    pub fn new(logical_device: LogicalDevice, count: usize) -> VkResult<Self> {
        let semaphore_info = SemaphoreCreateInfo::default();

        let fence_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);

        let mut image_available_semaphores = Vec::with_capacity(count);
        let mut render_finished_semaphores = Vec::with_capacity(count);
        let mut in_flight_fences = Vec::with_capacity(count);

        for _ in 0..=count {
            unsafe {
                image_available_semaphores.push(
                    logical_device
                        .device()
                        .create_semaphore(&semaphore_info, None)?,
                );

                render_finished_semaphores.push(
                    logical_device
                        .device()
                        .create_semaphore(&semaphore_info, None)?,
                );

                in_flight_fences.push(logical_device.device().create_fence(&fence_info, None)?);
            }
        }

        Ok(Self(Rc::new(InnerSyncObjects {
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            logical_device,
        })))
    }

    pub fn wait_in_flight_fence(&self, index: usize) -> VkResult<()> {
        let fences = [self.0.in_flight_fences[index]];

        unsafe {
            self.0
                .logical_device
                .device()
                .wait_for_fences(&fences, true, u64::MAX)
        }
    }

    pub fn reset_in_flight_fence(&self, index: usize) -> VkResult<()> {
        let fences = [self.0.in_flight_fences[index]];

        unsafe { self.0.logical_device.device().reset_fences(&fences) }
    }

    pub fn image_available_semaphore(&self, index: usize) -> &Semaphore {
        &self.0.image_available_semaphores[index]
    }

    pub fn render_finished_semaphore(&self, index: usize) -> &Semaphore {
        &self.0.render_finished_semaphores[index]
    }

    pub fn in_flight_fence(&self, index: usize) -> &Fence {
        &self.0.in_flight_fences[index]
    }
}

struct InnerSyncObjects {
    image_available_semaphores: Vec<Semaphore>,
    render_finished_semaphores: Vec<Semaphore>,
    in_flight_fences: Vec<Fence>,
    logical_device: LogicalDevice,
}

impl Drop for InnerSyncObjects {
    fn drop(&mut self) {
        unsafe {
            for semaphore in self.image_available_semaphores.iter() {
                self.logical_device
                    .device()
                    .destroy_semaphore(*semaphore, None);
            }

            for semaphore in self.render_finished_semaphores.iter() {
                self.logical_device
                    .device()
                    .destroy_semaphore(*semaphore, None);
            }

            for fence in self.in_flight_fences.iter() {
                self.logical_device.device().destroy_fence(*fence, None);
            }
        }
    }
}
