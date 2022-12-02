// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

mod utils;
#[tokio::main]
async fn main() {
    tokio::fs::create_dir_all(dirs::home_dir().expect("failed to get home dir").join(".cs346").join("bin")).await.unwrap();
    tokio::fs::create_dir_all(dirs::home_dir().expect("failed to get home dir")).await.unwrap();

}
