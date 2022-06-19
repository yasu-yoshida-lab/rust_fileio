use std::fs; 
use std::io::{self, BufRead, Write, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
	let mut content = fs::read_to_string("./src/data.txt")?;
	content = content.replace("\n", ",");
	println!("{}", content);

	let mut file = fs::File::create("./src/data.csv")?;
	write!(file, "{}", content)?;
	return Ok(());
}
