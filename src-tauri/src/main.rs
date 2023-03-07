#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
extern crate walkdir;
extern crate image;
extern crate base64;

use std::path::Path;
use walkdir::WalkDir;
use image::ImageOutputFormat;
use std::io::Cursor;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

#[tauri::command]
fn is_path_exist(path:&str) -> bool{
    Path::new(path).exists()
}
#[tauri::command]
fn get_file_list(folder_path: &str) -> Vec<String>{
    println!("Rust receive req: {}", folder_path);
    let mut v: Vec<String> = Vec::new();
    if is_path_exist(folder_path){
        for file in WalkDir::new(folder_path).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                let path_str = file.path().display().to_string();
                if let Some(ext) = Path::new(&path_str).extension(){
                    if ext.to_str().unwrap()=="JPG" {
                        //println!("extension: {:?}",ext);
                        v.push(path_str);
                    }
                    
                }
                
            }
        }
        println!("Done searching {} file",v.len());
    }
    
    v
}
#[tauri::command]
fn get_photo(path:&str)->String{
    let mut base_img = image::open(path).unwrap();
    let mut image_data: Vec<u8> = Vec::new();
    base_img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = general_purpose::STANDARD.encode(image_data);
    format!("data:image/png;base64,{}", res_base64)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![is_path_exist, get_file_list, get_photo])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

