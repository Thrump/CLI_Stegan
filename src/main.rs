extern crate steganography;
extern crate reqwest;
extern crate clap;

use std::io;
use std::fs::File;
use steganography::encoder::*;
use steganography::decoder::*;
use steganography::util::*;
use std::io::Read;
use clap::{Arg, App, SubCommand};


fn main() {
    let app = App::new("cl_stegan")
        .subcommand(SubCommand::with_name("encode")
            .about("encodes an image, either from the web or your files, with a hidden message")
            .version("0.0.1")
            .author("Von Mbah <vonchude@gmail.com>")
            .arg(Arg::with_name("from")
                .value_name("FROM")
                .required(true)))
        .subcommand(SubCommand::with_name("decode")
            .about("decodes an image from the files and prints out the hidden message"))
        .get_matches();

}

fn encode(input: String, message: String, output:String ){
    let byte_mes = str_to_bytes(&message);
    let destination_image = file_as_dynamic_image(input.trim().to_string());
    let enc  = Encoder::new(byte_mes, destination_image);
    let result = enc.encode_alpha();
    save_image_buffer(result, output.trim().to_string());
}

fn decode(input: String) -> String {
    let encoded_image = file_as_image_buffer(input.trim().to_string());
    let dec = Decoder::new(encoded_image);
    let out_buff= dec.decode_alpha();

    let clean_filt: Vec<u8> = out_buff.into_iter().filter(|p| {*p != 0xff_u8}).collect();
    String::from_utf8(clean_filt).unwrap()


}
