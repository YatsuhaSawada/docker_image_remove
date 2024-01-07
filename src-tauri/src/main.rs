// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod docker_command;

#[tauri::command]
fn docker_image() -> String {
    //execute "docker images"
    docker_command::docker_image()
}

#[tauri::command]
fn delete_docker_img(id: &str) -> String {
    //execute "docker rmi id"
    docker_command::remove_docker_img(id)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![docker_image, delete_docker_img])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
