import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import DockerImage from "./components/DockerImage";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <App />
    <DockerImage />
  </React.StrictMode>,
);
