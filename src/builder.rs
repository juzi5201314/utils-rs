#[macro_export]
macro_rules! builder {
    ($(#[$attr:meta])* $vis:vis $name:ident { $($(#[$field_attr:meta])* $field_name:ident: $field_type:ty $(=> { $(#[$setter_attr:meta])* $($t:tt)* })?),* }) => {
        $(#[$attr])* $vis struct $name {
            $($(#[$field_attr])* $field_name: $field_type),*
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new() -> Self {
                Default::default()
            }
        }

        $($crate::builder_setter!{$name, $field_name, $field_type $(=> { $(#[$setter_attr])* $($t)* })? })*
    };
}

#[macro_export]
macro_rules! builder_setter {
    ($name:ident, $field_name:ident, $field_type:ty => { $(#[$setter_attr:meta])* $($t:tt)* }) => {
        impl $name {
            #[inline]
            #[allow(dead_code)]
            $(#[$setter_attr:meta])*
            pub fn $($t)*
        }
    };
    ($name:ident, $field_name:ident, $field_type:ty) => {
        impl $name {
            #[inline]
            #[allow(dead_code)]
            pub fn $field_name(mut self, t: $field_type) -> Self {
                self.$field_name = t;
                self
            }
        }
    };
}
