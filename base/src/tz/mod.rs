#[cfg(all(target_arch = "wasm32", feature = "browser"))]
mod browser_tz;
#[cfg(all(target_arch = "wasm32", feature = "browser"))]
pub(crate) use browser_tz::*;

#[cfg(not(all(target_arch = "wasm32", feature = "browser")))]
mod os_tz;
#[cfg(not(all(target_arch = "wasm32", feature = "browser")))]
pub(crate) use os_tz::*;
