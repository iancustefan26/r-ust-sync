use anyhow::Result;

mod cli_parsing;
mod errors;

fn main() -> Result<()> {
    if let Some(args) = cli_parsing::parse_args()? {
        println!("Parsed arguments: {:?}", args);
    } else {
        println!("Running...");
    }
    Ok(())
}
