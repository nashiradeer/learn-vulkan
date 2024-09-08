use std::{ffi::CString, rc::Rc};

use ash::{
    ext, khr,
    prelude::VkResult,
    vk::{ApplicationInfo, InstanceCreateFlags, InstanceCreateInfo, API_VERSION_1_0},
    Entry,
};

use crate::{
    debug_layer::create_debug_messenger,
    utils::{to_vec_cstring, to_vec_pointer},
    ENABLE_VALIDATION_LAYERS, VALIDATION_LAYERS,
};

#[derive(Clone)]
pub struct Instance(Rc<InnerInstance>);

impl Instance {
    pub fn new(
        entry: Entry,
        required_extensions: Vec<String>,
        application_name: &str,
        application_version: u32,
        engine_name: &str,
        engine_version: u32,
    ) -> VkResult<Self> {
        let application_name = CString::new(application_name).unwrap();
        let engine_name = CString::new(engine_name).unwrap();

        let app_info = ApplicationInfo::default()
            .application_name(&application_name)
            .application_version(application_version)
            .engine_name(&engine_name)
            .engine_version(engine_version)
            .api_version(API_VERSION_1_0);

        let required_extensions = to_vec_cstring(required_extensions);
        let extensions = get_extensions(&required_extensions);

        let mut create_info = InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extensions.as_slice());

        let validation_layers;
        let layers;
        let mut debug_messenger;

        if ENABLE_VALIDATION_LAYERS {
            validation_layers = to_vec_cstring(VALIDATION_LAYERS);
            debug_messenger = create_debug_messenger();
            layers = get_layers(&validation_layers);

            create_info = create_info
                .enabled_layer_names(&layers)
                .push_next(&mut debug_messenger);
        }

        if cfg!(target_os = "macos") {
            create_info = create_info.flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR);
        }

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        Ok(Self(Rc::new(InnerInstance { entry, instance })))
    }

    pub fn entry(&self) -> &Entry {
        &self.0.entry
    }

    pub fn instance(&self) -> &ash::Instance {
        &self.0.instance
    }
}

struct InnerInstance {
    entry: Entry,
    instance: ash::Instance,
}

impl Drop for InnerInstance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

fn get_extensions(base: &Vec<CString>) -> Vec<*const i8> {
    let mut extensions = to_vec_pointer(base);

    if cfg!(target_os = "macos") {
        extensions.push(khr::portability_enumeration::NAME.as_ptr());
    }

    if ENABLE_VALIDATION_LAYERS {
        extensions.push(ext::debug_utils::NAME.as_ptr());
    }

    extensions
}

fn get_layers(base: &Vec<CString>) -> Vec<*const i8> {
    to_vec_pointer(base)
}
