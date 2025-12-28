use crate::config::current_config::HtrsConfig;
use crate::config::versioned_config::versions::v1::v1config::HtrsConfigV1;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
enum VersionedHtrsConfig {
    Current(HtrsConfig),
    V1(HtrsConfigV1),
}