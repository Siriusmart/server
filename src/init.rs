use dotenv::dotenv;
use std::{env, error::Error, fs, path::Path};

pub fn init() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv().ok();

    let mut path = Path::new("./storage/");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    path = Path::new("./storage/stats/");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    path = Path::new("./storage/accounts/");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    path = Path::new("./storage/accounts/profiles");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    path = Path::new("./storage/accounts/usernames");
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}
