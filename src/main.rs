#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod vulkan;
pub mod window;

fn main() {
    let context = window::Context::new().unwrap();
    context.message_loop();
    println!("Hello, world!");
}
