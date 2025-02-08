#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::error::Error;
use std::fmt::{self, Display};
use std::path::PathBuf;
use dirs;

const ERR: &'static str = "Generic VsaError:";
const CUSTOM_ERR: &'static str = "Custom VsaError:";

#[derive(Debug)] // Add Serde
enum VsaError {
    FileNotFound,
    FileEditError,
    FileReadError,
    CustomErr(String),
}

impl Error for VsaError {}

impl VsaError {
    fn custom_err(msg: &str) -> Self {
        Self::CustomErr(msg.to_string())
    }
}

impl Serialize for VsaError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Display for VsaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VsaError::FileNotFound => write!(f, "{} File Not Found (Check Url Related Functions)", ERR),
            VsaError::FileEditError => write!(f, "{} File Edit Error (Check Url Related Functions)", ERR),
            VsaError::FileReadError => write!(f, "{} File Could Not Be Read (Check Url Related Functions)", ERR),
            VsaError::CustomErr(err_msg) => write!(f, "{} {}", CUSTOM_ERR, err_msg),
        }
    }
}

#[derive(Serialize)]
struct VsaClient {
    name: String,
    date: String,
    cost: String,
}

impl VsaClient {
    /* Appends file vsadata.txt with VsaClient fields */
    fn append_text(&self) -> Result<String, VsaError> { 
        let mut file: File = Self::open_file()?;

        let cost: String = format!("${}", self.cost);

        writeln!(file, "{} {} {}", self.name, self.date, cost).map_err(|_| { VsaError::FileEditError })?;

        Ok("File Editted Successfully!".to_string())
    }
    /* Returns all data in vsadata.txt */
    fn read_text() -> Result<String, VsaError> {
        let mut file: File = Self::open_file()?;
        let mut text_return: String = String::new();

        file.read_to_string(&mut text_return).map_err(|_| {
            VsaError::custom_err("fn read_text file.read_to_string() Err")
        })?;

        if text_return == "" {
            return Ok("No Classes Inputed".to_string());
        }

        Ok(text_return)
    }
    /* Removes Last Inputted Class */
    fn remove_text() -> Result<String, VsaError> {
        let mut file_string: File = Self::open_file()?;
        let mut buffer: String = String::new();
        let mut lines: Vec<&str> = Vec::new();
        let mut return_string: String = String::new();

        file_string.read_to_string(&mut buffer).map_err(|_| {
            VsaError::custom_err("fn remove_text file.read_to_string() Err")
        })?;

        for line in buffer.trim().lines() {
            if line != "" {
                lines.push(line);
            } else {
                continue;
            }
        }
        lines.pop();

        for c in lines {
            return_string.push_str(c);
            return_string.push_str("\n");
        }
        
        let mut file: File = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Self::find_txt_file()?).unwrap();

        file.write_all(b"").unwrap();

        write!(file_string, "{}", return_string).map_err(|_| {
            VsaError::custom_err("fn write! Err")
        })?;

        Ok("Last Class Removed!".to_string())
    }
   
    /* Opens file from find_txt_file() */
    fn open_file() -> Result<File, VsaError> {
        let filename: String = Self::find_txt_file()?;

        let file_data: Result<File, io::Error> = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&filename);   

        let file_content: File = file_data.map_err(|_| {
            VsaError::custom_err("Absolute Url in url_locator.txt not correct")
        })?;
        Ok(file_content)
    }

    fn find_txt_file() -> Result<String, VsaError> {
        if let Some(desktop_dir) = dirs::desktop_dir() {
            let file_name = "VSA Organizer App\\vsa-organizer\\src-tauri\\src\\vsa-text-files\\vsa_data.txt";
            let file_path: PathBuf = desktop_dir.join(file_name);
            if file_path.exists() {
                let retval = file_path.to_str().unwrap();
                return Ok(String::from(retval));
            } else {
                return Err(VsaError::custom_err("File Url in fetch filename is incorrect for exe or tauri dev"))
            }
        }
        Err(VsaError::custom_err("URL Unreachable"))
    }
}

#[derive(Serialize)]
struct Query {
    query: String,
}

impl Query {
    fn search(&self) -> Result<String, VsaError> {
        let mut results: Vec<String> = Vec::new();
        let mut results_return: String = String::new();
        let contents: String = VsaClient::read_text()?;

        if contents == "" {
            return Ok("No Classes Inputed".to_string());
        } else {
            for line in contents.trim().to_lowercase().lines() {
                if line.contains(&self.query.to_lowercase()) {
                    results.push(line.to_string());
                } else {
                    continue;
                }
            }
    
            for lines in results {
                let i = format!("{}\n", lines);
                results_return.push_str(&i);
            }
    
            if results_return == "" {
                return Ok("No Classes With Specified Student".to_string())
            }
            Ok(results_return)
        }
    }
}

#[tauri::command]
fn vsa_write(name: String, date: String, cost: String) -> String {
    let vsa_client: VsaClient = VsaClient { name, date, cost, };

    match VsaClient::append_text(&vsa_client) {
        Ok(v) => {
            return v;
        }
        Err(e) => {
            return e.to_string();
        }
    }
}

#[tauri::command]
fn vsa_read() -> String {
    match VsaClient::read_text() {
        Ok(v) => {
            return v;
        }
        Err(e) => {
            return e.to_string();
        }
    }
}

#[tauri::command]
fn vsa_remove() -> String {
    match VsaClient::remove_text() {
        Ok(v) => {
            return v;
        }
        Err(e) => {
            return e.to_string();
        }
    }
}

#[tauri::command]
fn vsa_query(query: String) -> String {
    let query_new: Query = Query { query };
    
    match Query::search(&query_new) {
        Ok(v) => {
            return v;
        }
        Err(e) => {
            return e.to_string();
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![vsa_write, vsa_read, vsa_remove, vsa_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}