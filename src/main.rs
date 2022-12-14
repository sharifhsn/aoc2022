use std::{fs::File, str::FromStr};

use color_eyre::Report;
use tracing::info;
use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use aoc2022::*;

fn main() -> Result<(), Report> {
    setup_tracing()?;
    info!("Starting AOC 2022!");
    info!("Checking one...");
    info!("Answer to one_1 is {}", one::first(File::open("one.txt")?)?);
    info!(
        "Answer to one_2 is {}",
        one::second(File::open("one.txt")?)?
    );
    info!("Answer to two_1 is {}", two::first(File::open("two.txt")?)?);
    info!("Answer to two_2 is {}", two::second(File::open("two.txt")?)?);
    info!("Answer to three_1 is {}", three::first(File::open("three.txt")?)?);
    info!("Answer to three_2 is {}", three::second(File::open("three.txt")?)?);
    Ok(())
}

fn setup_tracing() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    let filter_layer =
        Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info")).unwrap();
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();

    Ok(())
}
