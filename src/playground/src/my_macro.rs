#[macro_export]
macro_rules! my_str {
    ( $( $x:expr ),* ) => {
        {
            let mut out = String::new();
            $(
                out.push_str($x);
            )*
            out
        }
    };
}

#[macro_export]
macro_rules! four {
    () => { 1 + 3 };
}