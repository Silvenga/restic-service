use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ResticConfig {
    pub environment: HashMap<String, String>,
}
