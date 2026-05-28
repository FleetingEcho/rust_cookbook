#![allow(
    dead_code,
    unused_assignments,
    unused_must_use,
    unused_mut,
    unused_variables
)]

//! Shared library entry point for Rust learning notes.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod advanced;
pub mod base_type;
pub mod basics;
pub mod config;
pub mod errors;
pub mod learning_additions;
pub mod ownership;
pub mod practice_core;
pub mod rust_by_example;
pub mod structs_enums;
pub mod traits;
pub mod types;
pub mod utils;

pub mod kinds {
    //! Defines types of colors.

    /// Primary colors
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// Secondary colors
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}
