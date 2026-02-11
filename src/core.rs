// src/core.rs

// ====================================================
// === Opejson Core: Surgical Instruments for JSON ===
// ====================================================

// =============================
// === Opejson: Strict Mode ===
// =============================

// -----------------------------------------------------------------------------
// 1. PROBE (Strict Read) -> "biopsy" / "scan"
//    - Action: Search for a value without modifying anatomy.
//    - Returns: Option<&Value>
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_probe {
    ($val:expr) => {
        Some($val)
    };

    ($val:expr, ) => {
        Some($val)
    };

    ($val:expr, . ($key:expr) $($rest:tt)*) => {
        match $val.get($key) {
            Some(inner) => $crate::__impl_probe!(inner, $($rest)*),
            None => None,
        }
    };

    ($val:expr, . $key:literal $($rest:tt)*) => {
        match $val.get($key) {
            Some(inner) => $crate::__impl_probe!(inner, $($rest)*),
            None => None,
        }
    };

    ($val:expr, . $key:ident $($rest:tt)*) => {
        match $val.get(std::stringify!($key)) {
            Some(inner) => $crate::__impl_probe!(inner, $($rest)*),
            None => None,
        }
    };

    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {
        match $val.get($idx) {
            Some(inner) => $crate::__impl_probe!(inner, $($rest)*),
            None => None,
        }
    };
}

// --- Entry Point 1 ---
#[macro_export]
#[doc(hidden)]
macro_rules! _probe {
    ($val:expr, $($args:tt)+) => {{
        let _: &$crate::serde_json::Value = &$val;
        $crate::__impl_probe!($val, $($args)+)
    }};

    ($val:expr) => {{
        let _: &$crate::serde_json::Value = &$val;
        Some($val)
    }};
}

// -----------------------------------------------------------------------------
// 2. INJECT (Strict Write) -> "incise" / "radio_knife"
//    - Action: Modify an existing value. Fails if path is missing.
//    - Returns: Option<()>
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_inject {
    ($val:expr, = $value:expr) => {{
        *$val = $crate::serde_json::json!($value);
        Some(())
    }};

    ($val:expr) => {
        Some($val)
    };

    ($val:expr, . ($key:expr) $($rest:tt)*) => {
        match $val.get_mut($key) {
            Some(inner) => $crate::__impl_inject!(inner, $($rest)*),
            None => None,
        }
    };

    ($val:expr, . $key:literal $($rest:tt)*) => {
        match $val.get_mut($key) {
            Some(inner) => $crate::__impl_inject!(inner, $($rest)*),
            None => None,
        }
    };

    ($val:expr, . $key:ident $($rest:tt)*) => {
        match $val.get_mut(std::stringify!($key)) {
            Some(inner) => $crate::__impl_inject!(inner, $($rest)*),
            None => None,
        }
    };

    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {
        match $val.get_mut($idx) {
            Some(inner) => $crate::__impl_inject!(inner, $($rest)*),
            None => None,
        }
    };
}

// --- Entry Point 2 ---
#[macro_export]
#[doc(hidden)]
macro_rules! _inject {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_inject!((&mut $val), $($args)+)
    }};

    ($val:expr) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        Some($val)
    }};
}

// -----------------------------------------------------------------------------
// 3. EXCISE (Delete) -> "amputate" / "amputate"
//    - Action: Remove a value from object or array.
//    - Returns: Option<Value> (The removed organ)
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_excise {
    ($val:expr, . ($target:expr)) => {
        $val.as_object_mut().and_then(|o| o.remove($target))
    };

    ($val:expr, . $target:literal) => {
        $val.as_object_mut().and_then(|o| o.remove($target))
    };

    ($val:expr, . $target:ident) => {
        $val.as_object_mut()
            .and_then(|o| o.remove(std::stringify!($target)))
    };

    ($val:expr, [ $idx:expr ]) => {
        $val.as_array_mut().and_then(|a| {
            if $idx < a.len() {
                Some(a.remove($idx))
            } else {
                None
            }
        })
    };

    ($val:expr, . ($key:expr) $($rest:tt)+) => {
        match $val.get_mut($key) {
            Some(v) => $crate::__impl_excise!(v, $($rest)+),
            None => None,
        }
    };

    ($val:expr, . $key:literal $($rest:tt)+) => {
        match $val.get_mut($key) {
            Some(v) => $crate::__impl_excise!(v, $($rest)+),
            None => None,
        }
    };

    ($val:expr, . $key:ident $($rest:tt)+) => {
        match $val.get_mut(std::stringify!($key)) {
            Some(v) => $crate::__impl_excise!(v, $($rest)+),
            None => None,
        }
    };

    ($val:expr, [ $idx:expr ] $($rest:tt)+) => {
        match $val.get_mut($idx) {
            Some(v) => $crate::__impl_excise!(v, $($rest)+),
            None => None,
        }
    };
}

// --- Entry Point 3 ---
#[macro_export]
#[doc(hidden)]
macro_rules! _excise {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_excise!($val, $($args)+)
    }};
}

// ==============================
// === Opejson: Genesis Mode ===
// ==============================

