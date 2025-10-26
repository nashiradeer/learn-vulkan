use std::{env, path::PathBuf};

fn main() {
    let vulkan_sdk_path = env::var("VULKAN_SDK");

    if cfg!(target_os = "windows") {
        println!(
            "cargo:rustc-link-search=native={}/Lib",
            vulkan_sdk_path
                .as_ref()
                .expect("On Windows, VULKAN_SDK environment variable must be set")
        );
    }

    let link_kind = if cfg!(target_feature = "crt-static") {
        "static"
    } else {
        "dylib"
    };

    println!("cargo::rustc-link-lib={}=vulkan-1", link_kind);

    let mut vulkan_bindings_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    if cfg!(target_os = "windows") {
        vulkan_bindings_builder = vulkan_bindings_builder
            .clang_arg(format!(
                "-I{}/Include",
                vulkan_sdk_path
                    .as_ref()
                    .expect("On Windows, VULKAN_SDK environment variable must be set")
            ))
            .clang_arg("-DVK_USE_PLATFORM_WIN32_KHR");
    }

    let vulkan_bindings = vulkan_bindings_builder
        .generate()
        .expect("Unable to generate bindings for Vulkan API");

    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    vulkan_bindings
        .write_to_file(output_path.join("vulkan_bindings.rs"))
        .expect("Couldn't write bindings for Vulkan API!");
}
