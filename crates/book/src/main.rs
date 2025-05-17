mod cli;
mod docs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run()?;
    Ok(())
}
