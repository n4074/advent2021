use anyhow::{Result, Context};
use std::{env, str::FromStr};

mod dayone;
mod daytwo;
mod daythree;

fn main() -> Result<()> {
    let mut args = env::args();
    if let (_, Some(day), Some(part), Some(input)) = (args.next(), args.next(), args.next(), args.next()) {
        let input = read_input(&input)?;
        match (day.as_str(), part.as_str()) {
            ("1", "1") => {
                dayone::partone(input)?
            }
            ("1", "2") => {
                dayone::parttwo(input)?
            }
            ("2", "1") => {
                daytwo::partone(input)?
            }
            ("2", "2") => {
                daytwo::parttwo(input)?
            }
            ("3", "1") => {
                daythree::partone(input)?
            }
            ("3", "2") => {
                daythree::parttwo(input)?
            }
            _ => println!("Unimplemented challenge day {} part {}", day, part)
        }

    } else {
        println!("Usage: advent2021 <day> <part> <path>");
    }

    Ok(())
}

fn read_input(input: &str) -> Result<String> {
    let mut path = std::path::PathBuf::from_str(input)?;
    std::fs::read_to_string(&path).context(format!("Failed to open input file {:?}", path))


}
