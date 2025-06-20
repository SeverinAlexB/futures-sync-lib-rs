mod say_hello;
#[cfg(feature = "sync")]
pub mod my_rpc;

pub use say_hello::say_hello;