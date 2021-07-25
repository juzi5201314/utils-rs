
/// 快速构建一个[std::collections::HashMap]
///
/// ```rust
/// use utils_rs::hashmap;
///
/// hashmap! {
///     1 => 'a'
/// };
/// ```
///
/// with_capacity:
/// ```
/// use utils_rs::hashmap;
///
/// hashmap! (2, {
///     1 => 'a',
///     2 => 'b'
/// });
/// ```
#[macro_export]
macro_rules! hashmap {
    ($capacity:expr, { $($key:expr => $value:expr),* }) => {
        {
            let mut _map = std::collections::HashMap::with_capacity($capacity);
            $( _map.insert($key, $value); )*
            _map
        }
    };

    (capacity = $capacity:expr, { $($key:expr => $value:expr),* }) => {
        $crate::hashmap!($capacity, { $($key => $value),* })
    };

    { $($key:expr => $value:expr),* } => {
        $crate::hashmap!(0, { $($key => $value),* })
    };
}

#[test]
fn test_hashmap() {
    let mut map = std::collections::HashMap::new();
    map.insert(1, 2);

    let map2 = hashmap! {
        1 => 2
    };

    assert_eq!(map, map2);
}
