#[cfg(target_os = "linux")]
pub mod hid;
#[cfg(target_os = "linux")]
pub use hid::*;

#[cfg(not(target_os = "linux"))]
pub mod stub;
#[cfg(not(target_os = "linux"))]
pub use stub::*;

