//! Contains the `Instance` struct and related types.

use std::{borrow::Borrow, ffi::CString, ops::Deref};

use super::Extensions;
use ash::{ext::debug_utils, vk};

mod builder;
mod debug_layer;
mod error;

pub use builder::*;
pub use debug_layer::*;
pub use error::*;

/// A Vulkan instance, debug layer, and entry.
pub struct Instance {
    /// The Vulkan instance.
    pub instance: ash::Instance,
    /// The Vulkan entry.
    pub entry: ash::Entry,
    /// The debug layer, if enabled.
    pub debug_layer: Option<DebugLayer>,
}

impl Instance {
    /// Create a new Vulkan instance with the given parameters.
    ///
    /// You can use the `InstanceBuilder` to create a new instance that's easier to configure and has default values.
    pub fn new(
        entry: ash::Entry,
        application_name: &str,
        application_version: u32,
        engine_name: &str,
        engine_version: u32,
        api_version: u32,
        extensions: Extensions,
        mut layers: Extensions,
        enable_debug_layer: bool,
        debug_callback: vk::PFN_vkDebugUtilsMessengerCallbackEXT,
    ) -> Result<Self, InstanceError> {
        let available_layers = Extensions::try_from(
            unsafe { entry.enumerate_instance_layer_properties() }.map_err(InstanceError::from)?,
        )
        .map_err(InstanceError::from)?;

        let validation_layers = get_validation_layers();

        if enable_debug_layer
            && validation_layers
                .iter()
                .all(|v| available_layers.contains(v))
        {
            return Err(InstanceError::NoValidationLayer);
        }

        let application_name = CString::new(application_name).map_err(InstanceError::from)?;
        let engine_name = CString::new(engine_name).map_err(InstanceError::from)?;

        let app_info = vk::ApplicationInfo::default()
            .application_name(&application_name)
            .application_version(application_version)
            .engine_name(&engine_name)
            .engine_version(engine_version)
            .api_version(api_version);

        let extensions_ptr = extensions.as_vec_ptr();

        let mut create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extensions_ptr);

        let mut debug_messenger;
        let layers_ptr;

        create_info = if enable_debug_layer {
            layers.append(&mut Vec::from(validation_layers));
            layers_ptr = layers.as_vec_ptr();

            debug_messenger = create_debug_messenger(debug_callback);

            create_info
                .enabled_layer_names(&layers_ptr)
                .push_next(&mut debug_messenger)
        } else {
            layers_ptr = layers.as_vec_ptr();
            create_info.enabled_layer_names(&layers_ptr)
        };

        if cfg!(target_os = "macos") {
            create_info = create_info.flags(vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR);
        }

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        let debug_layer = if enable_debug_layer {
            Some(DebugLayer::new(
                debug_utils::Instance::new(&entry, &instance),
                debug_callback,
            )?)
        } else {
            None
        };

        Ok(Self {
            instance,
            debug_layer,
            entry,
        })
    }

    /// Get the available extensions from the Vulkan entry.
    pub fn available_extensions(&self) -> Result<Extensions, InstanceError> {
        let extensions = unsafe { self.entry.enumerate_instance_extension_properties(None) }
            .map_err(InstanceError::from)?;

        Extensions::try_from(extensions).map_err(InstanceError::from)
    }

    /// Get the available layers from the Vulkan entry.
    pub fn available_layers(&self) -> Result<Extensions, InstanceError> {
        let layers = unsafe { self.entry.enumerate_instance_layer_properties() }
            .map_err(InstanceError::from)?;

        Extensions::try_from(layers).map_err(InstanceError::from)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if let Some(debug_layer) = self.debug_layer.take() {
            drop(debug_layer);
        }

        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

impl AsRef<ash::Instance> for Instance {
    fn as_ref(&self) -> &ash::Instance {
        &self.instance
    }
}

impl Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl Borrow<ash::Instance> for Instance {
    fn borrow(&self) -> &ash::Instance {
        &self.instance
    }
}

/// Get the validation layers to enable.
#[inline]
pub fn get_validation_layers() -> [CString; 1] {
    [CString::new("VK_LAYER_KHRONOS_validation").unwrap()]
}
