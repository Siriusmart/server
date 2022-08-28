use std::{error::Error, path::Path, fs};

pub fn init() -> Result<(), Box<dyn Error>> {
    let mut path = Path::new("./storage/");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    path = Path::new("./storage/stats/");
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}
