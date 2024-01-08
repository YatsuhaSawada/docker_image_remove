import React from "react";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useTable } from "react-table";
import DockerImageTable from "./DockerImageTable";

export default function DockerImage({data, handleUpdate}) {
  const [imgList, setImgList] = useState("{\"ItemIdList\":[{\"Repository\":\"-\",\"Tag\":\"-\",\"ImageId\":\"-\",\"Created\":\"-\",\"Size\":\"-\"}]}");
  async function update() {
    setImgList(await invoke("docker_image"));
  }

  function handleUpdate() {
    update();
  }
  return (
    <div className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          update();
        }}
      >
      <button type="submit">Update</button>
      </form>
      {/* <details>
        <summary>filter</summary>
        <div className="toggleButton">
          <input type="checkbox" id="check1" name="group1"/><label for="check1">Tag</label>
        </div>
      </details> */}
      <DockerImageTable data={imgList} handleUpdate = {()=>{handleUpdate()}}/>
    </div>
  );
}