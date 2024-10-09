use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{self, ShaderModuleCreateInfo},
};

use crate::logical_device::LogicalDevice;

pub struct ShaderModule(Rc<InnerShaderModule>);

impl ShaderModule {
    pub fn new(logical_device: LogicalDevice, shader: &[u32]) -> VkResult<Self> {
        let create_info = ShaderModuleCreateInfo::default().code(shader);

        let shader_module = unsafe {
            logical_device
                .device()
                .create_shader_module(&create_info, None)?
        };

        Ok(Self(Rc::new(InnerShaderModule {
            shader_module,
            logical_device,
        })))
    }

    pub fn shader_module(&self) -> &vk::ShaderModule {
        &self.0.shader_module
    }
}

struct InnerShaderModule {
    shader_module: vk::ShaderModule,

    #[allow(dead_code)]
    logical_device: LogicalDevice,
}

impl Drop for InnerShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .device()
                .destroy_shader_module(self.shader_module, None);
        }
    }
}
