//! Builder for creating a new [Instance].

use ash::vk::{self, make_api_version};

use super::{print_warnings, Extensions, Instance, InstanceBuilderError};

/// Builder for creating a new [Instance].
#[derive(Clone, Default)]
pub struct InstanceBuilder {
    /// The name of the application.
    pub application_name: Option<String>,
    /// The version of the application.
    pub application_version: Option<u32>,
    /// The name of the engine.
    pub engine_name: Option<String>,
    /// The version of the engine.
    pub engine_version: Option<u32>,
    /// The extensions to enable.
    pub extensions: Option<Extensions>,
    /// The layers to enable.
    pub layers: Option<Extensions>,
    /// The Vulkan entry.
    pub entry: Option<ash::Entry>,
    /// Whether to enable the debug layer.
    pub enable_debug_layer: bool,
    /// The debug callback for the debug layer.
    pub debug_callback: Option<vk::PFN_vkDebugUtilsMessengerCallbackEXT>,
}

impl InstanceBuilder {
    /// Get the available extensions from the Vulkan entry.
    pub fn available_extensions(&self) -> Result<Extensions, InstanceBuilderError> {
        let entry = self
            .entry
            .as_ref()
            .ok_or(InstanceBuilderError::NoVulkanEntry)?;

        Extensions::try_from(
            unsafe { entry.enumerate_instance_extension_properties(None) }
                .map_err(InstanceBuilderError::from)?,
        )
        .map_err(InstanceBuilderError::from)
    }

    /// Get the available layers from the Vulkan entry.
    pub fn available_layers(&self) -> Result<Extensions, InstanceBuilderError> {
        let entry = self
            .entry
            .as_ref()
            .ok_or(InstanceBuilderError::NoVulkanEntry)?;

        Extensions::try_from(
            unsafe { entry.enumerate_instance_layer_properties() }
                .map_err(InstanceBuilderError::from)?,
        )
        .map_err(InstanceBuilderError::from)
    }

    /// Set the name of the application.
    pub fn application_name(mut self, name: &str) -> Self {
        self.application_name = Some(name.to_owned());
        self
    }

    /// Set the version of the application.
    pub fn application_version(mut self, version: u32) -> Self {
        self.application_version = Some(version);
        self
    }

    /// Set the name of the engine.
    pub fn engine_name(mut self, name: &str) -> Self {
        self.engine_name = Some(name.to_owned());
        self
    }

    /// Set the version of the engine.
    pub fn engine_version(mut self, version: u32) -> Self {
        self.engine_version = Some(version);
        self
    }

    /// Set the extensions to enable.
    pub fn extensions(mut self, extensions: Extensions) -> Self {
        self.extensions = Some(extensions);
        self
    }

    /// Set the layers to enable.
    pub fn layers(mut self, layers: Extensions) -> Self {
        self.layers = Some(layers);
        self
    }

    /// Set the Vulkan entry.
    pub fn entry(mut self, entry: ash::Entry) -> Self {
        self.entry = Some(entry);
        self
    }

    /// Enable the debug layer.
    pub fn enable_debug_layer(mut self, enable: bool) -> Self {
        self.enable_debug_layer = enable;
        self
    }

    /// Set the debug callback for the debug layer.
    pub fn debug_callback(mut self, callback: vk::PFN_vkDebugUtilsMessengerCallbackEXT) -> Self {
        self.debug_callback = Some(callback);
        self
    }

    /// Build the [Instance].
    pub fn build(mut self) -> Result<Instance, InstanceBuilderError> {
        let application_name = self
            .application_name
            .take()
            .unwrap_or("Dragonlaze Application".to_owned());
        let application_version = self
            .application_version
            .take()
            .unwrap_or(make_api_version(0, 0, 0, 0));
        let engine_name = self
            .engine_name
            .take()
            .unwrap_or("Dragonlaze Powered Engine".to_owned());
        let engine_version = self
            .engine_version
            .take()
            .unwrap_or(make_api_version(0, 0, 0, 0));
        let extensions = self.extensions.take().unwrap_or_default();
        let layers = self.layers.take().unwrap_or_default();
        let entry = match self.entry.take() {
            Some(entry) => entry,
            None => unsafe { ash::Entry::load() }.map_err(InstanceBuilderError::from)?,
        };
        let debug_callback = self.debug_callback.take().unwrap_or(Some(print_warnings));

        Instance::new(
            entry,
            &application_name,
            application_version,
            &engine_name,
            engine_version,
            1,
            extensions,
            layers,
            self.enable_debug_layer,
            debug_callback,
        )
        .map_err(InstanceBuilderError::from)
    }
}
