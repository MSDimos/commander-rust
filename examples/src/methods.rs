use std::fs::{ File, OpenOptions, write, create_dir_all, read };
use std::path::Path;
use std::process::exit;
use std::io::{Error, ErrorKind, Write, Result};

pub fn write_file(path: &Path, content: String, force: bool) -> Result<()> {
    if !path.exists() {
        if !force {
            eprintln!("can't find output file, please add '-f' to create it");
            exit(0);
        } else {
            if path.is_file() {
                create_dir_all(path.parent().unwrap())?;
                write(path, content.as_bytes())?;
            } else if path.is_dir() {
                if path.ends_with(".hash") {
                    create_dir_all(path.parent().unwrap())?;
                    write(path, content.as_bytes())?;
                } else {
                    create_dir_all(path)?;
                    write(path.join(".hash"), content.as_bytes())?;
                }
            }
        }
    } else {
        write(path, content.as_bytes())?;
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