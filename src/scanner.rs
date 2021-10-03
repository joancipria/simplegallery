use std::fs;
use std::vec::Vec;


pub fn get_images() -> Vec<String> {

    let mut images:Vec<String> = Vec::new();

    let paths = fs::read_dir("/home/joan/Pictures").unwrap();

    for path in paths {
        //println!("Name: {}", path.unwrap().path().display())
        images.push(path.unwrap().path().display().to_string());
    }

    images
}