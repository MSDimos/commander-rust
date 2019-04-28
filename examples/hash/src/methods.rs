use std::fs::{ write, create_dir_all, read };
use std::path::Path;
use std::process::exit;
use std::io::{Result};

pub fn write_file(path: &Path, content: String, force: bool) -> Result<()> {
    let path = path.join(".hash");

    if !force {
        if !path.exists() {
            eprintln!("{:?} doesn't exist, please add '-f' to create it", path);
            exit(0);
        }
    } else {
        create_dir_all(path.parent().unwrap())?;
        write(path, content)?;
    }

    Ok(())
}

pub fn read_file(path: &Path) -> String {
    if let Ok(c) = read(path) {
        unsafe {
            String::from_utf8_unchecked(c)
        }
    } else {
        eprintln!("Error, file don't exist or permission reject");
        exit(0);
    }
}