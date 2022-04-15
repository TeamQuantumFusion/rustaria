use std::fmt::Display;

#[derive(Debug)]
pub enum SmartError {
    CarrierUnavailable
}

impl Display for SmartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartError::CarrierUnavailable => f.write_str("Carrier is unavailable, Force reloading instance."),
        }
    }
}