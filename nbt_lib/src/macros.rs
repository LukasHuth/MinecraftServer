//! This module contains macros needed to work with a [`NbtValue`]
/// creates a Compound for data inserted
#[macro_export]
macro_rules! create_compound_map {
    ($($name:ident : $data:expr),*) => {
        [
            $(
                (stringify!($name).to_string(), $data),
            )*
        ].into_iter().collect::<HashMap<std::string::String, NbtValue>>()
    };
}
/// Unwraps an [`Option<NbtValue>`] and converts it to a specific type using a method named `as_<type>`.
///
/// # Examples
///
/// ```
/// use nbt_lib::unwrap_to_empty;
/// use nbt_lib::NbtValue;
///
/// fn test() -> Result<i32, ()> {
///     let data: Option<NbtValue> = Some(NbtValue::Int(42));
///     let result: i32 = unwrap_to_empty!(data, i32);
///     Ok(result)
/// }
/// assert_eq!(test(), Ok(42))
/// ```
///
/// ```
/// use nbt_lib::unwrap_to_empty;
/// use nbt_lib::NbtValue;
/// use std::collections::HashMap;
///
/// fn test() -> Result<i32, ()> {
///     let mut map = HashMap::new();
///     map.insert("key".to_string(), NbtValue::Int(42));
///     let data: HashMap<String, NbtValue> = map;
///     let result: i32 = unwrap_to_empty!(data, "key", i32);
///     Ok(result)
/// }
/// assert_eq!(test(), Ok(42))
/// ```
///
/// # Errors
///
/// Returns an error if the option is `None` or if the conversion fails.
#[macro_export]
macro_rules! unwrap_to_empty {
    ($data:expr, $to:ident) => {
        paste::paste! {
            $data.ok_or(())?.[<as_ $to>]().map_err(|_|())?
        }
    };
    ($data:expr, $name:expr, $to:ident) => {
        paste::paste! {
            $data.get($name).ok_or(())?.[<as_ $to>]().map_err(|_|())?
        }
    };
}
/// Unwraps an [`Option<NbtValue>`] and converts it into a specified type.
///
/// # Examples
///
/// ```
/// use nbt_lib::unwrap_to;
/// use nbt_lib::NbtValue;
/// use nbt_lib::traits::FromNbtValue;
///
/// #[derive(Debug, PartialEq)]
/// struct Test { data: i32 };
///
/// impl FromNbtValue for Test {
///     fn from_nbt_value(value: NbtValue) -> Result<Self, ()> {
///         match value {
///             NbtValue::Int(v) => Ok(Self { data: v }),
///             _ => Err(())
///         }
///     }
/// }
///
/// fn test() -> Result<Test, ()> {
///     let data: Option<NbtValue> = Some(NbtValue::Int(42));
///     let result: Test = unwrap_to!(data, Test);
///     Ok(result)
/// }
/// assert_eq!(test(), Ok(Test { data: 42 }));
/// ```
///
/// ```
/// use nbt_lib::unwrap_to;
/// use nbt_lib::NbtValue;
/// use std::collections::HashMap;
/// use nbt_lib::traits::FromNbtValue;
///
/// #[derive(Debug, PartialEq)]
/// struct Test { data: i32 };
///
/// impl FromNbtValue for Test {
///     fn from_nbt_value(value: NbtValue) -> Result<Self, ()> {
///         match value {
///             NbtValue::Int(v) => Ok(Self { data: v }),
///             _ => Err(())
///         }
///     }
/// }
///
/// fn test() -> Result<Test, ()> {
///     let mut map = HashMap::new();
///     map.insert("data".to_string(), NbtValue::Int(42));
///     let data: HashMap<String, NbtValue> = map;
///     let result: Test = unwrap_to!(data, "data", Test);
///     Ok(result)
/// }
/// assert_eq!(test(), Ok(Test { data: 42 }));
/// ```
///
/// # Errors
///
/// Returns an error if the option is `None` or if the conversion fails.
///
/// [`NbtValue`]: `nbt_lib::NbtValue`
#[macro_export]
macro_rules! unwrap_to {
    ($data:expr, $t:ty) => {
        paste::paste! {
            <$t>::from_nbt_value($data.ok_or(())?.to_owned()).map_err(|_|())?
        }
    };
    ($data:expr, $name:expr, $t:ty) => {
        unwrap_to!($data.get($name), $t)
    };
}
/// Unwraps an [`Option<NbtValue>`] and converts it to a specific type if the key exists, otherwise returns `None`.
///
/// # Examples
///
/// ```
/// use nbt_lib::{unwrap_to_empty_if_exists, NbtValue};
/// use std::collections::HashMap;
///
/// fn test() -> Result<Option<i32>, ()> {
///     let mut map = HashMap::new();
///     map.insert("key".to_string(), NbtValue::Int(42));
///     let data: HashMap<String, NbtValue> = map;
///     let result: Option<i32> = unwrap_to_empty_if_exists!(data, "key", i32);
///     Ok(result)
/// }
/// assert_eq!(test(), Ok(Some(42)));
/// ```
///
/// # Errors
///
/// Returns an error if the option is `None` or if the conversion fails.
#[macro_export]
macro_rules! unwrap_to_empty_if_exists {
    ($data:expr, $name:expr, $to:ident) => {
        paste::paste! {
            if $data.contains_key($name) { Some($data.get($name).ok_or(())?.[<as_ $to>]().map_err(|_|())? ) } else { None }
        }
    };
}
/// Converts a list of [`NbtValue`]s to a vector of a specified type.
///
/// # Examples
///
/// ```
/// use nbt_lib::{convert_list_to, NbtValue, traits::FromNbtValue};
///
/// #[derive(Debug, PartialEq)]
/// struct Data(i32);
///
/// impl FromNbtValue for Data {
///     fn from_nbt_value(value: NbtValue) -> Result<Self, ()> {
///         match value {
///             NbtValue::Int(v) => Ok(Data(v)),
///             _ => Err(()),
///         }
///     }
/// }
///
/// fn test() -> Result<Vec<Data>, ()> {
///     let data: Option<NbtValue> = Some(NbtValue::List(vec![NbtValue::Int(1), NbtValue::Int(2)]));
///     let result: Vec<Data> = convert_list_to!(data, Data);
///     Ok(result)
/// }
/// assert_eq!(test(), Ok(vec![Data(1), Data(2)]));
/// ```
///
/// # Errors
///
/// Returns an error if the option is `None` or if the conversion fails.
#[macro_export]
macro_rules! convert_list_to {
    ($data:expr, $t:ty) => {
        nbt_lib::unwrap_to_empty!($data, list)
            .into_iter()
            .map(|data| <$t>::from_nbt_value(data))
            .collect::<Result<Vec<_>, ()>>()?
    };
}
/// Converts a list of [`NbtValue`]s to a `Result` with a vector of a specified type or an error.
///
/// # Examples
///
/// ```
/// use nbt_lib::{convert_list_to_result, NbtValue, traits::FromNbtValue};
///
/// #[derive(Debug, PartialEq)]
/// struct Data(i32);
///
/// impl FromNbtValue for Data {
///     fn from_nbt_value(value: NbtValue) -> Result<Self, ()> {
///         match value {
///             NbtValue::Int(v) => Ok(Data(v)),
///             _ => Err(()),
///         }
///     }
/// }
///
/// fn test() -> Result<Vec<Data>, ()> {
///     let data: Option<NbtValue> = Some(NbtValue::List(vec![NbtValue::Int(1), NbtValue::Int(2)]));
///     let result: Result<Vec<Data>, ()> = convert_list_to_result!(data, Data);
///     result
/// }
/// assert_eq!(test(), Ok(vec![Data(1), Data(2)]));
/// ```
///
/// # Errors
///
/// Returns an error if the option is `None` or if the conversion fails.
#[macro_export]
macro_rules! convert_list_to_result {
    ($data:expr, $t:ty) => {
        nbt_lib::unwrap_to_empty!($data, list)
            .into_iter()
            .map(|data| <$t>::from_nbt_value(data))
            .collect::<Result<Vec<_>, ()>>()
    };
}
///
#[macro_export]
macro_rules! convert_to_bool {
    ($data:expr) => {
        $data == 1
    };
}
