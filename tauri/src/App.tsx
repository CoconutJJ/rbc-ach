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
  let [response, setResponse] = useState([]);

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
    let errorMessages = []
    if (inputFiles.length == 0) {
      errorMessages.push("Must select at least 1 input file.");
    }

    if (outputDir.trim().length == 0) {
      errorMessages.push("Must select an output directory.");
    }

    if (errorMessages.length > 0) {
      setResponse(errorMessages);
      return;
    }

    let data = await invoke("convert", {
      filename: inputFiles,
      recordType: recordType,
      outputDirectory: outputDir,
    }) as string[];

    setResponse([...data]);
  };

  return (
    <main className="container">
      <h1>RBC Automated Clearing House: CSV to CPA-005 Conversion Tool (v3.0)</h1>
      <p>Author: David Yue</p>
      <p>
          This is a conversion tool that converts .CSV files into the CPA-005
          specification. It supports the Debit (PAP-PAD) and Credit (PDS)
          specifications.
        </p>
        <p>
          PDS Specification:{" "}
          <a
            className="text-blue-500"
            href="https://www.rbcroyalbank.com/ach/file-451770.pdf"
          >
            https://www.rbcroyalbank.com/ach/file-451770.pdf
          </a>
        </p>
        <p>
          PAP-PAD Specification:{" "}
          <a
            className="text-blue-500"
            href="https://www.rbcroyalbank.com/ach/file-451771.pdf"
          >
            https://www.rbcroyalbank.com/ach/file-451771.pdf
          </a>
        </p>

        <p>
          To export a .CSV file from a Google Sheets Document: File &gt;
          Download &gt; Comma Seperated Values (.csv)
        </p>
        <p>
          Excel Files with extensions of .xlsx and .xls are not supported.
          Although Excel should allow you to export to a .csv file. Renaming a
          .xlsx or .xls extension to .csv file will NOT work.
        </p>
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
          {response.map((v) => (
            <li>{v}</li>
          ))}
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
