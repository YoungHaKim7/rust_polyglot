// https://riptutorial.com/ebook/rust
use serde::de::Visitor;
use serde::Deserializer;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Result, Value};

macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum $name {
        $($variant,)*
    }

    impl Serialize for $name {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer,
        {
            // Serialize the enum as a string.
            serializer.serialize_str(match *self {
            $( $name::$variant => $str, )*
            })
        }
    }
    impl Deserialize<'_> for $name {
        fn deserialize<'a, D>(deserializer: D) -> Result<Self, D::Error>
            where D: Deserializer<'a>,
    {
        struct Visitor;

        impl Visitor for Visitor {
            type Value = $name;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a string for {}", stringify!($name))
            }
            fn visit_str<E>(self, value: &str) -> Result<$name, E>
                where E: serde::de::Error,
            {
                match value {
                    $( $str => Ok($name::$variant), )*
                    _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                    &format!("unknown {} variant: {}", stringify!($name), value)
                    ), &self)),
                }
            }
        }
        // Deserialize the enum from a string.
        deserializer.deserialize_str(Visitor)
        }
    }
}
}

enum_str! (LanguageCode {
    English("en"),
    Spanish("es"),
    Italian("it"),
    Japanese("ja"),
    Chinese("zh"),
});

fn main() {
    use LanguageCode::*;
    let languages = vec![English, Spanish, Italian, Japanese, Chinese];
    // Prints ["en","es","it","ja","zh"]
    println!("{}", serde_json::to_string(&languages).unwrap());
    let input = r#" "ja" "#;
    assert_eq!(Japanese, serde_json::from_str(input).unwrap());
}
