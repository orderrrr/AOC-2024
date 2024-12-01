use std::{fs, io::{self, BufRead, BufReader}};

pub fn file_to_vec(filename: String) -> io::Result<Vec<String>> { 
    let file_in = fs::File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
}
