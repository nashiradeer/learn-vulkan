use std::{ffi::CStr, fmt, rc::Rc};

use ash::{
    prelude::VkResult,
    vk::{
        self, ColorSpaceKHR, Extent2D, PresentModeKHR, QueueFlags, SurfaceCapabilitiesKHR,
        SurfaceFormatKHR,
    },
};
use nalgebra::clamp;

use crate::{
    instance::Instance, logical_device::REQUIRED_EXTENSIONS, surface::Surface, window::Window,
};

#[derive(Clone)]
pub struct PhysicalDevice(Rc<InnerPhysicalDevice>);

impl PhysicalDevice {
    pub fn new(instance: Instance, surface: &Surface) -> Result<Self, PhysicalDeviceError> {
        let devices = unsafe {
            instance
                .instance()
                .enumerate_physical_devices()
                .map_err(PhysicalDeviceError::from)?
        };

        if devices.is_empty() {
            return Err(PhysicalDeviceError::NoDevices);
        }

        for physical_device in devices {
            if let Ok(v) =
                QueueFamilyIndices::find_queue_families(&instance, &physical_device, &surface)
            {
                if v.is_complete()
                    && check_device_extension_support(&instance, physical_device)
                        .map_err(PhysicalDeviceError::from)?
                {
                    let swapchain_support =
                        SwapchainSupportDetails::query_support(&surface, &physical_device)?;

                    if !swapchain_support.formats.is_empty()
                        && !swapchain_support.present_modes.is_empty()
                    {
                        return Ok(Self(Rc::new(InnerPhysicalDevice {
                            instance,
                            physical_device,
                            graphics_family: v.graphics_family.unwrap(),
                            present_family: v.present_family.unwrap(),
                            swapchain_support,
                        })));
                    }
                }
            }
        }

        Err(PhysicalDeviceError::NoSuitableDevices)
    }

    pub fn device(&self) -> &vk::PhysicalDevice {
        &self.0.physical_device
    }

    pub fn instance(&self) -> &Instance {
        &self.0.instance
    }

    pub fn graphics_family_u32(&self) -> u32 {
        self.0.graphics_family.try_into().unwrap()
    }

    pub fn present_family_u32(&self) -> u32 {
        self.0.present_family.try_into().unwrap()
    }

    pub fn swapchain_support(&self) -> &SwapchainSupportDetails {
        &self.0.swapchain_support
    }
}

struct InnerPhysicalDevice {
    instance: Instance,
    physical_device: vk::PhysicalDevice,
    graphics_family: usize,
    present_family: usize,
    swapchain_support: SwapchainSupportDetails,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct QueueFamilyIndices {
    graphics_family: Option<usize>,
    present_family: Option<usize>,
}

impl QueueFamilyIndices {
    pub fn find_queue_families(
        instance: &Instance,
        device: &vk::PhysicalDevice,
        surface: &Surface,
    ) -> VkResult<Self> {
        let queue_family = unsafe {
            instance
                .instance()
                .get_physical_device_queue_family_properties(*device)
        };

        let mut indices = Self::default();

        for (i, v) in queue_family.iter().enumerate() {
            if v.queue_flags.contains(QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(i);
            }

            if unsafe {
                surface
                    .surface_instance()
                    .get_physical_device_surface_support(
                        *device,
                        i as u32,
                        surface.surface().clone(),
                    )
            }? {
                indices.present_family = Some(i);
            }
        }

        Ok(indices)
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

fn check_device_extension_support(
    instance: &Instance,
    device: vk::PhysicalDevice,
) -> VkResult<bool> {
    let available_extensions = unsafe {
        instance
            .instance()
            .enumerate_device_extension_properties(device)
    }?;

    let supported = REQUIRED_EXTENSIONS.iter().all(|v| {
        available_extensions.iter().any(|extension| {
            let name = unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) };
            *v == name
        })
    });

    Ok(supported)
}

pub struct SwapchainSupportDetails {
    #[allow(dead_code)]
    pub capabilities: SurfaceCapabilitiesKHR,

    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>,
}

impl SwapchainSupportDetails {
    pub fn query_support(
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
    ) -> VkResult<SwapchainSupportDetails> {
        let capabilities = unsafe {
            surface
                .surface_instance()
                .get_physical_device_surface_capabilities(*physical_device, surface.surface())?
        };

        let formats = unsafe {
            surface
                .surface_instance()
                .get_physical_device_surface_formats(*physical_device, surface.surface())?
        };

        let present_modes = unsafe {
            surface
                .surface_instance()
                .get_physical_device_surface_present_modes(*physical_device, surface.surface())?
        };

        Ok(SwapchainSupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }

    pub fn choose_format(&self) -> &SurfaceFormatKHR {
        for format in &self.formats {
            if format.format == vk::Format::B8G8R8A8_SRGB
                && format.color_space == ColorSpaceKHR::SRGB_NONLINEAR
            {
                return format;
            }
        }

        &self.formats[0]
    }

    pub fn choose_present_mode(&self) -> PresentModeKHR {
        for present_mode in &self.present_modes {
            if *present_mode == PresentModeKHR::MAILBOX {
                return *present_mode;
            }
        }

        PresentModeKHR::FIFO
    }

    pub fn choose_extent(&self, window: &Window) -> Extent2D {
        let size = window.get_framebuffer_size();
        let mut current_extent = Extent2D {
            width: size.0 as u32,
            height: size.1 as u32,
        };

        current_extent.width = clamp(
            current_extent.width,
            self.capabilities.min_image_extent.width,
            self.capabilities.max_image_extent.width,
        );
        current_extent.height = clamp(
            current_extent.height,
            self.capabilities.min_image_extent.height,
            self.capabilities.max_image_extent.height,
        );

        current_extent
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalDeviceError {
    Vulkan(vk::Result),
    NoDevices,
    NoSuitableDevices,
}

impl From<vk::Result> for PhysicalDeviceError {
    fn from(value: vk::Result) -> Self {
        Self::Vulkan(value)
    }
}

impl fmt::Display for PhysicalDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Vulkan(e) => e.fmt(f),
            Self::NoDevices => write!(f, "failed to find GPUs with Vulkan support!"),
            Self::NoSuitableDevices => write!(f, "failed to find a suitable GPU!"),
        }
    }
}
