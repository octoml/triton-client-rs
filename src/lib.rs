#[cfg(feature = "tls")]
pub mod client;

pub mod inference {
    include!(concat!(env!("OUT_DIR"), "/inference.rs"));
}
