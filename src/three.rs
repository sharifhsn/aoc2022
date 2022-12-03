use std::{fs::File, io::Read};

fn atoi(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - 38
    } else {
        (c as u32) - 96
    }
}

pub fn first(mut file: File) -> color_eyre::Result<u32> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut total = 0;
    for line in buf.split("\n") {
        let (c1, c2) = line.split_at(line.len() / 2);
        for c in c1.chars() {
            if c2.contains(c) {
                total += atoi(c);
                break;
            }
        }
    }
    Ok(total)
}

pub fn second(mut file: File) -> color_eyre::Result<u32> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut total = 0;
    for line in buf.split("\n").array_chunks::<3>() {
        let mut common = vec![];
        for c in line[0].chars() {
            if line[1].contains(c) {
                common.push(c);
            }
        }
        for c in common {
            if line[2].contains(c) {
                total += atoi(c);
                break;
            }
        }
    }
    Ok(total)
}