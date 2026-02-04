use std::error::Error;

mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load_config("config.toml")?;
    config.validate()?;

    println!("Loaded config succesfully!");
    println!("{:#?}", config);

    Ok(())
}
