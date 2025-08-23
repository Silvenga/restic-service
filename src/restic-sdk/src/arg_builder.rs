use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct ArgumentsBuilder {
    verb: Option<String>,
    values: Vec<String>,
    arguments: HashSet<Argument>,
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
struct Argument {
    name: String,
    value: Option<String>,
}

impl ArgumentsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_verb(mut self, verb: &str) -> Self {
        self.verb = Some(String::from(verb));
        self
    }

    pub fn with_flag_and_value<V: BuilderValue>(mut self, name: &str, value: V) -> Self {
        self.arguments.insert(Argument {
            name: String::from(name),
            value: Some(value.to_builder_value()),
        });
        self
    }

    pub fn with_flag(mut self, name: &str) -> Self {
        self.arguments.insert(Argument {
            name: String::from(name),
            value: None,
        });
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.values = vec![String::from(value)];
        self
    }

    pub fn with_values(mut self, values: impl IntoIterator<Item = impl BuilderValue>) -> Self {
        self.values = values.into_iter().map(|x| x.to_builder_value()).collect();
        self
    }

    pub fn build(self) -> Vec<String> {
        let mut full_arguments = vec![];

        if let Some(verb) = self.verb {
            full_arguments.push(verb);
        }

        for arg in self.arguments {
            full_arguments.push(format!("--{}", arg.name));

            if let Some(value) = arg.value {
                full_arguments.push(value);
            }
        }

        for value in self.values {
            full_arguments.push(value);
        }

        full_arguments
    }
}

pub trait BuilderValue {
    fn to_builder_value(&self) -> String;
}

impl BuilderValue for u32 {
    fn to_builder_value(&self) -> String {
        self.to_string()
    }
}

impl BuilderValue for f64 {
    fn to_builder_value(&self) -> String {
        self.to_string()
    }
}

impl BuilderValue for bool {
    fn to_builder_value(&self) -> String {
        self.to_string()
    }
}

impl BuilderValue for str {
    fn to_builder_value(&self) -> String {
        String::from(self)
    }
}

impl BuilderValue for &str {
    fn to_builder_value(&self) -> String {
        String::from(self.to_owned())
    }
}

impl BuilderValue for String {
    fn to_builder_value(&self) -> String {
        self.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_empty_return_empty() {
        let command = ArgumentsBuilder::new().build();
        assert!(command.is_empty());
    }

    #[test]
    fn when_verb_then_start_with_verb() {
        let command = ArgumentsBuilder::new()
            .with_verb("verb")
            .with_flag("flag")
            .build();
        assert_eq!(command, vec!["verb", "--flag"]);
    }

    #[test]
    fn when_flag_with_value_then_value_as_separate_token() {
        let command = ArgumentsBuilder::new()
            .with_flag_and_value("flag", "value")
            .build();
        assert_eq!(command, vec!["--flag", "value"]);
    }

    #[test]
    fn when_flag_with_duplicate_value_then_ignore_duplicate() {
        let command = ArgumentsBuilder::new()
            .with_flag_and_value("flag", "value")
            .with_flag_and_value("flag", "value")
            .build();
        assert_eq!(command, vec!["--flag", "value"]);
    }

    #[test]
    fn when_value_then_end_with_value() {
        let command = ArgumentsBuilder::new()
            .with_flag_and_value("flag", "value")
            .with_value("end_value")
            .build();
        assert_eq!(command, vec!["--flag", "value", "end_value"]);
    }

    #[test]
    fn can_accept_str() {
        // This compiles.
        _ = ArgumentsBuilder::new()
            .with_flag_and_value("flag", "value")
            .build();
    }

    #[test]
    fn can_accept_u32() {
        // This compiles.
        _ = ArgumentsBuilder::new()
            .with_flag_and_value("flag", 42u32)
            .build();
    }

    #[test]
    fn can_accept_f64() {
        // This compiles.
        _ = ArgumentsBuilder::new()
            .with_flag_and_value("flag", 42.0)
            .build();
    }

    #[test]
    fn can_accept_bool() {
        // This compiles.
        _ = ArgumentsBuilder::new()
            .with_flag_and_value("flag", true)
            .build();
    }

    #[test]
    fn can_accept_owned_string() {
        // This compiles.
        _ = ArgumentsBuilder::new()
            .with_flag_and_value("flag", String::from("value"))
            .build();
    }
}
