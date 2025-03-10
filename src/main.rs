use std::env::args;

use data_model::CitiesWeather;
use tokio::{
    fs::File,
    io::{self, AsyncReadExt},
};

pub mod data_model;

const FILE_NAME: &str = "data.csv";

#[tokio::main]
async fn main() -> io::Result<()> {
    //Like Half-Global, state
    let mut data = CitiesWeather::new();

    //Getting file name
    let file_name = args().nth(1).unwrap_or(FILE_NAME.to_string());

    //let time_start = std::time::Instant::now();

    //Reading file
    let mut file = File::open(file_name).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    //Parsing file
    for line in contents.split_terminator('\n') {
        let mut parts = line.split(';');
        if let (Some(city), Some(temp_str)) = (parts.next(), parts.next()) {
            //We could trim city, but it is possible that some exotic city has leading/trailing spaces or etc
            let temp_str = temp_str.trim();
            match parts.next() {
                None => {
                    if let Ok(temp) = temp_str.parse() {
                        data.add(city.to_owned(), temp);
                    } else {
                        eprintln!("[WARN] Wrong temperature:{}", temp_str);
                    }
                }
                Some(x) => {
                    eprintln!("[WARN] Wrong: {}", x);
                }
            }
        }
    }
    //println!("Data parsed in {:?}", time_start.elapsed());

    //let data = data.into_sorted_ascii_vec();
    // IT IS VERY VERY SLOW COMPARED TO THE ASCII, BUT IT IS CORRECT (SORT IS GOING IN NATURAL MULTI-LANGUAGE ORDER)
    let data = data.into_slow_unicode_vec();

    //println!("Output parsed in {:?}", time_start.elapsed());
    let mut out_str = String::new();
    out_str.push('{');
    out_str.push_str(
        &data
            .iter()
            .map(|(city, entry)| format!("{}={}", city, entry))
            .collect::<Vec<_>>()
            .join(", "),
    );
    out_str.push('}');
    println!("{}", out_str);

    //println!("Output parsed in {:?}", time_start.elapsed());
    Ok(())
}
