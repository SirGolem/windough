use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    #[serde(default = "retry_count_default")]
    pub retry_count: usize,
    #[serde(default = "retry_interval_default")]
    pub retry_interval: usize,
}

const fn retry_count_default() -> usize {
    5
}
const fn retry_interval_default() -> usize {
    750
}
