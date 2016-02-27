/*!
A binding for SDL2_ttf.
 */

extern crate sdl2;
extern crate sdl2_sys;

#[macro_use]
extern crate bitflags;

use std::fmt;
use std::error;

#[allow(non_camel_case_types, dead_code)]
mod ffi;
mod font;
mod context;

pub type SdlResult<T> = Result<T, String>;

#[derive(Clone,Eq,PartialEq,Hash,Debug)]
pub struct ErrorMessage(pub String);


impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SDL error: {}", self.0)
    }
}

impl error::Error for ErrorMessage {
    fn description(&self) -> &str {
        "SDL error"
    }
}

impl From<String> for ErrorMessage {
    fn from(src: String) -> Self {
        ErrorMessage(src)
    }
}


// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_ttf")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_ttf")]
    extern {}
}

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
mod others {
    #[link(name="SDL2_ttf")]
    extern {}
}

pub use context::{
    init, has_been_initialized, get_linked_version, Sdl2TtfContext
};
pub use font::{
    Font, FontStyle, Hinting, GlyphMetrics, PartialRendering, FontError, 
    FontResult,
};
