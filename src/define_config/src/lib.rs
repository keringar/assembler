#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
pub extern crate serde_yaml;

pub use serde_yaml::{Error, Value};
pub use serde::{Serialize, Deserialize};
pub use serde::de::Deserializer;
pub use serde_derive::*;

/// Defines a YAML serializable configuration struct that
/// implements `Default`, `Display` and `FromStr` with the given values.
///
/// Note: Use of this macro requires serde to be in your Cargo.toml
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate define_config;
///
/// use std::str::FromStr;
///
/// define_config! {
///     #[derive(Clone)]
///     Person {
///         name: String = String::default(),
///         age: u32 = 0,
///     }
/// }
///
/// define_config! {
///     Family {
///         father: Person = Person::default(),
///         mother: Person = Person::default(),
///         children: Option<Vec<Person>> = None,
///     }
/// }
///
/// fn main() {
///     let father = Person {
///         name: "Chris".to_string(),
///         age: 41,
///     };
///
///     let mother = Person {
///         name: "Nyarlathotep the Crawling Chaos".to_string(),
///         age: <u32>::max_value(),
///     };
///
///     println!("{:?}", mother);
///     println!("{:?}", father);
///
///     let family = Family {
///         father: father,
///         mother: mother,
///         children: None,
///     };
///
///     let serialized_string = family.to_string();
///     println!("{}", serialized_string);
///
///     let family = Family::from_str(&serialized_string).unwrap();
///
///     assert!(family.father.age == 41);
///     assert!(family.mother.name == "Nyarlathotep the Crawling Chaos".to_string());
/// }
/// ```
///
#[macro_export]
macro_rules! define_config {
    (   #[derive( $attr_before:ident $(, $attr:ident )* )]
        $name:ident {
            $( $field:ident : $ty:ty = $e:expr, )+
        }
    ) => {
        #[derive( Serialize, Deserialize, Debug, $attr_before $( ,$attr )* )]
        pub struct $name {
            $( 
               pub $field : $ty, )+
        }

        impl_config_meta! {
            $name {
                $( $field, $ty, $e,)+
            }
        }
    };

    ( $name:ident {
          $( $field:ident : $ty:ty = $e:expr, )+
      }
    ) => {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $name {
            $( pub $field : $ty ,)+
        }

        impl_field_defaults! {
            $name {
                $( $field, $ty, $e,)+
            }
        }

        impl_config_meta! {
            $name {
                $( $field, $ty, $e,)+
            }
        }
     };
}

#[doc(no_inline)]
#[macro_export]
macro_rules! impl_config_meta {
    ( $name:ident {
        $( $field:ident, $ty:ty, $e:expr,)+
    }) => {
        impl Default for $name {
            fn default() -> $name {
                $name {
                    $( $field: $e ,)+
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let string = match define_config::serde_yaml::to_string(self) {
                    Ok(string) => string,
                    Err(_) => return Err(::std::fmt::Error),
                };
                write!(f, "{}", string)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = define_config::serde_yaml::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                define_config::serde_yaml::from_str(s)
            }
        }
    }
}