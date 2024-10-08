mod proto {
  include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
  // include!(concat!(env!("OUT_DIR"), "/protos.rs"));
}
pub use proto::api;

mod connection;
mod model;
pub use connection::Connection;
mod client;


pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
