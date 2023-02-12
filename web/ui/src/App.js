import React, { useState } from "react";
import { hot } from "react-hot-loader/root";

function App() {
  /**
   * @var File
   */
  let [files, setFiles] = useState([]);

  let [recordType, setRecordType] = useState(null);

  let onFileChange = (e) => {
    setFiles(e.target.files);
  };

  let download_as_file = (data, filename) => {
    let dataURL = window.URL.createObjectURL(
      new Blob([data], { type: "text/plain" })
    );

    let link = document.createElement("a");

    link.style.display = "none";
    link.href = dataURL;
    link.download = filename;

    document.body.appendChild(link);

    link.click();

    window.URL.revokeObjectURL(dataURL);
    link.parentElement.removeChild(link);
  };

  let convert = () => {
    let xhttp = new XMLHttpRequest();

    let formdata = new FormData();

    for (let i = 0; i < files.length; i++) {
      formdata.append("file", files[i]);
    }

    xhttp.onload = () => {
      download_as_file(xhttp.responseText, files[0].name);
    };

    let url = "/convert?convtype=" + recordType;

    xhttp.open("POST", url, true);
    xhttp.send(formdata);
  };

  return (
    <>
      <div>
        <h1>
          RBC Automated Clearing House Direct Payments Conversion Tool (v2.0)
        </h1>
        <hr />
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

        <h3 className="text-2xl">Upload .CSV file</h3>

        <table>
          <tr>
            <td>CSV File</td>
            <td>
              <input type="file" onChange={onFileChange} multiple={false} />
            </td>
          </tr>
          <tr>
            <td>Record Type</td>
            <td>
              {" "}
              <select
                id="rec-type"
                onChange={(e) => setRecordType(e.target.value)}
              >
                <option value="">-- SELECT --</option>
                <option value="PDS">PDS</option>
                <option value="PAD">PAP-PAD</option>
              </select>
            </td>
          </tr>
          <tr>
            <td></td>
            <td>
              <input type="button" onClick={convert} value="Convert" />
            </td>
          </tr>
        </table>

        <p>
          The first version of this tool was a desktop application I wrote in
          about half a day, the UI was generally horrible and the code (in
          JavaScript) was also a horrible mess. This version improves on a few
          notable things...
          <ul>
            <li>
              It is much more stringent on data formats. This version makes many
              more sanity checks on the input CSV file and it will not output a
              CPA-005 file if sanity checks are not met. This is to limit the
              number of human errors that can occur.
            </li>
            <li>
              This tool is much more portable and self-contained. There is only
              one program executable does not require external dependencies
              apart from a web browser.
            </li>
            <li>The codebase is cleanly written and maintainable</li>
            <li>It is written in Rust instead of JavaScript</li>
          </ul>
          Author: David Yue
        </p>
      </div>
    </>
  );
}

export default hot(App);
