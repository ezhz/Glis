
use std::{fmt::Display, result::Result};

// ------------------------------------------------------------

pub trait AbortExtension
{
    type T;
    type E;

    fn aborts(self) -> Self::T
    where Self::E: Display;
}

impl<T, E> AbortExtension for Result<T, E>
where E: std::fmt::Display
{
    type T = T;
    type E = E;

    fn aborts(self) -> T
    where E: Display
    {
        self.map_err
        (
            |error|
            {
                eprintln!("{error}");
                msgbox::create
                (
                    "",
                    &error.to_string(),
                    msgbox::IconType::Error
                ).unwrap();
                std::process::exit(1)
            }
        ).unwrap()
    }
}

