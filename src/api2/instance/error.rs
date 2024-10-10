//! Error types for the instance module.

use std::{error, ffi::NulError, fmt};

use super::super::PropertiesConversionError;
use ash::vk;

/// Errors that can occur in the [super::Instance] struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceError {
    /// No validation layer found.
    NoValidationLayer,
    /// Vulkan error.
    Vulkan(vk::Result),
    /// Nul error.
    NulError(NulError),
    /// Error converting properties.
    PropertiesConversion(PropertiesConversionError),
}

impl From<vk::Result> for InstanceError {
    fn from(result: vk::Result) -> Self {
        Self::Vulkan(result)
    }
}

impl From<NulError> for InstanceError {
    fn from(error: NulError) -> Self {
        Self::NulError(error)
    }
}

impl From<PropertiesConversionError> for InstanceError {
    fn from(error: PropertiesConversionError) -> Self {
        Self::PropertiesConversion(error)
    }
}

impl fmt::Display for InstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoValidationLayer => write!(f, "no validation layer found"),
            Self::Vulkan(e) => e.fmt(f),
            Self::NulError(e) => e.fmt(f),
            Self::PropertiesConversion(e) => e.fmt(f),
        }
    }
}

impl error::Error for InstanceError {}

/// Errors that can occur in the [super::InstanceBuilder].
#[derive(Debug)]
pub enum InstanceBuilderError {
    /// No Vulkan entry provided.
    NoVulkanEntry,
    /// Error creating the instance.
    Instance(InstanceError),
    /// Error loading the Vulkan entry.
    VulkanEntry(ash::LoadingError),
    /// Error converting properties.
    PropertiesConversion(PropertiesConversionError),
    /// Vulkan error.
    Vulkan(vk::Result),
}

impl From<InstanceError> for InstanceBuilderError {
    fn from(error: InstanceError) -> Self {
        Self::Instance(error)
    }
}

impl From<ash::LoadingError> for InstanceBuilderError {
    fn from(error: ash::LoadingError) -> Self {
        Self::VulkanEntry(error)
    }
}

impl From<PropertiesConversionError> for InstanceBuilderError {
    fn from(error: PropertiesConversionError) -> Self {
        Self::PropertiesConversion(error)
    }
}

impl From<vk::Result> for InstanceBuilderError {
    fn from(result: vk::Result) -> Self {
        Self::Vulkan(result)
    }
}

impl fmt::Display for InstanceBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoVulkanEntry => write!(f, "no Vulkan entry provided"),
            Self::Instance(e) => e.fmt(f),
            Self::VulkanEntry(e) => e.fmt(f),
            Self::PropertiesConversion(e) => e.fmt(f),
            Self::Vulkan(e) => e.fmt(f),
        }
    }
}

impl error::Error for InstanceBuilderError {}
