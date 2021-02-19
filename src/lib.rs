//! x0rs is a Rust library for using the x0 HTTP API
//! To get started, take a look at [Client].

pub mod client;
pub mod error;
mod http;
pub mod model;
#[cfg(test)]
mod test;

pub use client::Client;
