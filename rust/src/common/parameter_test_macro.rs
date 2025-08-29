#[macro_export]
macro_rules! parameter_tests {
    {$func:ident, $(($name:ident: $value:expr)),* $(,)?} => {
    $(
        #[test]
        fn $name() {
            $func($value);
        }
    )*
    }
}
