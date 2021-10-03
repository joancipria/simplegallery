use std::fs;
use std::vec::Vec;


pub fn get_images() -> Vec<String> {

    let mut homePath:String;

    match home::home_dir() {
        Some(path) => homePath = path.display().to_string()+"/Pictures",//println!("{}", path.display()),
        None => homePath="".to_string(),//println!("Impossible to get your home dir!"),
    }

    let mut images:Vec<String> = Vec::new();

    let paths = fs::read_dir(homePath).unwrap();

    for path in paths {
        //println!("Name: {}", path.unwrap().path().display())
        images.push(path.unwrap().path().display().to_string());
    }

    images
}