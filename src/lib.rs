use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
};

type ParseResult<T> = Result<T, Box<dyn Error>>;
pub type TestDirectoryName = String;

#[derive(Debug, Clone)]
pub struct Input {
    pub file: PathBuf,
    pub test_dir: TestDirectoryName,
}

pub fn get_args() -> ParseResult<Input> {
    let matches = Command::new("testmate")
        .version("0.1.0")
        .author("yamd1")
        .about("Returns the test file name corresponding to the implementation file.")
        .arg(
            Arg::new("target")
                .help("")
                .short('d')
                .long("target")
                .value_name("TARGET")
                .action(ArgAction::Set),
        )
        .get_matches();

    let test_dir = matches
        .get_one::<String>("target")
        .ok_or_else(|| Box::<dyn Error>::from("test_dir is missing"))?
        .to_string();

    Ok(Input {
        file: parse_file_name()?,
        test_dir,
    })
}

fn parse_file_name() -> Result<PathBuf, Box<dyn Error>> {
    let mut reader = BufReader::new(io::stdin());
    let mut buffer = String::new();

    if let Err(e) = reader.read_to_string(&mut buffer) {
        return Err(Box::new(e));
    };

    Ok(Path::new(&buffer.trim().to_string()).to_owned())
}
