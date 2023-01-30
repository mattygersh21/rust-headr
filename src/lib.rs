use clap::{App,Arg};
use std::{io::{BufRead, BufReader, self}, error::Error, fs::File};

type MyResult<T> = Result<T,Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("rust-headr")
        .version("0.1.0")
        .author("Matthew Gerszewski dakotaseia@hotmail.com")
        .about("Simple clone of head command-line tool.")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input files")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Print specified number of lines of file.")
                .default_value("10")
                .conflicts_with("bytes")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Print specified number of bytes of file.")
                .takes_value(true)
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("given line count not allowed -- {}",e))?;
    
    let bytes = matches
        .value_of("btyes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("given byte count not allowed -- {}",e))?;
    
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes
    })
    
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}",filename,err),
            Ok(_) => println!("Opened {}",filename)
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n>0 => Ok(n),
        _ => Err(From::from(val))
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),"foo".to_string());
    
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),"0".to_string());
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}