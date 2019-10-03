pub use indoc::indoc;
pub use rustc_hex::FromHex;

#[macro_export] macro_rules! inhex(
    ($l: literal) => {
        inhex::FromHex::from_hex(inhex::indoc!($l))
            .expect("Inhex expected to be used for statically checked data")
    }
);
