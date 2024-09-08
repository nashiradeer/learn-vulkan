use std::{fmt, rc::Rc};

use ash::vk::{self, QueueFlags};

use crate::instance::Instance;

#[derive(Clone)]
pub struct PhysicalDevice(Rc<InnerPhysicalDevice>);

impl PhysicalDevice {
    pub fn new(instance: Instance) -> Result<Self, PhysicalDeviceError> {
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
            if let Some(graphics_family) = find_queue_families(&instance, physical_device.clone()) {
                return Ok(Self(Rc::new(InnerPhysicalDevice {
                    graphics_family,
                    instance,
                    physical_device,
                })));
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

    pub fn graphics_family(&self) -> usize {
        self.0.graphics_family
    }

    pub fn graphics_family_u32(&self) -> u32 {
        self.0.graphics_family.try_into().unwrap()
    }
}

struct InnerPhysicalDevice {
    instance: Instance,
    physical_device: vk::PhysicalDevice,
    graphics_family: usize,
}

fn find_queue_families(instance: &Instance, device: vk::PhysicalDevice) -> Option<usize> {
    let queue_family = unsafe {
        instance
            .instance()
            .get_physical_device_queue_family_properties(device)
    };

    for (i, v) in queue_family.iter().enumerate() {
        if v.queue_flags.contains(QueueFlags::GRAPHICS) {
            return Some(i);
        }
    }

    None
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
