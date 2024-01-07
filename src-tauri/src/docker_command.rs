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

/// execute `docker image`
/// return the results of the exucution in JSON with follwing argument
/// docker image list --format "{{.Repository}}\t{{.Tag}}\t{{.ID}}\t{{.CreatedSince}}\t{{.Size}}"
pub fn docker_image() -> String {
    //execute "docker images"
    let output = Command::new("docker")
        .arg("image")
        .arg("list")
        .arg("--format")
        .arg("{{.Repository}}\t{{.Tag}}\t{{.ID}}\t{{.CreatedSince}}\t{{.Size}}")
        .current_dir("/")
        .output();
    let output = match output {
        Ok(x) => x.stdout,
        Err(err) => return format!("Err, {}", err),
    };

    let images = String::from(std::str::from_utf8(&output).unwrap());
    let lines = images.lines().collect::<Vec<&str>>();

    let mut o = DockerImags {
        ItemIdList: Vec::new(),
    };
    for line in lines {
        let parts: Vec<&str> = line.split('\t').collect();
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

/// execute `docker rmi id`
/// return the results of the exucution in JSON with follwing argument
/// docker rmi id
/// id is argument of this function
pub fn remove_docker_img(id: &str) -> String {
    //execute "docker rmi id"
    let output = Command::new("docker") // 実行したいコマンド
        .args(["rmi", id])
        .current_dir("/") // カレントディレクトリで実行
        .output();
    let output = match output {
        Ok(x) => x.stdout,
        Err(err) => return format!("Err, {}", err),
    };

    String::from(std::str::from_utf8(&output).unwrap())
}
