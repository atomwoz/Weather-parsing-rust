use std::env::args;

pub mod data_model;

const FILE_NAME: &str = "data.csv";

fn main() {
    //Getting file name
    let file_name = args().nth(1).unwrap_or(FILE_NAME.to_string());
}
