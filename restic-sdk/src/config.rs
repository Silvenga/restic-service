use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ResticConfig {
    pub environment: HashMap<String, String>,
}
