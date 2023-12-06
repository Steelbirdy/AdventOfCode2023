macro_rules! regex {
    ($NAME:ident = $pat:literal) => {
        pub static $NAME: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($pat).unwrap());
    };
}
