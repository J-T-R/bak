use clap::{App, Arg};
use std::fs::copy;
use std::io;

fn copy_file(name: &str, forward: bool) -> io::Result<u64> {
    let result: io::Result<u64>;
    if forward {
        result = copy(name, [name, ".bak"].concat());
    } else {
        result = copy([name, ".bak"].concat(), name);
    }
    return result;
}

fn main() {
    let matches = App::new("bak")
        .version("0.1.0")
        .about("For handling backing up your files")
        .arg(
            Arg::new("target")
                .about("target file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("restore")
                .short('r')
                .about("restore from ${target}.bak"),
        )
        .get_matches();

    let result = copy_file(
        matches.value_of("target").as_deref().unwrap(),
        !matches.is_present("restore"),
    );

    let _result = match result {
        Ok(_) => (),
        Err(error) => println!("{:?}", error),
    };
}
