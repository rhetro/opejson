// src/lib.rs

// | Roles                       | Names      | Law Mode    |
// |-----------------------------|------------|-------------|
// | Strict Read (Option)        | biopsy     | scan        |
// | Strict Write (Mut)          | incise     | radio_knife |
// | Strict Delete               | amputate   | amputate    |
// | Genesis Write (Auto-Create) | suture     | takt        |
// | Genesis Read (Result)       | acquire    | mes         |
// | Genesis Graft (Inplant)     | graft      | shambles    |
// | Deploy (Room Expansion)     | scaffold   | room        |

pub use serde_json;

pub mod prelude;

#[macro_use]
mod core;

///Opejson Error Type
#[derive(Debug, Clone)]
pub enum Error {
    PathNotFound(String),
    TypeMismatch(String),
    IndexOutOfBounds(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PathNotFound(path) => write!(f, "Path '{}' not found", path),
            Error::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            Error::IndexOutOfBounds(idx) => write!(f, "Index {} out of bounds", idx),
        }
    }
}

impl std::error::Error for Error {}

/// Mode A: Strict
/// The Surgeon: Operates precisely on existing anatomy.
pub mod strict {
    pub use crate::_excise as amputate;
    pub use crate::_inject as incise;
    pub use crate::_probe as biopsy;
}

/// Mode B: Genesis
/// The Creator: Cultivates and grows new paths.
pub mod genesis {
    pub use crate::_cultivate as suture;
    pub use crate::_deploy as scaffold;
    pub use crate::_extract as acquire;
    pub use crate::_graft as graft;
}

/// Mode One Piece: Law Mode
#[cfg(feature = "law_mode")]
pub mod law {
    //! The "Ope Ope no Mi" Interface.

    // ROOM: SCAN (biopsy)
    pub use crate::strict::biopsy as scan;
    // ROOM: RADIO KNIFE (incise)
    pub use crate::strict::incise as radio_knife;
    // ROOM: AMPUTATE (amputate)
    pub use crate::strict::amputate;
    // ROOM: MES (acquire)
    pub use crate::genesis::acquire as mes;
    // ROOM: TAKT (suture)
    pub use crate::genesis::suture as takt;
    // ROOM: SHAMBLES (graft)
    pub use crate::genesis::graft as shambles;
    // ROOM: ROOM (allocate)
    pub use crate::genesis::scaffold as room;
}
