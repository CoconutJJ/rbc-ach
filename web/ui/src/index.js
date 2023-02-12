import React from "react";
import ReactDOM from "react-dom";
import App from "./App";
import "./styles.css";

document.addEventListener("DOMContentLoaded", () => {
  var mountNode = document.getElementById("app");
  ReactDOM.render(<App name="Jane" />, mountNode);
});
