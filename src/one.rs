use std::{fs::File, io::Read};

pub fn first(mut file: File) -> color_eyre::Result<u32> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut biggest = 0;
    for elf in buf.split("\n\n") {
        let mut total = 0;
        for meal in elf.split("\n") {
            total += meal.parse::<u32>()?;
        }
        if total > biggest {
            biggest = total;
        }
    }
    
    Ok(biggest)
}

pub fn second(mut file: File) -> color_eyre::Result<u32> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut totals = vec![];
    for elf in buf.split("\n\n") {
        let mut total = 0;
        for meal in elf.split("\n") {
            total += meal.parse::<u32>()?;
        }
        totals.push(total);
    }
    totals.sort();
    let totals: Vec<&u32> = totals.iter().rev().collect();
    Ok(totals[0] + totals[1] + totals[2])
}