use ordermap::OrderSet;

#[derive(Debug, Clone, Default)]
pub struct ArgumentsBuilder {
    arguments: OrderSet<Argument>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Argument {
    Verb { verb: String },
    Flag { name: String },
    FlagWithValue { name: String, value: String },
    Value { value: String },
}

impl ArgumentsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_verb(self, verb: impl Into<String>) -> Self {
        self.push_argument(Argument::Verb { verb: verb.into() })
    }

    pub fn with_flag_and_value<V: BuilderValue>(self, name: impl Into<String>, value: V) -> Self {
        self.push_argument(Argument::FlagWithValue {
            name: name.into(),
            value: value.to_builder_value(),
        })
    }

    pub fn with_flag(self, name: impl Into<String>) -> Self {
        self.push_argument(Argument::Flag { name: name.into() })
    }

    pub fn with_value(self, value: impl Into<String>) -> Self {
        self.push_argument(Argument::Value {
            value: value.into(),
        })
    }

    pub fn with_values(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for value in values {
            self.arguments.insert(Argument::Value {
                value: value.into(),
            });
        }
        self
    }

    pub fn build(self) -> Vec<String> {
        let mut full_arguments = vec![];

        for argument in self.arguments {
            match argument {
                Argument::Verb { verb } => {
                    full_arguments.push(verb);
                }
                Argument::Flag { name } => {
                    full_arguments.push(format!("--{}", name));
                }
                Argument::FlagWithValue { name, value } => {
                    full_arguments.push(format!("--{}", name));
                    full_arguments.push(value);
                }
                Argument::Value { value } => {
                    full_arguments.push(value);
                }
            }
        }

        full_arguments
    }

    fn push_argument(mut self, argument: Argument) -> Self {
        self.arguments.insert(argument);
        self
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
    fn when_value_flag_and_verb_then_ordered_correctly() {
        let command = ArgumentsBuilder::new()
            .with_verb("list")
            .with_value("locks")
            .with_flag("json")
            .build();
        assert_eq!(command, vec!["list", "locks", "--json"]);
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
