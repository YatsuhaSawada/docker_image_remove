import React from "react";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useTable } from "react-table";

export default function DockerImageTable({data, handleUpdate}) {
  async function deleteImg(id) {
    //setImgList(await invoke("docker_image"));
    console.log(`Deleting image with ID: ${id}`);
    
    console.log(await invoke("delete_docker_img", {id}));
  };
  const handleDeleteClick = (id) => {
    // Call the deleteImg function with the provided image ID
    deleteImg(id);
    handleUpdate();
  };
  const dataArray = JSON.parse(data).ItemIdList;
  
  return (
    <table>
      <thead>
        <tr>
          <th>Repository</th>
          <th>Tag</th>
          <th>ImageId</th>
          <th>Created</th>
          <th>Size</th>
        </tr>
      </thead>
      <tbody>
        {dataArray.map((item, index) => (
          <tr key={index}>
            <td>{item.Repository}</td>
            <td>{item.Tag}</td>
            <td>{item.ImageId}</td>
            <td>{item.Created}</td>
            <td>{item.Size}</td>
            <td>
              <button onClick={()=>handleDeleteClick(item.ImageId)}>remove</button>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}