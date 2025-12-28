#[macro_export]
macro_rules! restic_message {
    (
        $(#[$enum_attr:meta])*
        $enum_vis:vis enum $enum_name:ident
        {
            $(
                $(#[$variant_attr:meta])*
                $variant_name:ident
            ),*
            $(,)?
        }
    ) => {
        $crate::enum_union! {
            $(#[$enum_attr])*
            #[derive(serde::Deserialize, Debug, Clone, PartialEq)]
            #[serde(tag = "message_type")]
            $enum_vis enum $enum_name {
                $(
                    $(#[$variant_attr])*
                    $variant_name,
                )*
                #[serde(rename = "exit_error")]
                ExitError,
            }
        }

        #[automatically_derived]
        impl $crate::parsing::ResticMessage for $enum_name {
            fn parse_message(message: &str) -> Result<Self, $crate::parsing::ParseError> {
                serde_json::from_str(message).map_err($crate::parsing::ParseError::SerdeError)
            }
        }
    };
}
