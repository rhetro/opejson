// src/prelude.rs

// ! The opejson Prelude.
// !
// ! import: `use opejson::prelude::*;`

#[doc(hidden)]
pub use crate::serde_json::{self, json, Value};

pub use crate::genesis::*;
pub use crate::strict::*;
