# Opejson 🩺🏴‍☠️

**The Ultimate Surgical Instruments for JSON in Rust.**

> *Forged as a byproduct of the **Cognitive OS** architecture.*

`opejson` is a powerful macro library that allows you to manipulate `serde_json::Value` with surgical precision.
It brings Perl-like **Auto-vivification** to Rust's JSON handling, overcoming strict type system barriers. It offers dynamic-language-like flexibility while maintaining absolute safety and zero path-parsing overhead.

> *"I'll sever those chaotic JSON paths... and stitch them back together."*

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
opejson = "0.2.4"
serde_json = "1.0"
```

### 🏴‍☠️ Unlock "Law Mode"
If you want to use the **Ope Ope no Mi** interface (Anime-inspired aliases), enable the `law_mode` feature.

```toml
[dependencies]
opejson = { version = "0.2.4", features = ["law_mode"] }
```

---

## 🚀 Usage

`opejson` provides three distinct modes of operation:

1.  **Genesis Mode**: The Creator. Auto-vivification, merging, and pre-allocation.
2.  **Strict Mode**: The Surgeon. Safe, validated operations on existing data.
3.  **Law Mode**: The "Ope Ope no Mi" Interface. (Requires feature flag).

`use serde_json::json;` is required for all examples.

### 1. Genesis Mode (Creation & Growth)
Automatically creates objects, expands arrays, and structurally merges JSON trees.

```rust,ignore
use opejson::genesis::*;

fn main() {
    let mut data = json!({});

    // 1. Mesh (Pre-allocation)
    // Deploys a multi-dimensional array instantly.
    mesh!(data, [10][20][5], serde_json::Value::Null);

    // 2. Suture (Safe Auto-vivification)
    // Creates paths safely. Stops and preserves if it hits an existing scalar.
    suture!(data, . "users" [0] . "name" = "Luffy");
    
    // 3. Force Suture (Destructive Auto-vivification)
    // Crushes existing scalars in the path to FORCIBLY build the new structure.
    force_suture!(data, . "users" [0] . "name" . "first" = "Monkey"); 

    // 4. Graft (Anatomical Merge / Shambles)
    // Surgically merges Objects or concatenates Arrays at the target joint.
    graft!(data, . "users" [0] = json!({"role": "Captain", "bounty": 3000000000u64}));
    
    // 5. Implant (Void Filler / Default Injection)
    // Injects data ONLY if the target is Null (Void). Preserves healthy data.
    implant!(data, . "users" [0] . "status" = "Alive");
    
    // 6. Acquire (Heart Extraction)
    // Safe read, returns Result<&Value, Error>.
    let name_res = acquire!(data, . "users" [0] . "name" . "first").unwrap();
    assert_eq!(name_res.as_str().unwrap(), "Monkey");
}
```

### 2. Strict Mode (Precision Surgery)
Strictly operates on *existing* paths. Returns `Option`. Safe for data validation and probing unknown JSON.

```rust,ignore
use opejson::strict::{biopsy, incise, excise};

fn main() {
    let mut data = json!({ "id": 101, "meta": { "active": true } });

    // 1. Biopsy (Probe / Read) -> Option<&Value>
    let is_active = biopsy!(data, . "meta" . "active");

    // 2. Incise (Inject / Write) -> Option<()>
    // Only succeeds if the path ALREADY exists.
    let res = incise!(data, . "id" = 102); 
    assert!(res.is_some());
    
    let fail = incise!(data, . "unknown" = 999); 
    assert!(fail.is_none());

    // 3. Excise (Remove / Delete) -> Option<Value>
    // cleanly cuts out the target node.
    let removed = excise!(data, . "meta");
}
```

#### 🎯 Dynamic Keys & Indices
You can use variables (expressions) for keys and indices by wrapping them in parentheses `( )`.

```rust,ignore
let key = "stats";
let idx = 5;

// Strict Read
biopsy!(data, . (key) [idx]); 

// Genesis Write
suture!(data, . (key) [idx] = "Updated");
```

---

### 3. Law Mode (The Ope Ope no Mi) 🏴‍☠️
For those who possess the ultimate power.
This mode maps surgical operations to the abilities of the "Ope Ope no Mi".

*(Requires `features = ["law_mode"]`)*

| Roles                       | Names | Law Mode | Action Description                               |
|-----------------------------|------------------|------------------|--------------------------------------------------|
| Strict Read (Option)        | `biopsy!`        | **`scan!`** | Safely scans the internal structure without harm |
| Strict Write (Mut)          | `incise!`        | **`radio_knife!`**| Precisely cuts an existing path to update a value|
| Strict Delete               | `excise!`        | **`amputate!`** | Cleanly severs (deletes) a specific node         |
| Genesis Write               | `suture!`        | **`takt!`** | Safely auto-vivifies structure to fill gaps      |
| Genesis Force Write         | `force_suture!`  | **`gamma_knife!`**| Destroys existing scalars to force path creation |
| Genesis Read (Result)       | `acquire!`       | **`mes!`** | Extracts the target heart (value) returning Result|
| Genesis Fill (Void Filler)  | `implant!`       | **`injection_shot!`**| Injects a value ONLY if the target is Null   |
| Genesis Merge               | `graft!`         | **`shambles!`** | Anatomically merges or concatenates structures   |
| Deploy (Room Expansion)     | `mesh!`          | **`room!`** | Instantly deploys a multi-dimensional spatial grid|

```rust,ignore
use opejson::law::*;

