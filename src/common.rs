use std::{io::Read, fmt::Display};

pub trait Challenge<T: Display> {
    fn run(file: std::fs::File) -> T;

    fn parse(mut file: std::fs::File) -> color_eyre::Result<String> {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }
}