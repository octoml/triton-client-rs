
#![doc = include_str!("../README.md")]

#[cfg(feature = "tls")]
pub mod client;

#[cfg(feature = "tls")]
pub use client::Client;

pub mod inference {
    include!(concat!(env!("OUT_DIR"), "/inference.rs"));
}
