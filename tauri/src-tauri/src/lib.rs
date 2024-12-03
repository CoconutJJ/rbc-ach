// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod csvconv;

use csvconv::csv::convert_to_cpa005;
use csvconv::types::RecordType;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
#[tauri::command]
fn convert(filename: Vec<&str>, record_type: &str, outputDirectory: &str) -> String {
    let mut error = String::new();

    for s in filename {
        let csv_file = File::open(s);

        match csv_file {
            Ok(mut f) => {
                let mut buf = String::new();

                let record_type = match record_type {
                    "PDS" => RecordType::Credit,
                    "PAD" => RecordType::Debit,
                    _ => panic!("invalid record type!"),
                };

                let result =
                    convert_to_cpa005(f.read_to_string(&mut buf).unwrap().to_string(), record_type);

                match result {
                    Ok(s) => {
                        let outfile_name = format!(
                            "{}.txt",
                            Path::new(&s).file_stem().unwrap().to_str().unwrap()
                        );

                        let outfile = File::create(Path::new(outputDirectory).join(&outfile_name));

                        match outfile {
                            Ok(mut f) => {
                                f.write_all(s.as_bytes());
                            }
                            Err(e) => error.push_str(format!("error: cannot write output file {}: {}", &outfile_name, e).as_str()),
                        };
                    }
                    Err(e) => error.push_str(e.to_string().as_str()),
                }
            }
            Err(e) => {
                return e.to_string();
            }
        }
    }

    return error;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
