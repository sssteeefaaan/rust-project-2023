// Stefan Aleksić E2-42-2022

pub mod utilities;
pub mod maze;
pub mod visualize;

use std::io::Error;

use utilities::convert_txt_to_bin;
use visualize::display;

use maze::Maze;

fn main() -> Result<(), Error>{
    let txt_file_path = "primer.txt".to_string();
    let bin_file_path = "primer.bin".to_string();

    convert_txt_to_bin(&txt_file_path, &bin_file_path);

    let m = Maze::parse_from_file(&bin_file_path)?;

    display(Some(m));

    Ok(())
}