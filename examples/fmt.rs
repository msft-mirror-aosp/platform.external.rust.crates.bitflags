//! An example of implementing Rust's standard formatting and parsing traits for flags types.

use core::{fmt, str};

fn main() -> Result<(), bitflags::parser::ParseError> {
    bitflags::bitflags! {
        // You can `#[derive]` the `Debug` trait, but implementing it manually
        // can produce output like `A | B` instead of `Flags(A | B)`.
        // #[derive(Debug)]
        #[derive(PartialEq, Eq)]
        pub struct Flags: u32 {
            const A = 1;
            const B = 2;
            const C = 4;
            const D = 8;
        }
    }

    impl fmt::Debug for Flags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    impl fmt::Display for Flags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    impl str::FromStr for Flags {
        type Err = bitflags::parser::ParseError;

        fn from_str(flags: &str) -> Result<Self, Self::Err> {
            Ok(Self(flags.parse()?))
        }
    }

    let flags = Flags::A | Flags::B;

    println!("{}", flags);

    let formatted = flags.to_string();
    let parsed: Flags = formatted.parse()?;

    assert_eq!(flags, parsed);

    Ok(())
}
