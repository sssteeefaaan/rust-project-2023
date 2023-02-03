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

    let mut m = Maze::parse_from_file(&bin_file_path)?;

    m.compare_times_for_path_search();

    display(Some(m));

    Ok(())
}