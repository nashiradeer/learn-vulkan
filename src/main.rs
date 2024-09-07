use std::ffi::{c_void, CString};

use ash::{
    ext::{self, debug_utils},
    khr,
    vk::{
        self, ApplicationInfo, Bool32, DebugUtilsMessageSeverityFlagsEXT,
        DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCallbackDataEXT,
        DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT, InstanceCreateFlags,
        InstanceCreateInfo,
    },
    Entry, Instance,
};
use glfw::{fail_on_errors, ClientApiHint, Glfw, PWindow, WindowHint, WindowMode};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

#[cfg(debug_assertions)]
const ENABLE_VALIDATION_LAYERS: bool = true;

#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

fn main() {
    let mut app = HelloTriangleApplication::default();
    app.run();
}

#[derive(Default)]
struct HelloTriangleApplication {
    window: Option<PWindow>,
    glfw: Option<Glfw>,
    instance: Option<Instance>,
    entry: Option<Entry>,
    debug_instance: Option<debug_utils::Instance>,
    debug_messsenger: Option<DebugUtilsMessengerEXT>,
}

impl HelloTriangleApplication {
    pub fn run(&mut self) {
        self.init_window();
        self.init_vulkan();
        self.main_loop();
        self.cleanup();
    }

    fn init_window(&mut self) {
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        glfw.window_hint(WindowHint::Resizable(false));

        let (window, _events) = glfw
            .create_window(WIDTH, HEIGHT, "Vulkan", WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        self.glfw = Some(glfw);
        self.window = Some(window);
    }

    fn init_vulkan(&mut self) {
        let entry = unsafe { Entry::load().unwrap() };
        self.entry = Some(entry);

        self.create_instance();
        self.setup_debug_messenger();
    }

    fn main_loop(&mut self) {
        let glfw = self.glfw.as_mut().unwrap();
        let window = self.window.as_mut().unwrap();

        while !window.should_close() {
            glfw.poll_events();
        }
    }

    fn cleanup(&mut self) {
        if ENABLE_VALIDATION_LAYERS {
            unsafe {
                self.debug_instance
                    .take()
                    .unwrap()
                    .destroy_debug_utils_messenger(self.debug_messsenger.take().unwrap(), None)
            };
        }

        unsafe { self.instance.take().unwrap().destroy_instance(None) };
        _ = self.entry.take().unwrap();
        _ = self.window.take().unwrap();
        _ = self.glfw.take().unwrap();
    }

    fn create_instance(&mut self) {
        if ENABLE_VALIDATION_LAYERS && !self.check_validation_layer_support() {
            panic!("validation layers requested, but not available!");
        }

        let entry = self.entry.as_ref().unwrap();
        let glfw = self.glfw.as_ref().unwrap();

        let application_name = CString::new("Hello Triangle").unwrap();
        let engine_name = CString::new("No Engine").unwrap();

        let app_info = ApplicationInfo::default()
            .application_name(&application_name)
            .application_version(vk::make_api_version(1, 0, 0, 0))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(1, 0, 0, 0))
            .api_version(vk::API_VERSION_1_0);

        println!("available extensions:");
        let extensions = unsafe { entry.enumerate_instance_extension_properties(None) }.unwrap();
        for extension in extensions {
            println!(
                "  {}",
                extension
                    .extension_name_as_c_str()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
        }

        let required_extensions = glfw
            .get_required_instance_extensions()
            .unwrap()
            .into_iter()
            .map(|s| CString::new(s).unwrap())
            .collect::<Vec<_>>();

        let mut required_extensions_ptr = required_extensions
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<_>>();

        if cfg!(target_os = "macos") {
            required_extensions_ptr.push(khr::portability_enumeration::NAME.as_ptr());
        }

        if ENABLE_VALIDATION_LAYERS {
            required_extensions_ptr.push(ext::debug_utils::NAME.as_ptr());
        }

        let mut create_info = InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(required_extensions_ptr.as_slice());

        if cfg!(target_os = "macos") {
            create_info = create_info.flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR);
        }

        let validation_layers = VALIDATION_LAYERS
            .into_iter()
            .map(|s| CString::new(s).unwrap())
            .collect::<Vec<_>>();

        let validation_layer_ptr = validation_layers
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<_>>();

        let mut debug_messenger = create_debug_messenger();

        if ENABLE_VALIDATION_LAYERS {
            create_info = create_info.enabled_layer_names(&validation_layer_ptr);

            create_info = create_info.push_next(&mut debug_messenger);
        }

        let instance = unsafe { entry.create_instance(&create_info, None) }.unwrap();

        self.instance = Some(instance);
    }

    fn check_validation_layer_support(&self) -> bool {
        let layers = unsafe {
            self.entry
                .as_ref()
                .unwrap()
                .enumerate_instance_layer_properties()
        }
        .unwrap();

        for required_layer in VALIDATION_LAYERS {
            let mut layer_found = false;

            for layer in layers.iter() {
                if layer.layer_name_as_c_str().unwrap().to_str().unwrap() == required_layer {
                    layer_found = true;
                    break;
                }
            }

            if !layer_found {
                return false;
            }
        }

        true
    }

    fn setup_debug_messenger(&mut self) {
        if !ENABLE_VALIDATION_LAYERS {
            return;
        }

        let instance = self.instance.as_ref().unwrap();
        let entry = self.entry.as_ref().unwrap();

        let create_info = create_debug_messenger();

        let debug_utils = debug_utils::Instance::new(entry, instance);
        let debug_messenger =
            unsafe { debug_utils.create_debug_utils_messenger(&create_info, None) }.unwrap();

        self.debug_instance = Some(debug_utils);
        self.debug_messsenger = Some(debug_messenger);
    }
}

fn create_debug_messenger<'a>() -> DebugUtilsMessengerCreateInfoEXT<'a> {
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
