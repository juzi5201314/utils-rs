#[macro_export]
macro_rules! ty_enum {
    ($(#[$attr:meta])* enum $name:ident : $typ:ty { $($k:ident = $v:expr),* }) => {
        $(#[$attr])*
        enum $name {
            $($k),*
        }

        impl From<$name> for $typ {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$k => $v),*
                }
            }
        }

        impl std::convert::TryFrom<$typ> for $name {
            type Error = $typ;

            fn try_from(value: $typ) -> Result<Self, Self::Error> {
                match value {
                    $($v => Ok($name::$k)),*,
                    _ => Err(value)
                }
            }
        }
    }
}

#[test]
fn test_map_enum() {
    use std::convert::TryFrom;

    ty_enum! (
        #[derive(Debug, Eq, PartialEq)]
        enum Jinser: usize {
            AGE = 18,
            HEIGHT = 180
        }
    );

    let age: usize = Jinser::AGE.into();
    assert_eq!(age, 18);

    assert_eq!(Jinser::try_from(180), Ok(Jinser::HEIGHT));
    assert_eq!(Jinser::try_from(10), Err(10));
}
