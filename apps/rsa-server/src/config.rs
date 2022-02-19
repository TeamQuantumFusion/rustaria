use std::{
    fs::File,
    io::{Read, Write},
    net::SocketAddr,
    path::Path,
};

use eyre::Result;
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerSettings,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerSettings {
    pub server_addr: SocketAddr,
}

impl Config {
    const DEFAULT_CONFIG: &'static str = include_str!("default_config.toml");

    pub fn from_file(path: &Path) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut data = vec![];
        f.read_to_end(&mut data)?;
        let conf = toml::from_slice(data.as_slice())?;

        Ok(conf)
    }
    pub fn from_file_or_default(path: &Path) -> Self {
        match Self::from_file(path) {
            Ok(c) => c,
            Err(e) => {
                warn!("Error when reading config file: {e}");
                // fallback
                warn!("Attempting to create default config file...");
                if let Err(e) = Self::create_default_config_file(path) {
                    warn!("Failed to create default config file: {e}");
                    warn!("Perhaps you need extra permissions and/or have a full disk?");
                }
                Self::default()
            }
        }
    }
    // kept invisible for users 'cause we don't want people to parse toml files
    // in an otherwise simple method.
    fn default() -> Self {
        // tests are run to test if the default config parses.
        // if this fail in the release build, someone must've borked something.
        toml::from_str(Self::DEFAULT_CONFIG).expect(
            "Default config failed to parse! THIS IS A BUG! Please report this to our issue tracker!"
        )
    }
    fn create_default_config_file(path: &Path) -> std::io::Result<()> {
        let mut f = File::create(path)?;
        f.write_all(Self::DEFAULT_CONFIG.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_config_parses() {
        toml::from_str::<Config>(include_str!("default_config.toml")).unwrap();
    }
}
