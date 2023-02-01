use std::fs::{File};
use std::io::{Read, Write, Error};

pub fn convert_string_to_u8(content: &String) -> Vec<u8>{
    let mut data = Vec::<u8>::new();
    let mut counter = 7;
    let mut value = 0;
    for c in content.split(""){
        if c == "1" {
            value += (2 as u8).pow(counter);
        }
        if c == "1" || c == "0" {
            if counter == 0 {
                data.push(value);
                value = 0;
                counter = 8;
            }
            counter -= 1;
        }
    }
    if counter < 7 {
        data.push(value);
    }
    return data;
}

pub fn convert_txt_to_bin(txt_file_path: &String, bin_file_path:&String){
    let mut file = File::open(txt_file_path).expect(format!("Couldn't create the file '{txt_file_path}'!").as_str());
    let mut content = String::new();
    file.read_to_string(&mut content).expect(format!("Couldn't read from the file '{txt_file_path}'!").as_str());
    let data = convert_string_to_u8(&content);

    let mut bin_file = File::create(bin_file_path).expect(format!("Couldn't create the file '{bin_file_path}'!").as_str());
    bin_file.write_all(&data).expect(format!("Couldn't write to '{bin_file_path}'!").as_str());
}

pub fn read_binary(bin_file_path :&String) -> Result<Vec<u8>, Error>{
    let mut data = Vec::<u8>::new();

    let mut file = File::open(bin_file_path)?;
    file.read_to_end(&mut data)?;

    Ok(data)
}