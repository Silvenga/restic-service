use crate::config::ServiceConfiguration;
use toml::de::Error;

pub fn parse_configuration(config: &str) -> Result<ServiceConfiguration, Error> {
    toml::from_str(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_handle_empty_config() {
        let config = r#"
            version = 1
        "#;

        let result: ServiceConfiguration = parse_configuration(config).unwrap();

        assert_eq!(result.version, 1);
    }

    #[test]
    fn can_handle_minimal_config() {
        let config = r#"
            version = 1

            [jobs.job1]
            cron = "0 0 * * *"
            repository = "C:\\Some Path\\"
            password = "secret"
        "#;

        let result: ServiceConfiguration = parse_configuration(config).unwrap();

        assert_eq!(result.version, 1);
        assert_eq!(result.jobs.len(), 1);
        assert_eq!(result.jobs["job1"].cron, "0 0 * * *");
    }
}
