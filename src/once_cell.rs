/// ```rust
/// use utils_soeur::lazy;
///
/// lazy!(pub STATE: String {
///     String::from("hi")
/// });
/// ```
#[macro_export]
macro_rules! lazy {
    ($vis:vis $name:ident: $ty:ty { $init:expr }) => {
        $vis static $name: once_cell::sync::Lazy<$ty> = once_cell::sync::Lazy::new(|| $init);
    };
}

/// ```rust
/// use utils_soeur::once_cell;
///
/// once_cell!(pub QUEUE: Vec<String>);
/// ```
#[macro_export]
macro_rules! once_cell {
    ($vis:vis $name:ident: $ty:ty) => {
        $vis static $name: once_cell::sync::OnceCell<$ty> = once_cell::sync::OnceCell::new();
    };
}
