#[cfg(feature = "jar")]
mod jar;

mod jar_stub;

#[cfg(feature = "jar")]
pub use jar::Jar;

#[cfg(not(feature = "jar"))]
pub use jar_stub::Jar;
