// src/lib.rs

#![doc = include_str!("../README.md")]

// | Roles                        | Names        | Law Mode       |
// |------------------------------|--------------|----------------|
// | Strict Read (Option)         | biopsy       | scan           |
// | Strict Write (Mut)           | incise       | radio_knife    |
// | Strict Delete                | excise       | amputate       |
// | Genesis Write (Auto-Create)  | suture       | takt           |
// | Genesis Force Write          | force suture | gamma_knife    |
// | Genesis Read (Result)        | acquire      | mes            |
// | Genesis Merge (Merge/Concat) | graft        | shambles       |
// | Genesis Fill (Void Filler)   | implant      | injection_shot |
// | Deploy (Room Expansion)      | mesh         | room           |

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
    pub use crate::_excise as excise;
    pub use crate::_inject as incise;
    pub use crate::_probe as biopsy;
}

/// Mode B: Genesis
/// The Creator: Cultivates and grows new paths.
pub mod genesis {
    pub use crate::_cultivate as suture;
    pub use crate::_deploy as mesh;
    pub use crate::_extract as acquire;
    pub use crate::_force_cultivate as force_suture;
    pub use crate::_graft as graft;
    pub use crate::_implant as implant;
}

/// Mode One Piece: Law Mode
#[cfg(feature = "law_mode")]
pub mod law {
    //! The "Ope Ope no Mi" Interface.

    // SCAN (biopsy)
    pub use crate::strict::biopsy as scan;
    // RADIO KNIFE (incise)
    pub use crate::strict::incise as radio_knife;
    // AMPUTATE (excise)
    pub use crate::strict::excise as amputate;
    // MES (acquire)
    pub use crate::genesis::acquire as mes;
    // TAKT (suture)
    pub use crate::genesis::suture as takt;
    // GAMMA KNIFE (force suture)
    pub use crate::genesis::force_suture as gamma_knife;
    // SHAMBLES (graft)
    pub use crate::genesis::graft as shambles;
    // INGECTION SHOT (implant)
    pub use crate::genesis::implant as ingection_shot;
    // ROOM (mesh)
    pub use crate::genesis::mesh as room;
}
