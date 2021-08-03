#[macro_export]
macro_rules! builder {
    ($(#[$attr:meta])* $vis:vis $name:ident$(<$($g:ident),*>)? { $($(#[$field_attr:meta])* $field_name:ident: $field_type:ty $(=> { $(#[$setter_attr:meta])* $($t:tt)* })?),* }) => {
        $(#[$attr])* $vis struct $name$(<$($g),*>)? {
            $($(#[$field_attr])* $field_name: $field_type),*
        }

        impl$(<$($g),*>)? $name$(<$($g),*>)? {
            #[allow(dead_code)]
            pub fn new() -> Self {
                Default::default()
            }
        }

        impl$(<$($g),*>)? $name$(<$($g),*>)? {
            $($crate::builder_setter!{$field_name, $field_type $(=> { $(#[$setter_attr])* $($t)* })? })*
        }
    };
}

#[macro_export]
macro_rules! builder_setter {
    ($field_name:ident, $field_type:ty => { $(#[$setter_attr:meta])* $($t:tt)* }) => {
        #[inline]
        #[allow(dead_code)]
        $(#[$setter_attr:meta])*
        pub fn $($t)*
    };
    ($field_name:ident, $field_type:ty) => {
        #[inline]
        #[allow(dead_code)]
        pub fn $field_name(mut self, t: $field_type) -> Self {
            self.$field_name = t;
            self
        }
    };
}
