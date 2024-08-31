use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    io::{self, BufReader, Read},
};

type ParseResult<T> = Result<T, Box<dyn Error>>;
type Output<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Input {
    file_name: String,
    config_file_name: String,
    is_create: bool,
}

pub fn run(input: Input) -> Output<()> {
    println!("file_name: {}", input.file_name);
    println!("config_file_name: {}", input.config_file_name);
    println!("is_create: {}", input.is_create);
    Ok(())
}

pub fn get_args() -> ParseResult<Input> {
    let matches = Command::new("testp")
        .version("0.1.0")
        .author("yamd1")
        .about("Returns the test file name corresponding to the implementation file.")
        .arg(
            Arg::new("config_file_name")
                .help("")
                .short('f')
                .long("config-file")
                .value_name("CONFIG_FILE_NAME")
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("is_create")
                .help("")
                .short('c')
                .long("is-create")
                .value_name("IS_CREATE")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(
            Arg::new("file_name")
                .help("")
                .value_name("FILE_NAME")
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .get_matches();

    let file_name = matches.get_one::<String>("file_name").unwrap().to_owned();
    let config_file_name = matches
        .get_one::<String>("config_file_name")
        .unwrap()
        .to_owned();
    let is_create = matches.get_flag("is_create");

    Ok(Input {
        file_name: open(&file_name)?,
        config_file_name,
        is_create,
    })
}

fn open(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut reader = Box::new(BufReader::new(io::stdin()));
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    Ok(buffer)
}
