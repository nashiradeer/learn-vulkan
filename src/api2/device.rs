use std::{error::Error, fmt};

use super::{Extensions, Instance, PropertiesConversionError, SwapchainSupportDetails};
use ash::{khr::surface, prelude::*, vk};

/// Represents a Vulkan physical and logical device.
pub struct Device<T: AsRef<Instance>> {
    /// The Vulkan instance.
    pub instance: T,
    /// The Vulkan physical device.
    pub physical: vk::PhysicalDevice,
    /// The graphics queue family index.
    pub graphics_family: u32,
    /// The present queue family index.
    pub present_family: u32,
    /// Details about what the swapchain supports.
    pub swapchain_support: SwapchainSupportDetails,
    /// The Vulkan logical device.
    pub logical: ash::Device,
    /// The Vulkan queue.
    pub queue: vk::Queue,
}

impl<T: AsRef<Instance>> Device<T> {
    /// Creates a new Vulkan device.
    pub fn new(
        instance: T,
        extensions: &Extensions,
        surface_instance: &surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> Result<Self, DeviceError> {
        let devices = unsafe {
            instance
                .as_ref()
                .enumerate_physical_devices()
                .map_err(DeviceError::from)?
        };

        if devices.is_empty() {
            return Err(DeviceError::NoDevices);
        }

        let mut detected = None;

        for physical_device in devices {
            if let Ok(v) = QueueFamilyIndices::find_queue_families(
                instance.as_ref(),
                physical_device,
                surface_instance,
                surface,
            ) {
                if v.is_complete()
                    && check_device_extension_support(
                        instance.as_ref(),
                        physical_device,
                        &Extensions::default(),
                    )
                    .map_err(DeviceError::from)?
                {
                    let local_swapchain_support = SwapchainSupportDetails::query_support(
                        surface_instance,
                        surface,
                        physical_device,
                    )?;

                    if !local_swapchain_support.formats.is_empty()
                        && !local_swapchain_support.present_modes.is_empty()
                    {
                        detected = Some((
                            physical_device,
                            v.graphics_family.unwrap() as u32,
                            v.present_family.unwrap() as u32,
                            local_swapchain_support,
                        ));

                        break;
                    }
                }
            }
        }

        let Some((physical, graphics_family, present_family, swapchain_support)) = detected else {
            return Err(DeviceError::NoSuitableDevices);
        };

        let queue_priority = [1.0];
        let queue_family_indices = [graphics_family, present_family];
        let queue_create_infos = create_queue_create_infos(&queue_family_indices, &queue_priority);
        let device_features = vk::PhysicalDeviceFeatures::default();

        let extensions_ptr = extensions.as_vec_ptr();

        let create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features)
            .enabled_extension_names(&extensions_ptr);

        let logical = unsafe {
            instance
                .as_ref()
                .create_device(physical, &create_info, None)
                .map_err(DeviceError::from)
        }?;

        let queue = unsafe { logical.get_device_queue(graphics_family, 0) };

        Ok(Self {
            instance,
            physical,
            graphics_family,
            present_family,
            swapchain_support,
            logical,
            queue,
        })
    }
}

/// Represents an error that occurred while creating a device.
#[derive(Debug)]
pub enum DeviceError {
    /// No devices were found.
    NoDevices,
    /// No suitable devices were found.
    NoSuitableDevices,
    /// An error occurred while converting extension properties.
    PropertiesConversion(PropertiesConversionError),
    /// A Vulkan error occurred.
    VulkanError(vk::Result),
}

impl From<PropertiesConversionError> for DeviceError {
    fn from(error: PropertiesConversionError) -> Self {
        DeviceError::PropertiesConversion(error)
    }
}

impl From<vk::Result> for DeviceError {
    fn from(result: vk::Result) -> Self {
        DeviceError::VulkanError(result)
    }
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoDevices => write!(f, "no devices found"),
            Self::NoSuitableDevices => write!(f, "no suitable devices found"),
            Self::VulkanError(e) => e.fmt(f),
            Self::PropertiesConversion(e) => e.fmt(f),
        }
    }
}

impl Error for DeviceError {}

/// Checks if a device supports the required extensions.
pub fn check_device_extension_support(
    instance: &ash::Instance,
    device: vk::PhysicalDevice,
    extensions: &Extensions,
) -> Result<bool, DeviceError> {
    let raw_available_extensions = unsafe {
        instance
            .enumerate_device_extension_properties(device)
            .map_err(DeviceError::from)
    }?;

    let available_extensions =
        Extensions::try_from(raw_available_extensions).map_err(DeviceError::from)?;

    let mut required_extensions = Extensions::from([vk::KHR_SWAPCHAIN_NAME]);
    required_extensions.extend_from_slice(extensions);

    Ok(required_extensions
        .iter()
        .all(|e| available_extensions.contains(e)))
}

/// Represents the queue family indices.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct QueueFamilyIndices {
    /// The graphics queue family index.
    graphics_family: Option<usize>,
    /// The present queue family index.
    present_family: Option<usize>,
}

impl QueueFamilyIndices {
    /// Finds the queue families.
    pub fn find_queue_families(
        instance: &ash::Instance,
        device: vk::PhysicalDevice,
        surface_instance: &surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Self> {
        let queue_family = unsafe { instance.get_physical_device_queue_family_properties(device) };

        let mut indices = Self::default();

        for (i, v) in queue_family.iter().enumerate() {
            if v.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(i);
            }

            if unsafe {
                surface_instance.get_physical_device_surface_support(device, i as u32, surface)
            }? {
                indices.present_family = Some(i);
            }
        }

        Ok(indices)
    }

    /// Checks if all queue families are set.
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

/// Helper function to create queue create infos.
///
/// This function checks for duplicate queue family indices and only adds them once.
pub fn create_queue_create_infos<'a>(
    indices: &'a [u32],
    queue_priority: &'a [f32],
) -> Vec<vk::DeviceQueueCreateInfo<'a>> {
    let mut queue_create_infos = Vec::new();
    let mut processed_indices = Vec::new();

    for &index in indices {
        if !processed_indices.contains(&index) {
            let queue_create_info = vk::DeviceQueueCreateInfo::default()
                .queue_family_index(index)
                .queue_priorities(&queue_priority);

            queue_create_infos.push(queue_create_info);
            processed_indices.push(index);
        }
    }

    queue_create_infos
}
