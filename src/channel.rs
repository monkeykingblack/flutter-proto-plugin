use std::fmt;

use schematic::Schematic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Schematic, Deserialize, Serialize, PartialEq)]
pub enum Channel {
    #[default]
    #[serde(rename = "stable")]
    Stable,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "dev")]
    Dev,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Channel::Beta => write!(f, "beta"),
            Channel::Stable => write!(f, "stable"),
            Channel::Dev => write!(f, "dev"),
        }
    }
}
