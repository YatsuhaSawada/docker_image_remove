import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import DockerImage from "./components/DockerImage";

function App() {

  return (
    <div className="container">
      <h1>Docker Images</h1>
      <DockerImage/>
    </div>
  );
}

export default App;
