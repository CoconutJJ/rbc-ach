/**
 * Author: David Yue
 * Email: davidyue5819@gmail.com
 */
let { dialog } = require("electron").remote;
let fs = require("fs");
let { spawn } = require("child_process");
let path = require("path");
let writeACHFile = require("./ach-converter");

window.addEventListener("DOMContentLoaded", () => {
    let btn = document.getElementById("convert-btn");
    let input_file = document.getElementById("in-file");
    let out_dir = document.getElementById("out-dir");

    let in_file_list = document.getElementById("in-file-list");
    let out_file_list = document.getElementById("out-file-list");
    let file_list = document.getElementById("progress");

    let input_file_path = [];

    input_file.addEventListener("click", () => {
        input_file_path = dialog.showOpenDialogSync(null, {
            properties: ["openFile", "multiSelections"],
        });

        if (!input_file_path) {
            input_file_path = [];
            return;
        }

        in_file_list.innerHTML = "";

        for (let f of input_file_path) {
            let li = document.createElement("li");
            li.innerHTML = f;
            in_file_list.appendChild(li);
        }
    });

    let out_file_path = [];

    out_dir.addEventListener("click", () => {
        out_file_path = dialog.showOpenDialogSync(null, {
            properties: ["openDirectory"],
        });

        if (!out_file_path) {
            out_file_path = [];
            return;
        }
        out_file_list.innerHTML = "";
        let li = document.createElement("li");
        li.innerHTML = out_file_path[0];
        out_file_list.appendChild(li);
    });

    btn.addEventListener("click", () => {
        file_list.innerHTML = "";
        let export_type = document.getElementById("export-type").value;

        for (let fpath of input_file_path) {
            let xl_basename = path.basename(fpath);

            let xl_file_removed_extension = xl_basename.split(".")[0];
            
            let outPath = path.join(out_file_path[0], xl_file_removed_extension + ".txt");

            writeACHFile(
                fpath,
                outPath,
                export_type
            );

            let li = document.createElement("li");

            li.innerHTML =
                "Converted input file: " + fpath + " -> " + outPath;

            file_list.appendChild(li);
        }

        out_file_path = []
        out_file_list.innerHTML = ""
        input_file_path = []
        in_file_list.innerHTML = ""

    });
});
