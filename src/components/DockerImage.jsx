import React from "react";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useTable } from "react-table";

export default function DockerImage() {
  const dataArray = [
      { name: '田中', age: 25, city: '東京' },
      { name: '高橋', age: 30, city: '大阪' },
      { name: '佐藤', age: 35, city: '北海道' },
  ];
  
  return (
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Age</th>
          <th>City</th>
        </tr>
      </thead>
      <tbody>
        {dataArray.map((item, index) => (
          <tr key={index}>
            <td>{item.name}</td>
            <td>{item.age}</td>
            <td>{item.city}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}