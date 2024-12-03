use std::{
    fs,
    io::{self, BufRead, BufReader, Read},
};

pub fn file_to_string(filename: String) -> io::Result<String> {
    let file_in = fs::File::open(filename)?;
    let mut file_reader = BufReader::new(file_in);
    let mut s = String::new();
    file_reader.read_to_string(&mut s).unwrap();
    return Ok(s);
}

pub fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
