// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

use utils::download_file;

mod utils;
#[tokio::main]
async fn main() {
    let dir = dirs::home_dir()
        .expect("failed to get home dir")
        .join(".cs346");
    tokio::fs::create_dir_all(dir.join("java")).await.unwrap();
    let os = if std::env::consts::OS == "macos" {
        "mac"
    } else {
        std::env::consts::OS
    };
    let arch = if std::env::consts::ARCH == "x86_64" {
        "x64"
    } else {
        std::env::consts::ARCH
    };
    // let jre_url =  format!(
    //     "https://api.adoptium.net/v3/binary/latest/17/ga/{}/{}/jre/hotspot/normal/eclipse",
    //     os, arch
    // );
    // if the file doesn't exist, download it
    if dir.join("cs346_frontend.jar").exists() {
        println!("cs346_frontend.jar exists");
    }
    download_file(
        "https://github.com/CheatCod/test/raw/main/cs346_frontend.jar",
        &dir,
        Some("cs346_frontend.jar"),
        false,
    )
    .await;

    std::process::Command::new("java")
        .arg("-jar")
        .arg(dir.join("cs346_frontend.jar"))
        .status()
        .expect("failed to start java");
}
