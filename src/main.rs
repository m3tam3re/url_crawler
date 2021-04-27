use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::{collections::HashMap, env};

//TODO write documentation & unit tests

type Record = HashMap<String, String>;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut wtr =
        csv::Writer::from_path(format!("{:?}{:?}", get_first_arg()?, String::from("_log")))?;
    wtr.write_record(&["URL", "Error"])?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        for (k, _v) in &record {
            if k.to_lowercase().contains("url") {
                let crawl = crawl_url(record.get(k).unwrap());
                match crawl {
                    Ok(_) => continue,
                    Err(err) => {
                        wtr.write_record(vec![record.get(k).unwrap(), &err.to_string()])?;
                    }
                }
            }
        }
    }
    wtr.flush()?;
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected one argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn crawl_url(url: &String) -> Result<(), Box<dyn Error>> {
    let res = reqwest::blocking::get(url);
    match res {
        Ok(r) => {
            println!("checking URL: {}...{}", url, r.status());
            Ok(())
        }
        Err(err) => {
            println!("checking URL: {}...{}", url, "cannot crawl URL");
            Err(From::from(err.to_string()))
        }
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