fn main() {
    let mut target = json!(null);

    // "ROOM." (Expand N-dimensional Space)
    room!(target, [3][3][3]);

    // "TAKT." (Auto-vivify structure)
    takt!(target, [1][1][1] = "Heart");

    // "SHAMBLES." (Merge / Append)
    let switch_obj = json!(["New Limb"]);
    shambles!(target, [0] = switch_obj);
    
    // "MES." (Extract)
    let heart = mes!(target, [1][1][1]).unwrap();
}
```

---

## 🧪 Error Types

`acquire!` (Genesis Read) returns a `Result<&Value, opejson::Error>`:

- **PathNotFound(String)**: The specified JSON path does not exist.

> **Note:** Strict Mode (`biopsy!`) returns `Option<&Value>` instead, following the idiomatic "silent probe" pattern.

---

## ⚠️ Surgical Precision Required

`opejson` is designed as a sharp scalpel, prioritizing performance and brevity over restrictive safety barriers.

* **Index Safety:** Array indices are cast raw using as usize in release mode for maximum performance. Passing negative values is caught via debug_assert! in development, but you must ensure logic correctness before production.
* **Handle with Care:** You are the surgeon. Verify your dynamic inputs before operating.

---

## 🧠 Design Philosophy: Minimal Intervention

`opejson` is a **precision instrument**, not a safety-guarded machine. It strictly follows the "Zero-Cost Abstraction" doctrine.

### 1. True Zero-Cost Abstraction (Zero Path-Parsing Overhead)
Opejson does not reduce the inherent cost of JSON allocation or mutation — it removes only the abstraction overhead. Unlike traditional JSON libraries, there is absolutely **no runtime path parsing**.

- **Physical Memory Operations:** Uses inline operations (`extend`, `append`, `match`) with no intermediate path representation, no dynamic dispatch, and no runtime tokenization.
- **Compile-Time Resolution:** Path expansion is fully resolved at compile time, expanding to equivalent, highly optimized handwritten JSON access.

### 2. Explicit Responsibility
Error handling should be visible and under the caller’s control. Macros are a sealed environment; they should not silently reinterpret or absorb structural errors. **Responsibility belongs to the surgeon (the programmer).**

### 3. "Programmer Errors" are not "Library Errors"
Misusing indices (e.g., negative wrapping via `as usize`) or logical flaws in your loops are treated as **programmer mistakes**. Rust provides the tools to prevent these; `opejson` will not add runtime bloat to compensate for them.

### 4. Genesis vs. Strict
- **Genesis Mode**: Permissive, creative, and dangerously fast (Auto-vivification).
- **Strict Mode**: Safe, non-destructive, and explicit.

Both modes expand to raw, handwritten-level JSON operations — the difference is intent and safety, not performance.

Choose your instrument according to the risk of the operation.

---

## ⚡ Comparison: The Pain vs. Opejson

**Without Opejson (Standard Serde):**
```rust,ignore
// The "Option Hell"
if let Some(users) = data.get_mut("users") {
    if let Some(user) = users.get_mut(0) {
        if let Some(obj) = user.as_object_mut() {
            obj.insert("name".to_string(), json!("Chopper"));
        }
    }
}
```

**With Opejson:**
```rust,ignore
// Surgical Precision (Auto-vivification)
suture!(data, . "users" [0] . "name" = "Chopper");
```

---

## 📊 Testing & Performance

### Comprehensive Validation
- **133 test cases** covering all macro patterns, dynamic keys, Auto-vivification logic, and structural edge cases.
- Validated up to **100 levels of deep nesting** with zero path-parsing overhead.

### Performance Benchmark
Measurements taken from `tests/performance_limit.rs` (Release mode on standard hardware):

- **256-Level Deep Penetration**: `~366µs`
- **100,000 Massive Sutures**: `~28ms`
- **Overall Throughput**: **~3,560,000 operations/sec**

> **Zero path-parsing overhead.** The macro expands directly into raw pointer access and pattern matching during compilation, making complex JSON manipulation as fast as native struct access.

---
## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
