use ash::{
    ext::debug_utils,
    prelude::VkResult,
    vk::{
        self, Bool32, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateInfoEXT,
        DebugUtilsMessengerEXT,
    },
};
use std::{ffi::c_void, rc::Rc};

use crate::instance::Instance;

#[derive(Clone)]
pub struct DebugLayer(Rc<InnerDebugLayer>);

impl DebugLayer {
    pub fn new(instance: Instance) -> VkResult<Self> {
        let create_info = create_debug_messenger();

        let debug_instance = debug_utils::Instance::new(instance.entry(), instance.instance());
        let debug_messenger =
            unsafe { debug_instance.create_debug_utils_messenger(&create_info, None)? };

        Ok(Self(Rc::new(InnerDebugLayer {
            debug_instance,
            debug_messenger,
            instance,
        })))
    }

    pub fn instance(&self) -> &Instance {
        &self.0.instance
    }
}

struct InnerDebugLayer {
    instance: Instance,
    debug_instance: debug_utils::Instance,
    debug_messenger: DebugUtilsMessengerEXT,
}

impl Drop for InnerDebugLayer {
    fn drop(&mut self) {
        unsafe {
            self.debug_instance
                .destroy_debug_utils_messenger(self.debug_messenger, None);
        }
    }
}

pub fn create_debug_messenger<'a>() -> DebugUtilsMessengerCreateInfoEXT<'a> {
    DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | DebugUtilsMessageSeverityFlagsEXT::WARNING
                | DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(debug_callback))
}

unsafe extern "system" fn debug_callback(
    severity: DebugUtilsMessageSeverityFlagsEXT,
    _: DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
    _: *mut c_void,
) -> Bool32 {
    if severity >= DebugUtilsMessageSeverityFlagsEXT::WARNING {
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
