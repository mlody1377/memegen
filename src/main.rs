use std::io::{self, Write};
use reqwest::blocking::Client;


fn main() {
    println!("Please enter the text you want to add to your meme:");
    let mut input = String::new();
    
    let response = reqwest::blocking::get("url");
    
}
