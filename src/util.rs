extern crate steganography;

use steganography::encoder::*;
use steganography::decoder::*;
use steganography::util::*;



pub fn encode(input: String, message: String, output:String ){
    let byte_mes = str_to_bytes(&message);
    let destination_image = file_as_dynamic_image(input.trim().to_string());
    let enc  = Encoder::new(byte_mes, destination_image);
    let result = enc.encode_alpha();
    save_image_buffer(result, output.trim().to_string());
}

pub fn decode(input: String){
    let encoded_image = file_as_image_buffer(input.trim().to_string());
    let dec = Decoder::new(encoded_image);
    let out_buff= dec.decode_alpha();

    let clean_filt: Vec<u8> = out_buff.into_iter().filter(|p| {*p != 0xff_u8}).collect();
    println!("{}", String::from_utf8(clean_filt).unwrap());
}