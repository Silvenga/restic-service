//! A Rust SDK for interacting with the restic cli over json messages.

mod arg_builder;
mod client;
mod commands;
mod config;
mod extensions;

mod parsing;

pub mod errors;
pub mod messages;

pub use arg_builder::*;
pub use client::*;
pub use commands::*;
pub use config::*;
