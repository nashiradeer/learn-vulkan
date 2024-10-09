use std::rc::Rc;

use ash::{
    khr::swapchain,
    prelude::VkResult,
    vk::{
        CompositeAlphaFlagsKHR, Extent2D, Image, ImageUsageFlags, PresentModeKHR, SharingMode,
        SurfaceFormatKHR, SwapchainCreateInfoKHR, SwapchainKHR,
    },
};

use crate::{
    logical_device::LogicalDevice, physical_device::PhysicalDevice, surface::Surface,
    window::Window,
};

#[derive(Clone)]
pub struct Swapchain(#[allow(dead_code)] Rc<InnerSwapchain>);

impl Swapchain {
    pub fn new(
        physical_device: PhysicalDevice,
        logical_device: LogicalDevice,
        surface: Surface,
        window: &Window,
    ) -> VkResult<Self> {
        let swapchain_support = physical_device.swapchain_support();

        let format = swapchain_support.choose_format().clone();
        let present_mode = swapchain_support.choose_present_mode();
        let extent = swapchain_support.choose_extent(window);

        let mut image_count = swapchain_support.capabilities.min_image_count + 1;

        if swapchain_support.capabilities.max_image_count > 0
            && image_count > swapchain_support.capabilities.max_image_count
        {
            image_count = swapchain_support.capabilities.max_image_count;
        }

        let mut swapchain_create_info = SwapchainCreateInfoKHR::default()
            .surface(surface.surface())
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(swapchain_support.capabilities.current_transform)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let queue_family_indices = [
            physical_device.graphics_family_u32(),
            physical_device.present_family_u32(),
        ];

        if physical_device.graphics_family_u32() != physical_device.present_family_u32() {
            swapchain_create_info = swapchain_create_info
                .image_sharing_mode(SharingMode::CONCURRENT)
                .queue_family_indices(&queue_family_indices);
        } else {
            swapchain_create_info =
                swapchain_create_info.image_sharing_mode(SharingMode::EXCLUSIVE);
        }

        let swapchain_instance = swapchain::Device::new(
            physical_device.instance().instance(),
            logical_device.device(),
        );

        let swapchain =
            unsafe { swapchain_instance.create_swapchain(&swapchain_create_info, None)? };

        let images = unsafe { swapchain_instance.get_swapchain_images(swapchain)? };

        Ok(Self(Rc::new(InnerSwapchain {
            physical_device,
            logical_device,
            surface,
            format,
            present_mode,
            extent,
            swapchain_instance,
            swapchain,
            images,
        })))
    }

    pub fn images(&self) -> &[Image] {
        &self.0.images
    }

    pub fn format(&self) -> SurfaceFormatKHR {
        self.0.format
    }

    pub fn extent(&self) -> Extent2D {
        self.0.extent
    }

    pub fn device(&self) -> &LogicalDevice {
        &self.0.logical_device
    }
}

struct InnerSwapchain {
    swapchain_instance: swapchain::Device,
    swapchain: SwapchainKHR,
    images: Vec<Image>,
    format: SurfaceFormatKHR,

    #[allow(dead_code)]
    present_mode: PresentModeKHR,

    #[allow(dead_code)]
    extent: Extent2D,

    #[allow(dead_code)]
    physical_device: PhysicalDevice,

    #[allow(dead_code)]
    logical_device: LogicalDevice,

    #[allow(dead_code)]
    surface: Surface,
}

impl Drop for InnerSwapchain {
    fn drop(&mut self) {
        unsafe {
            self.swapchain_instance
                .destroy_swapchain(self.swapchain, None);
        }
    }
}
