use std::{fs, io};

pub fn load_project_id() -> io::Result<String> {
    Ok(fs::read_to_string(".env")?.trim().into())
}
