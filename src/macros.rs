#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! map {
        ( $( $k:expr => $v:expr ),+ ) => {
            {
                let mut temp_map = ::std::collections::BTreeMap::new();
                $(
                    temp_map.insert($k, $v);
                )+
                    temp_map
            }
        };
    }
}
