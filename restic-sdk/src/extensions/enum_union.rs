#[macro_export]
macro_rules! enum_union {
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
        $(#[$enum_attr])*
        $enum_vis enum $enum_name {
            $(
                $(#[$variant_attr])*
                $variant_name($variant_name),
            )*
        }

        $(
            #[automatically_derived]
            impl From<$variant_name> for $enum_name {
                fn from(value: $variant_name) -> Self {
                    $enum_name::$variant_name(value)
                }
            }

            #[automatically_derived]
            impl TryFrom<$enum_name> for $variant_name {
                // Just using a string because macro's have no method of concatenating idents outside of nightly.
                type Error = String;

                fn try_from(value: $enum_name) ->  Result<Self, Self::Error> {
                    match value {
                        $enum_name::$variant_name(inner) => Ok(inner),
                        _ => Err(
                                format!(
                                    "Failed to convert enum variant {}::{} to {}",
                                    stringify!($enum_name),
                                    stringify!($variant_name),
                                    stringify!($variant_name)
                                )
                            ),
                    }
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    pub struct A {}
    pub struct B {}

    enum_union! {
        pub enum UnionTest {
            A,
            B,
        }
    }

    #[test]
    fn can_into() {
        let result: UnionTest = A {}.into();
        match result {
            UnionTest::A(_) => {}
            _ => panic!("Expected A"),
        }
    }

    #[test]
    fn can_try_into() {
        A::try_from(UnionTest::A(A {})).expect("convert to A");
    }
}
