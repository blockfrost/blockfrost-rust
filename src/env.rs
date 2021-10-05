use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::error::DotEnvError;
use crate::Error;

#[derive(Debug, Clone, Default)]
pub struct EnvSettings {
    pub project_id: Option<String>,
    pub network_address: Option<String>,
}

pub fn load_project_id() -> crate::Result<Option<String>> {
    let project_id = load_settings()?.project_id;
    Ok(project_id)
}

pub fn load_network_address() -> crate::Result<Option<String>> {
    let network_address = load_settings()?.network_address;
    Ok(network_address)
}

pub fn load_settings() -> crate::Result<EnvSettings> {
    let current_dir = env::current_dir()?;
    let dotenv_file = scan_for_first_dotenv_file(&current_dir);

    let mut settings = EnvSettings::default();

    if let Some(dotenv_file) = dotenv_file {
        let contents = fs::read_to_string(&dotenv_file)?;

        for (i, line) in contents.lines().enumerate() {
            let line_number = i + 1;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let separator_index = match line.find('=') {
                Some(index) => index,
                None => {
                    return Err(Error::DotEnv(DotEnvError {
                        reason: "Expected separator '=' was not found",
                        path: dotenv_file,
                        line_number,
                    }))
                }
            };

            let (key, value) = line.split_at(separator_index);

            // Skip separator left in the second part
            let value = &value[1..];

            if key.is_empty() {
                return Err(Error::DotEnv(DotEnvError {
                    reason: "KEY is empty",
                    path: dotenv_file,
                    line_number,
                }));
            }

            if value.is_empty() {
                return Err(Error::DotEnv(DotEnvError {
                    reason: "VALUE is empty",
                    path: dotenv_file,
                    line_number,
                }));
            }

            if key == "BLOCKFROST_NETWORK_ADDRESS" {
                settings.network_address = Some(value.to_string());
            } else if key == "BLOCKFROST_PROJECT_ID" {
                settings.project_id = Some(value.to_string());
            }
        }
    }

    if settings.network_address.is_none() {
        settings.network_address = env::var("BLOCKFROST_NETWORK_ADDRESS").ok();
    }

    if settings.project_id.is_none() {
        settings.project_id = env::var("BLOCKFROST_PROJECT_ID").ok();
    }

    Ok(settings)
}

fn scan_for_first_dotenv_file(mut current_dir: &Path) -> Option<PathBuf> {
    loop {
        let candidate = current_dir.join(".env");
        if candidate.exists() {
            return Some(candidate);
        }

        if let Some(parent) = current_dir.parent() {
            current_dir = parent
        } else {
            return None;
        }
    }
}
