use std::ptr;

use crate::{
    vulkan::{
        Error, INSTANCE_EXTENSIONS, Result,
        bindings::{
            VK_VERSION_1_0, VkApplicationInfo, VkInstance_T, VkInstanceCreateInfo,
            VkResult_VK_SUCCESS, VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
            VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, vkCreateInstance,
            vkDestroyInstance,
        },
        vk_make_version,
    },
    window::Context,
};

pub struct Instance {
    window_context: Context,
    instance: *mut VkInstance_T,
}

impl Instance {
    pub fn new(window_context: Context) -> Result<Self> {
        let app_name = b"Learn Vulkan\0";
        let engine_name = b"No Engine\0";
        let result;

        let app_info = VkApplicationInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pApplicationName: app_name.as_ptr() as *const i8,
            applicationVersion: vk_make_version(1, 0, 0),
            pEngineName: engine_name.as_ptr() as *const i8,
            engineVersion: vk_make_version(1, 0, 0),
            apiVersion: VK_VERSION_1_0,
            pNext: ptr::null(),
        };

        let create_info = VkInstanceCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            flags: 0,
            pApplicationInfo: &app_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: INSTANCE_EXTENSIONS.len() as u32,
            ppEnabledExtensionNames: INSTANCE_EXTENSIONS.as_ptr() as *const *const i8,
            pNext: ptr::null(),
        };

        let mut instance = ptr::null_mut();
        result = unsafe { vkCreateInstance(&create_info, ptr::null(), &mut instance) };
        if result != VkResult_VK_SUCCESS {
            return Err(Error::from(result));
        };

        Ok(Self {
            window_context,
            instance,
        })
    }

    pub fn window_context(&self) -> &Context {
        &self.window_context
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self.instance, ptr::null());
        }
    }
}
