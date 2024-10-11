use ash::{khr::surface, prelude::*, vk};

/// Details about what the swapchain supports.
#[derive(Clone, Default)]
pub struct SwapchainSupportDetails {
    /// The capabilities of the swapchain.
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    /// The formats of the swapchain.
    pub formats: Vec<vk::SurfaceFormatKHR>,
    /// The present modes of the swapchain.
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupportDetails {
    /// Query the swapchain support details.
    pub fn query_support(
        surface_instance: &surface::Instance,
        surface: vk::SurfaceKHR,
        device: vk::PhysicalDevice,
    ) -> VkResult<SwapchainSupportDetails> {
        let capabilities =
            unsafe { surface_instance.get_physical_device_surface_capabilities(device, surface)? };

        let formats =
            unsafe { surface_instance.get_physical_device_surface_formats(device, surface)? };

        let present_modes =
            unsafe { surface_instance.get_physical_device_surface_present_modes(device, surface)? };

        Ok(SwapchainSupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }

    /// Choose the format of the swapchain.
    pub fn choose_format(&self) -> &vk::SurfaceFormatKHR {
        for format in &self.formats {
            if format.format == vk::Format::B8G8R8A8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return format;
            }
        }

        &self.formats[0]
    }

    /// Choose the present mode of the swapchain.
    pub fn choose_present_mode(&self) -> vk::PresentModeKHR {
        for present_mode in &self.present_modes {
            if *present_mode == vk::PresentModeKHR::MAILBOX {
                return *present_mode;
            }
        }

        vk::PresentModeKHR::FIFO
    }

    /// Choose the extent of the swapchain.
    pub fn choose_extent(&self, width: u32, height: u32) -> vk::Extent2D {
        let mut current_extent = vk::Extent2D { width, height };

        current_extent.width = nalgebra::clamp(
            current_extent.width,
            self.capabilities.min_image_extent.width,
            self.capabilities.max_image_extent.width,
        );
        current_extent.height = nalgebra::clamp(
            current_extent.height,
            self.capabilities.min_image_extent.height,
            self.capabilities.max_image_extent.height,
        );

        current_extent
    }
}
