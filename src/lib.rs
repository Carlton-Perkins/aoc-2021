use std::{
    fs,
    io::{Error, ErrorKind},
};

use clap::{App, Arg};

pub fn load_data_file() -> Result<String, Error> {
    let matches = App::new("aoc")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true),
        )
        .get_matches();

    if let Some(file) = matches.value_of("file") {
        fs::read_to_string(file)
    } else {
        Err(Error::new(ErrorKind::Other, "no file param"))
    }
}
