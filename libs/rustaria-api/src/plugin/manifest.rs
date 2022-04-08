use std::collections::HashMap;

use semver::Version;

use serde::{Deserialize, Serialize};

use crate::plugin::id::PluginId;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub id: PluginId,
    pub name: String,
    pub version: Version,
    // Entry points
    pub common_pre_entry: Option<String>,
    pub common_entry: Option<String>,
    #[cfg(any(feature = "client"))]
    pub client_pre_entry: Option<String>,
    #[cfg(any(feature = "client"))]
    pub client_entry: Option<String>,
    // Other plugins
    #[serde(default)]
    pub dependencies: HashMap<PluginId, Version>,
    #[serde(default)]
    pub incompatibilities: HashMap<PluginId, Version>,
    #[serde(default)]
    pub recommendations: HashMap<PluginId, Version>,
}
