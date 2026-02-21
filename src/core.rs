// src/core.rs

// ====================================================
// === Opejson Core: Surgical Instruments for JSON ===
// ====================================================

// =============================
// === Opejson: Strict Mode ===
// =============================

// -----------------------------------------------------------------------------
// 1. PROBE (Strict Read) -> "biopsy" / "scan"
//    - Action: Search for a value.
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
        *$val = $crate::serde_json::Value::from($value);
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
// 3. EXCISE (Delete) -> "excise" / "amputate"
//    - Action: Remove a value from object or array.
//    - Returns: Option<Value>
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

// =================================================================
// === Opejson: Genesis Mode (Structure Creation & Manipulation) ===
// =================================================================

// -----------------------------------------------------------------------------
// 0. THE MATTER FORGE (Static Allocation Engine)
//    - Action: Pre-calculate capacity and forge structures without runtime resize.
// -----------------------------------------------------------------------------

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_forge {
    // Case A: Empty Array
    ([ ]) => {
        $crate::serde_json::Value::Array(std::vec::Vec::new())
    };

    // Case B: Content Array (Static Capacity Calculation)
    ([ $($item:tt)+ ]) => {{
        {
            // Count tokens for O(1) allocation
            const __forge_cap: usize = 0usize $(+ $crate::__impl_forge_count!($item))*;
            let mut __vec = std::vec::Vec::with_capacity(__forge_cap);
            $(
                __vec.push($crate::__impl_forge!($item));
            )*
            $crate::serde_json::Value::Array(__vec)
        }
    }};

    // Case C: Empty Object
    ({ }) => {
        $crate::serde_json::Value::Object($crate::serde_json::Map::new())
    };

    // Case D: Content Object (Static Capacity Calculation)
    ({ $($key:tt : $val:tt),+ $(,)? }) => {{
        {
            const __forge_cap: usize = 0usize $(+ $crate::__impl_forge_count!($key))*;
            let mut __forge_map = $crate::serde_json::Map::with_capacity(__forge_cap);
            $(
                __forge_map.insert(
                    std::string::String::from($key),
                    $crate::__impl_forge!($val)
                );
            )*
            $crate::serde_json::Value::Object(__forge_map)
        }
    }};

    // Case E: Primitives
    ($val:expr) => {
        $crate::serde_json::Value::from($val)
    };
}

// Unary Counter for Forge
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_forge_count {
    ($_:tt) => {
        1usize
    };
}

// -----------------------------------------------------------------------------
// 1. SUTURE (Genesis Creation) -> "suture" / "takt"
//    - Action: Safe Auto-vivification.
//    - Logic: Match -> Conform or Reincarnate (Forge & Retry).
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_cultivate {
    // Phase 1: Terminal Assignment (Forge Integration)
    ($val:expr, = [ $($item:tt)* ]) => {
        *$val = $crate::__impl_forge!([ $($item)* ]);
    };
    ($val:expr, = { $($item:tt)* }) => {
        *$val = $crate::__impl_forge!({ $($item)* });
    };
    ($val:expr, = $value:expr) => {
        *$val = $crate::serde_json::Value::from($value);
    };

    // Phase 2: Object Traversal ( .key )
    ($val:expr, . $key:tt $($rest:tt)*) => {{
        // 1. Safe Transmutation
        if $val.is_null() {
            *$val = $crate::__impl_forge!({ });
        }

        // 2. Drill & Recurse
        if let $crate::serde_json::Value::Object(map) = $val {
            let next = map.entry($crate::__opejson_key!($key))
                          .or_insert($crate::serde_json::Value::Null);
            $crate::__impl_cultivate!(next, $($rest)*);
        }
    }};

    // Phase 3: Array Traversal ( [index] )
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {{
        // 1. Safe Transmutation
        if $val.is_null() {
            *$val = $crate::__impl_forge!([ ]);
        }

        // 2. Drill & Recurse
        if let $crate::serde_json::Value::Array(arr) = $val {
            #[allow(unused_comparisons)]
            {
                debug_assert!($idx >= 0, "Opejson: Array index must be zero or positive");
            }
            let idx = $idx as usize;
            if idx >= arr.len() {
                arr.resize(idx + 1, $crate::serde_json::Value::Null);
            }
            $crate::__impl_cultivate!(&mut arr[idx], $($rest)*);
        }
    }};
}

