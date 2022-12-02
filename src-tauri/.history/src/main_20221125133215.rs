// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

mod utils;
#[tokio::main]
async fn main() {
    let dir = dirs::home_dir().expect("failed to get home dir").join(".cs346");
    tokio::fs::create_dir_all(.join("java")).await.unwrap();

}
