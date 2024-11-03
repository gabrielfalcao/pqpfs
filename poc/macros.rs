#[macro_export]
macro_rules! data {
    ( $( $x:expr ),* ) => {
        {
            let mut data = Vec::new();
            $(
                data.push($x);
            )*
            Data::from(data)
        }
    };
}
