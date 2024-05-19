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
#[macro_export]
macro_rules! unwrap_to_empty_if_exists {
    ($data:expr, $name:expr, $to:ident) => {
        paste::paste! {
            if $data.contains_key($name) { Some($data.get($name).ok_or(())?.[<as_ $to>]().map_err(|_|())? ) } else { None }
        }
    };
}
#[macro_export]
macro_rules! convert_list_to {
    ($data:expr, $t:ty) => {
        unwrap_to_empty!($data, list).into_iter().map(|data| <$t>::from_nbt_value(data)).collect()?
    };
}
#[macro_export]
macro_rules! convert_to_bool {
    ($data:expr) => {
        $data == 1
    };
}
