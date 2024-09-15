use std::{ffi::CStr, fmt, rc::Rc};

use ash::{
    prelude::VkResult,
    vk::{self, QueueFlags},
};

use crate::{instance::Instance, logical_device::REQUIRED_EXTENSIONS, surface::Surface};

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
            if let Ok(v) = QueueFamilyIndices::find_queue_families(
                &instance,
                physical_device.clone(),
                &surface,
            ) {
                if v.is_complete()
                    && check_device_extension_support(&instance, physical_device)
                        .map_err(PhysicalDeviceError::from)?
                {
                    return Ok(Self(Rc::new(InnerPhysicalDevice {
                        instance,
                        physical_device,
                        graphics_family: v.graphics_family.unwrap(),
                        present_family: v.present_family.unwrap(),
                    })));
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
}

struct InnerPhysicalDevice {
    instance: Instance,
    physical_device: vk::PhysicalDevice,
    graphics_family: usize,
    present_family: usize,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct QueueFamilyIndices {
    graphics_family: Option<usize>,
    present_family: Option<usize>,
}

impl QueueFamilyIndices {
    pub fn find_queue_families(
        instance: &Instance,
        device: vk::PhysicalDevice,
        surface: &Surface,
    ) -> VkResult<Self> {
        let queue_family = unsafe {
            instance
                .instance()
                .get_physical_device_queue_family_properties(device)
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
                        device,
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
