import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import DockerImage from "./components/DockerImage";

function App() {
  const [imgList, setImgList] = useState("{\"ItemIdList\":[{\"Repository\":\"-\",\"Tag\":\"-\",\"ImageId\":\"-\",\"Created\":\"-\",\"Size\":\"-\"}]}");
  const [name, setName] = useState("");
  async function update() {
    setImgList(await invoke("docker_image"));
  }

  function handleUpdate() {
    update();
  }

  return (
    <div className="container">
      <h1>Docker Images</h1>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          update();
        }}
      >
      <button type="submit">Update</button>
      </form>      
    <DockerImage data={imgList} handleUpdate = {()=>{handleUpdate()}}/>
    </div>
  );
}

export default App;