// --- Entry Point (Safe) ---
#[macro_export]
#[doc(hidden)]
macro_rules! _cultivate {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_cultivate!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 2. FORCE SUTURE (Forced Override) -> "force_suture" / "gamma_knife"
//    - Action: Destructive Auto-vivification.
//    - Logic: Sequential Check -> Enforce (if NOT expected type) -> Drill
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_force_cultivate {
    // Phase 1: Terminal Assignment (Same as safe)
    ($val:expr, = [ $($item:tt)* ]) => {
        *$val = $crate::__impl_forge!([ $($item)* ]);
    };
    ($val:expr, = { $($item:tt)* }) => {
        *$val = $crate::__impl_forge!({ $($item)* });
    };
    ($val:expr, = $value:expr) => {
        *$val = $crate::serde_json::Value::from($value);
    };

    // Phase 2: Object Traversal ( .key )
    ($val:expr, . $key:tt $($rest:tt)*) => {{
        // 1. Force Transmutation
        if !$val.is_object() {
            *$val = $crate::__impl_forge!({ });
        }

        // 2. Drill & Recurse
        if let $crate::serde_json::Value::Object(map) = $val {
            let next = map.entry($crate::__opejson_key!($key))
                          .or_insert($crate::serde_json::Value::Null);
            $crate::__impl_force_cultivate!(next, $($rest)*);
        }
    }};

    // Phase 3: Array Traversal ( [index] )
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {{
        // 1. Force Transmutation
        if !$val.is_array() {
            *$val = $crate::__impl_forge!([ ]);
        }

        // 2. Drill & Recurse
        if let $crate::serde_json::Value::Array(arr) = $val {
            #[allow(unused_comparisons)]
            {
                debug_assert!($idx >= 0, "Opejson: Array index must be zero or positive");
            }
            let idx = $idx as usize;
            if idx >= arr.len() {
                arr.resize(idx + 1, $crate::serde_json::Value::Null);
            }
            $crate::__impl_force_cultivate!(&mut arr[idx], $($rest)*);
        }
    }};
}

// --- Entry Point (Force) ---
#[macro_export]
#[doc(hidden)]
macro_rules! _force_cultivate {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_force_cultivate!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 3. EXTRACT (Genesis Read) -> "acquire" / "mes"
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
// 4. IMPLANT (Void Filler) -> "implant" / "injection_shot"
//    - Action: Inject value ONLY if the target is Null (or missing).
//    - Update: Now uses Forge logic to handle structure creation.
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_implant {
    // Phase 1: Terminal Assignment (Void Only)
    ($val:expr, = $value:expr) => {
        if $val.is_null() {
            *$val = $crate::serde_json::Value::from($value);
        }
    };

    // Phase 2: Object Traversal
    ($val:expr, . $key:tt $($rest:tt)*) => {
        // Enforce Object if Null (using Forge)
        if $val.is_null() {
            *$val = $crate::__impl_forge!({ });
        }
        // Drill only if structure matches
        if let Some(map) = $val.as_object_mut() {
            let next = map.entry($crate::__opejson_key!($key))
                          .or_insert($crate::serde_json::Value::Null);
            $crate::__impl_implant!(next, $($rest)*);
        }
    };

    // Phase 3: Array Traversal
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {
        // Enforce Array if Null (using Forge)
        if $val.is_null() {
            *$val = $crate::__impl_forge!([ ]);
        }
        // Drill only if structure matches
        if let Some(arr) = $val.as_array_mut() {
            #[allow(unused_comparisons)]
            {
                debug_assert!($idx >= 0, "Opejson: Array index must be zero or positive");
            }
            let idx = $idx as usize;
            if idx >= arr.len() {
                arr.resize(idx + 1, $crate::serde_json::Value::Null);
            }
            $crate::__impl_implant!(&mut arr[idx], $($rest)*);
        }
    };
}

// --- Entry Point ---
#[macro_export]
#[doc(hidden)]
macro_rules! _implant {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_implant!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 5. GRAFT (Anatomical Transplantation) -> "graft" / "shambles"
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_graft {
    // Phase 1: Terminal Assignment (Anatomical Graft at the cut line)
    ($val:expr, = $value:expr) => {{
        match (&mut *$val, $crate::serde_json::Value::from($value)) {
            // 1. Object + Object
            ($crate::serde_json::Value::Object(h_map), $crate::serde_json::Value::Object(s_map)) => {
                h_map.extend(s_map);
            }
            // 2. Array + Array
            ($crate::serde_json::Value::Array(h_arr), $crate::serde_json::Value::Array(mut s_arr)) => {
                h_arr.append(&mut s_arr);
            }
            // 3. Otherwise
            (h_val, s_val) => {
                *h_val = s_val;
            }
        }
    }};

    // Phase 2: Object Traversal
    ($val:expr, . $key:tt $($rest:tt)*) => {{
        if $val.is_null() {
            *$val = $crate::__impl_forge!({ });
        }
        if let $crate::serde_json::Value::Object(map) = $val {
            let next = map.entry($crate::__opejson_key!($key))
                          .or_insert($crate::serde_json::Value::Null);
            $crate::__impl_graft!(next, $($rest)*);
        }
    }};

    // Phase 3: Array Traversal
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {{
        if $val.is_null() {
            *$val = $crate::__impl_forge!([ ]);
        }
        if let $crate::serde_json::Value::Array(arr) = $val {
            #[allow(unused_comparisons)]
            {
                debug_assert!($idx >= 0, "Opejson: Array index must be zero or positive");
            }
            let idx = $idx as usize;
            if idx >= arr.len() {
                arr.resize(idx + 1, $crate::serde_json::Value::Null);
            }
            $crate::__impl_graft!(&mut arr[idx], $($rest)*);
        }
    }};
}

// --- Entry Point ---
#[macro_export]
#[doc(hidden)]
macro_rules! _graft {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_graft!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 6. MESH (Room Expansion) -> "mesh" / "room"
//   - Action: Pre-allocates and deploys multidimensional arrays.
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_construct_room {
    // 1D Array (Initialized with specific value)
    ([ $len:expr ] , $init:expr) => {{
        let len = $len as usize;
        let init_val = $crate::serde_json::Value::from($init);
        let mut v = std::vec::Vec::with_capacity(len);
        for _ in 0..len {
            v.push(init_val.clone());
        }
        $crate::serde_json::Value::Array(v)
    }};

    // 1D Array (Default Null)
    ([ $len:expr ]) => {
        $crate::__impl_construct_room!([ $len ] , $crate::serde_json::Value::Null)
    };

    // Multi-Dimensional (Recursive)
    ([ $len:expr ] [ $($next:tt)+ ] $($rest:tt)*) => {{
        let len = $len as usize;
        let mut v = std::vec::Vec::with_capacity(len);
        for _ in 0..len {
            v.push($crate::__impl_construct_room!([ $($next)+ ] $($rest)*));
        }
        $crate::serde_json::Value::Array(v)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_deploy {
    ($val:expr, $($args:tt)+) => {{
        *$val = $crate::__impl_construct_room!($($args)+);
    }};
}

// --- Entry Point ---
#[macro_export]
#[doc(hidden)]
macro_rules! _deploy {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_json::Value = &mut $val;
        $crate::__impl_deploy!((&mut $val), $($args)+)
    }};
}

// -----------------------------------------------------------------------------
// Helper for Key normalization
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __opejson_key {
    (($e:expr)) => {
        $e
    }; // Expression
    ($l:literal) => {
        $l
    }; // Literal
    ($i:ident) => {
        std::stringify!($i)
    }; // Identifier
}