// -----------------------------------------------------------------------------
// 4. CULTIVATE (Genesis Write) -> "suture" / "takt"
//    - Action: Create path if missing (Auto-vivification).
//    - Safety: Only overwrites Null, Preserves existing scalars/types.
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_cultivate {
    ($val:expr, = $value:expr) => {
        *$val = $crate::serde_json::json!($value);
    };

    ($val:expr, . ($key:expr) $($rest:tt)*) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::Object($crate::serde_json::Map::new());
        }

        if let Some(obj) = $val.as_object_mut() {
            let next = obj.entry($key).or_insert($crate::serde_json::Value::Null);
            $crate::__impl_cultivate!(next, $($rest)*);
        } else {
            #[cfg(debug_assertions)]
            eprintln!("Opejson Warning: Suture blocked. Path segment is not an Object.")
        }
    };

    ($val:expr, . $key:literal $($rest:tt)*) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::Object($crate::serde_json::Map::new());
        }

        if let Some(obj) = $val.as_object_mut() {
            let next = obj.entry($key).or_insert($crate::serde_json::Value::Null);
            $crate::__impl_cultivate!(next, $($rest)*);
        } else {
            #[cfg(debug_assertions)]
            eprintln!("Opejson Warning: Suture blocked. Path segment is not an Object.");
        }
    };

    ($val:expr, . $key:ident $($rest:tt)*) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::Object($crate::serde_json::Map::new());
        }

        if let Some(obj) = $val.as_object_mut() {
            let next = obj.entry(std::stringify!($key)).or_insert($crate::serde_json::Value::Null);
            $crate::__impl_cultivate!(next, $($rest)*);
        } else {
            #[cfg(debug_assertions)]
            eprintln!("Opejson Warning: Suture blocked. Path segment is not an Object.");
        }
    };

    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::Array(std::vec::Vec::new());
        }

        if let Some(arr) = $val.as_array_mut() {
            let idx = $idx as usize;

            if idx >= arr.len() {
                arr.resize(idx + 1, $crate::serde_json::Value::Null);
            }

            let next = &mut arr[idx];
            $crate::__impl_cultivate!(next, $($rest)*);
        } else {
            #[cfg(debug_assertions)]
            eprintln!("Opejson Warning: Suture blocked. Path segment is not an Array.")
        }
    };
}

// --- Entry Point 4 ---
#[macro_export]
#[doc(hidden)]
macro_rules! _cultivate {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_cultivate!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 5. EXTRACT (Genesis Read) -> "acquire" / "mes"
//    - Action: Get value strictly, return Error if missing.
//    - Returns: Result<T, opejson::Error>
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! _extract {
    ($val:expr, $($path:tt)+) => {{
        $crate::_probe!($val, $($path)+)
            .ok_or_else(|| $crate::Error::PathNotFound(format!("{}", std::stringify!($($path)+))))
    }};
}

// -----------------------------------------------------------------------------
// 6. GRAFT (Implant) -> "graft" / "shambles"
//    - Action: Inject value ONLY if the target is Null (or missing).
//    - Logic: Use _extract to check, then _cultivate to inject.
// -----------------------------------------------------------------------------

// src/core.rs

// -----------------------------------------------------------------------------
// 6. GRAFT (Void Filler) -> "graft" / "shambles"
//    - Action: Inject value ONLY if the target is Null (or missing).
//    - Logic: Single-pass traversal. Auto-vivifies path, injects only into void.
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_graft {
    ($val:expr, = $value:expr) => {
        if $val.is_null() {
            *$val = $crate::serde_json::json!($value);
        }
    };

    ($val:expr, . ($key:expr) $($rest:tt)*) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::Object($crate::serde_json::Map::new());
        }
        if let Some(obj) = $val.as_object_mut() {
            let next = obj.entry($key).or_insert($crate::serde_json::Value::Null);
            $crate::__impl_graft!(next, $($rest)*);
        }
    };

    ($val:expr, . $key:literal $($rest:tt)*) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::Object($crate::serde_json::Map::new());
        }
        if let Some(obj) = $val.as_object_mut() {
            let next = obj.entry($key).or_insert($crate::serde_json::Value::Null);
            $crate::__impl_graft!(next, $($rest)*);
        }
    };

    ($val:expr, . $key:ident $($rest:tt)*) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::Object($crate::serde_json::Map::new());
        }
        if let Some(obj) = $val.as_object_mut() {
            let next = obj.entry(std::stringify!($key)).or_insert($crate::serde_json::Value::Null);
            $crate::__impl_graft!(next, $($rest)*);
        }
    };

    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::Array(std::vec::Vec::new());
        }
        if let Some(arr) = $val.as_array_mut() {
            let idx = $idx as usize;
            if idx >= arr.len() {
                arr.resize(idx + 1, $crate::serde_json::Value::Null);
            }
            let next = &mut arr[idx];
            $crate::__impl_graft!(next, $($rest)*);
        }
    };
}

// -- Entry Point 6
#[macro_export]
#[doc(hidden)]
macro_rules! _graft {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_graft!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 7. DEPLOY (Room Expansion) -> "scaffold" / "room"
//    - Action: Pre-allocate N-dimensional memory (ROOM).
//    - Returns: ()
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_construct_room {
    ([ $len:expr ]) => {
        $crate::serde_json::Value::Array(std::vec![$crate::serde_json::Value::Null; $len])
    };

    ([ $len:expr ], $init:expr) => {{
        let __ope_init = $crate::serde_json::json!($init);
        $crate::serde_json::Value::Array(std::vec![__ope_init; $len])
    }};

    ([ $len:expr ] [ $($next:tt)+ ] $($rest:tt)*) => {
        $crate::serde_json::Value::Array(std::vec![
            $crate::__impl_construct_room!([ $($next)+ ] $($rest)*);
            $len
        ])
    };
}

// --- Entry Point 7 ---
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_deploy {
    ($val:expr, $($args:tt)+) => {{
        *$val = $crate::__impl_construct_room!($($args)+);
    }};
}

// --- Entry Point 7 (Unchanged) ---
#[macro_export]
#[doc(hidden)]
macro_rules! _deploy {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_deploy!((&mut $val), $($args)+)
    }};
}
