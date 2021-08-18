use std::{fs, io};

pub fn project_id_from_env() -> io::Result<String> {
    Ok(fs::read_to_string(".env")?.trim().into())
}
