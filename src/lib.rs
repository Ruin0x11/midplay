#[cfg(windows)] extern crate winapi;
extern crate midir;
extern crate midly;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate anyhow;

pub mod generic;

pub mod native {
    #[cfg(target_os="windows")] mod windows;
    #[cfg(target_os="windows")] pub use self::windows::*;

    #[cfg(target_os="linux")] mod linux;
    #[cfg(target_os="linux")] pub use self::linux::*;

    #[cfg(target_os="macos")] mod macos;
    #[cfg(target_os="macos")] pub use self::macos::*;
}
