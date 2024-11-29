#[macro_export]
macro_rules! data {
    ( $( $x:expr ),* ) => {
        {
            use pqpfs::Data;
            let mut data = Vec::new();
            $(
                data.push($x);
            )*
            Data::from(data)
        }
    };
}
