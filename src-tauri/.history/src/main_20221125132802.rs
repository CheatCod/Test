// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

mod utils;
#[tokio::main]
fn main() {
    println!("Hello, world!");
}
