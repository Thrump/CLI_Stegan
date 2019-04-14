
extern crate reqwest;
extern crate clap;

use std::io;
use std::fs::File;
use clap::{Arg, App};

use cli_stegan::util::*;

use std::fs;

fn main() -> std::io::Result<()> {
    let app = App::new("cl_stegan")
        .author("Von Mbah <vonchude@gmail.com>")
        .version("0.0.1")
        .about("A command line interface for decoding and encoding images with hidden messages.")
        .arg(Arg::with_name("encode")
            .help("Encodes an image with a hidden message.")
            .short("e")
            .long("encode")
            .case_insensitive(true))
        .arg(Arg::with_name("decode")
            .help("Decodes an image with a hidden message.")
            .short("d")
            .long("decode")
            .case_insensitive(true))
        .get_matches();



    if let Some(_o) = app.values_of("encode"){
        println!("Is your image from the web (w) or file (f)?: ");
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        let format = response.trim();
        if format.to_lowercase() == "web" || format.to_lowercase() == "w" {

            println!("Enter in the website address exactly to the image: ");
            let mut string_format1 = String::new();
            io::stdin().read_line(&mut string_format1)?;
            let image_link = string_format1.trim();
            let mut resp = reqwest::get(image_link).expect("request failed");

            let mut creation = File::create("dummy.jpg").expect("failed to create a file");
            io::copy(&mut resp, &mut creation).expect("failed to copy content");

            println!("Enter in your hidden message: ");
            let mut message = String::new();
            io::stdin().read_line(&mut message)?;

            println!("Enter in output file (Must end in .png): ");
            let mut out =String::new();
            io::stdin().read_line(&mut out)?;

            encode("dummy.jpg".to_string(), message, out);


        }else if format.to_lowercase() == "file" || format.to_lowercase() == "f" {
            println!("Enter in the file: ");
            let mut file = String::new();
            io::stdin().read_line(&mut file)?;

            println!("Enter in your hidden message: ");
            let mut message = String::new();
            io::stdin().read_line(&mut message)?;

            println!("Enter in output file (Must end in .png): ");
            let mut out =String::new();
            io::stdin().read_line(&mut out)?;

            encode(file, message, out);

            fs::remove_file("dummy.jpg").expect("Error trying to delete file");
        }else {
            eprintln!("Error, didn't choose either web (w) or file (f)");
        }

    }

    if let Some(_o) = app.values_of("decode") {
        println!("Enter in the file: ");
        let mut string_format = String::new();
        io::stdin().read_line(&mut string_format).expect("Error trying to read");
        decode(string_format);
    }

    Ok(())

}


