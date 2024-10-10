//! Controls the lifecycle of the debug layer.

use std::ffi::c_void;

use ash::{ext::debug_utils, vk};

/// Controls the lifecycle of the debug layer.
pub struct DebugLayer {
    pub instance: debug_utils::Instance,
    pub messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugLayer {
    /// Create a new debug layer.
    pub fn new(
        instance: debug_utils::Instance,
        callback: vk::PFN_vkDebugUtilsMessengerCallbackEXT,
    ) -> Result<Self, vk::Result> {
        let create_info = create_debug_messenger(callback);

        let messenger = unsafe { instance.create_debug_utils_messenger(&create_info, None)? };

        Ok(Self {
            instance,
            messenger,
        })
    }
}

impl Drop for DebugLayer {
    fn drop(&mut self) {
        unsafe {
            self.instance
                .destroy_debug_utils_messenger(self.messenger, None);
        }
    }
}

/// Create a new debug messenger with all message types and severities enabled.
pub fn create_debug_messenger<'a>(
    callback: vk::PFN_vkDebugUtilsMessengerCallbackEXT,
) -> vk::DebugUtilsMessengerCreateInfoEXT<'a> {
    vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(callback)
}

/// Print all messages with a severity of warning or higher.
pub unsafe extern "system" fn print_warnings(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _: *mut c_void,
) -> vk::Bool32 {
    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        println!(
            "validation layer: {}",
            callback_data
                .read()
                .message_as_c_str()
                .unwrap()
                .to_str()
                .unwrap()
        );
    }

    vk::TRUE
}
