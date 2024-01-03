// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct DockerImage {
    Repository: String,
    Tag: String,
    ImageId: String,
    Created: String,
    Size: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct DockerImags {
    ItemIdList: Vec<DockerImage>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn docker_image() -> String {
    let cur = std::env::current_dir();
    println!(
        "{:?}",
        cur.as_ref().unwrap().clone().into_os_string().into_string()
    );
    match cur {
        Ok(x) => {
            let output = Command::new("docker") // 実行したいコマンド
                .args(["images"])
                .current_dir("/") // カレントディレクトリで実行
                .output()
                .expect("failed to execute process");
            let hello = output.stdout;
            let images = String::from(std::str::from_utf8(&hello).unwrap());
            let lines = images.lines().skip(1).collect::<Vec<&str>>();

            let mut o = DockerImags {
                ItemIdList: Vec::new(),
            };
            for line in lines {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let image_info = DockerImage {
                    Repository: String::from(parts[0]),
                    Tag: String::from(parts[1]),
                    ImageId: String::from(parts[2]),
                    Created: String::from(parts[3]),
                    Size: String::from(parts[4]),
                };
                o.ItemIdList.push(image_info);
            }

            let s = serde_json::to_string(&o).unwrap();
            s
        }
        Err(err) => {
            eprintln!("{:?}", err);
            String::from("-----")
        }
    }
}

#[tauri::command]
fn delete_docker_img(id: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", id)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let s = docker_image();
        println!("{}", s);
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            docker_image,
            delete_docker_img
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
