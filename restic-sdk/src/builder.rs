use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct CommandBuilder<'a> {
    verb: Option<&'a str>,
    values: Vec<&'a str>,
    arguments: HashSet<Argument<'a>>,
}

#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Argument<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> CommandBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_verb(mut self, verb: &'a str) -> Self {
        self.verb = Some(verb);
        self
    }

    pub fn with_flag_and_value(mut self, name: &'a str, value: &'a str) -> Self {
        self.arguments.insert(Argument {
            name,
            value: Some(value),
        });
        self
    }

    pub fn with_flag(mut self, name: &'a str) -> Self {
        self.arguments.insert(Argument { name, value: None });
        self
    }

    pub fn with_value(mut self, value: &'a str) -> Self {
        self.values = vec![value];
        self
    }

    pub fn with_values(mut self, value: impl Iterator<Item = &'a str>) -> Self {
        self.values = value.collect();
        self
    }

    pub fn build(self) -> Vec<String> {
        let mut full_arguments = vec![];

        if let Some(verb) = self.verb {
            full_arguments.push(verb.to_owned());
        }

        for arg in self.arguments {
            full_arguments.push(format!("--{}", arg.name));

            if let Some(value) = arg.value {
                full_arguments.push(value.to_owned());
            }
        }

        for value in self.values {
            full_arguments.push(value.to_owned());
        }

        full_arguments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_empty_return_empty() {
        let command = CommandBuilder::new().build();
        assert!(command.is_empty());
    }

    #[test]
    fn when_verb_then_start_with_verb() {
        let command = CommandBuilder::new()
            .with_verb("verb")
            .with_flag("flag")
            .build();
        assert_eq!(command, vec!["verb", "--flag"]);
    }

    #[test]
    fn when_flag_with_value_then_value_as_separate_token() {
        let command = CommandBuilder::new()
            .with_flag_and_value("flag", "value")
            .build();
        assert_eq!(command, vec!["--flag", "value"]);
    }

    #[test]
    fn when_flag_with_duplicate_value_then_ignore_duplicate() {
        let command = CommandBuilder::new()
            .with_flag_and_value("flag", "value")
            .with_flag_and_value("flag", "value")
            .build();
        assert_eq!(command, vec!["--flag", "value"]);
    }

    #[test]
    fn when_value_then_end_with_value() {
        let command = CommandBuilder::new()
            .with_flag_and_value("flag", "value")
            .with_value("end_value")
            .build();
        assert_eq!(command, vec!["--flag", "value", "end_value"]);
    }
}
