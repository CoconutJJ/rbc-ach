import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

declare module "react" {
  interface InputHTMLAttributes<T> extends HTMLAttributes<T> {
    webkitdirectory?: string;
    directory?: string;
  }
}

function App() {
  let [inputFiles, setInputFiles] = useState([]);
  let [recordType, setRecordType] = useState("PDS");
  let [outputDir, setOutputDir] = useState("");
  let [response, setResponse] = useState("");

  let removeDuplicates = (L: string[]) => {
    let unique = [];

    for (let s of L) {
      if (unique.indexOf(s) == -1) unique.push(s);
    }

    return unique;
  };

  let dialogSelector = async (e) => {
    e.preventDefault();
    let selected = await open({
      multiple: false,
      directory: true,
    });
    setOutputDir(selected);
  };

  let onInputSelect = async (e) => {
    let selected = await open({
      multiple: true,
      directory: false,
    });

    setInputFiles(removeDuplicates([...inputFiles, ...selected]));
  };

  let onRemoveInputFile = (file) => {
    setInputFiles(inputFiles.filter((v) => v != file));
  };

  let onConvert = async () => {
    let errorCount = 0;

    if (inputFiles.length == 0) {
      setResponse("Must select at least 1 input file.\n");
      errorCount++;
    }

    if (outputDir.trim().length == 0) {
      setResponse("Must select an output directory.\n");
    }

    if (errorCount > 0) return;

    let data = await invoke("convert", {
      filename: inputFiles,
      recordType: recordType,
      outputDirectory: outputDir,
    });

    setResponse(data as string);
  };

  return (
    <main className="container">
      <h1>RBC Automated Clearing House Conversion Tool (v3.0)</h1>
      <p>Author: David Yue</p>
      <div className="body">
        <div className="left">
          <form>
            <div>
              <h3>Choose CSV File (.csv)</h3>
              <button type="button" onClick={onInputSelect}>
                Add Files
              </button>
              <ul>
                {inputFiles.map((v) => (
                  <li key={v}>
                    {v} &nbsp;&nbsp;&nbsp;&nbsp;
                    <a href="#" onClick={(_) => onRemoveInputFile(v)}>
                      Remove
                    </a>
                  </li>
                ))}
              </ul>
            </div>
            <div>
              <h3>Choose Record Type</h3>
              <select
                onChange={(e) => {
                  setRecordType(e.target.value);
                }}
                value={recordType}
              >
                <option value="PDS">PDS</option>
                <option value="PAD">PAD</option>
              </select>
            </div>
            <div>
              <h3>Output Directory</h3>
              <div>
                <button type="button" onClick={dialogSelector}>
                  Choose Directory
                </button>
                &nbsp; {outputDir}
              </div>
            </div>
          </form>
        </div>
        <div className="right">
          <h3>Log</h3>
          {response}
        </div>
      </div>
      <div>
        <button type="button" className="btn-green" onClick={onConvert}>
          Convert
        </button>
      </div>
    </main>
  );
}

export default App;
