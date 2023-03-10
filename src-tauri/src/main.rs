#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::time::{Duration, Instant};
extern crate walkdir;
extern crate image;
extern crate base64;
use image::{DynamicImage, ImageOutputFormat};

use std::path::Path;
use walkdir::WalkDir;

use image::io::Reader as ImageReader;
use image::imageops::FilterType;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
use win_ocr::ocr;
use std::fs;

fn image_to_base64(img:DynamicImage)->String{
    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let base64 = general_purpose::STANDARD.encode(image_data);
    base64
}

fn resized_width_height(old_width: u32, old_height: u32, ratio:(u32, u32))->(u32, u32){
    let mut nwidth: u32 = 0;
    let mut nhight: u32 = 0;
    let ratio_three = ratio.0;
    let ratio_four= ratio.1;
    if old_width > old_height{
        nwidth = ratio_four;
        nhight = ratio_three;
    }
    else{
        nwidth = ratio_three;
        nhight = ratio_four;
    }
    (nwidth, nhight)
}

fn res_image(img: DynamicImage, higher:bool)->DynamicImage{
    let mut nwidth: u32;
    let mut nheight: u32;
    if higher{
        (nwidth, nheight) = resized_width_height(img.width(), img.height(),(750, 1000));        
    }
    else{
        (nwidth, nheight) = resized_width_height(img.width(), img.height(), (450, 600));
    }
    let res_image = img.resize(nwidth, nheight, FilterType::Lanczos3);
    res_image
}
fn resized_image(img: DynamicImage)->DynamicImage{
    // Read source image from file
    res_image(img, false)
}
fn resized_image_higher(img: DynamicImage)->DynamicImage{
    res_image(img, true)
}
fn rotate_image(img: DynamicImage, deg:u32)->DynamicImage{
    let mut rot_img: DynamicImage;
    if deg == 90{
        rot_img = img.rotate90();
    }
    else if deg == 180 {
        rot_img = img.rotate180();
    }
    else if deg == 270 {
        rot_img = img.rotate270();
    }
    else{
        rot_img = img;
    }
    rot_img
}
#[tauri::command]
fn get_rotated_image(src_path:&str, deg:u32)->String{
    let img = ImageReader::open(src_path)
        .unwrap()
        .decode()
        .unwrap();
    let res_img = resized_image_higher(img);
    let rot_image = rotate_image(res_img, deg);
    let base64 = image_to_base64(rot_image);
    format!("data:image/png;base64,{}", base64)
}
#[tauri::command]
fn get_ocr_info(src_path: &str, deg:u32)->String{
    let img = ImageReader::open(src_path)
        .unwrap()
        .decode()
        .unwrap();
    let res_img = resized_image(img);
    let mut rot_img: DynamicImage;
    if deg==90{
        rot_img = res_img.rotate90()
    }
    else if deg == 180 {
        rot_img = res_img.rotate180()
    }
    else if deg == 270 {
        rot_img = res_img.rotate270()
    }
    else{
        rot_img = res_img
    }
    if let Ok(msg) = rot_img.save("_temp_rotated.jpg"){
        let ocr_text: String = ocr("_temp_rotated.jpg").unwrap();
        return ocr_text;
    }

    String::from("Ocr failed")
}


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
    println!("Receive path:{}",path);
    let start = Instant::now();
    let mut duration: Duration;
    let img = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap();
    let mut sub_start = Instant::now();
    let res_img = resized_image(img);
    duration = sub_start.elapsed();
    println!("Resize time:{}ms",duration.as_millis());

    sub_start = Instant::now();

    let base64 = image_to_base64(res_img);
    println!("To base64 time:{}ms",sub_start.elapsed().as_millis()); 
    
    duration = start.elapsed();

    println!("Time elapsed in get_photo function() is: {:?}ms", duration.as_millis());
    format!("data:image/png;base64,{}", base64)
}

#[tauri::command]
fn rotate_and_copy(deg:u32, src_path:&str, dest_path:&str)->bool{
    println!("RAC received:");
    println!("src: {}", src_path);
    println!("dest: {}", dest_path);
    println!("deg: {}", deg);
    let img = ImageReader::open(src_path)
        .unwrap()
        .decode()
        .unwrap();
    let res_img = resized_image_higher(img);
    let rot_image = rotate_image(res_img, deg);
    let result = rot_image.save(dest_path);
    if let Ok(_) = result{
        return true
    }
    else{
        println!("{:?}", result)
    }
    false
}
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![is_path_exist, get_file_list, get_photo, get_ocr_info, get_rotated_image, rotate_and_copy])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

