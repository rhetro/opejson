# Opejson 🩺🏴‍☠️

**The Ultimate Surgical Instruments for JSON in Rust.**

> *Forged as a byproduct of the **Cognitive OS** architecture.*

`opejson` is a powerful macro library that allows you to manipulate `serde_json::Value` with surgical precision.
It overcomes Rust's strict type system barriers, offering dynamic-language-like flexibility while maintaining safety and performance.

> *"I'll sever those chaotic JSON paths... and stitch them back together."*

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
opejson = "0.1.0"
serde_json = "1.0"
```

### 🏴‍☠️ Unlock "Law Mode"
If you want to use the **Ope Ope no Mi** interface (Anime-inspired aliases), enable the `law_mode` feature.

```toml
[dependencies]
opejson = { version = "0.1.0", features = ["law_mode"] }
```

---

## 🚀 Usage

`opejson` provides three distinct modes of operation:

1.  **Genesis Mode**: The Creator. Auto-vivification and Pre-allocation.
2.  **Strict Mode**: The Surgeon. Safe, validated operations on existing data.
3.  **Law Mode**: The "Ope Ope no Mi" Interface. (Requires feature flag).

`use serde_json::json;` is required for all examples.

### 1. Genesis Mode (Creation & Growth)
Automatically creates objects, expands arrays, and allocates memory.

```rust
use opejson::genesis::{suture, graft, acquire, scaffold};

fn main() {
    let mut data = json!({});

    // 1. Scaffold (Pre-allocation)
    scaffold!(data, [10][20][5], serde_json::Value::Null);

    // 2. Suture (Force Write / Overwrite)
    // "God Mode". Creates paths and FORCIBLY overwrites existing values.
    suture!(data, . "users" [0] . "name" = "Luffy");
    
    // 3. Graft (Safe Write / Void Filler)
    // Injects data ONLY if the target is Null or missing (Void).
    // If a value exists, it does nothing (No-op).
    graft!(data, . "users" [0] . "name" = "Zoro"); // Luffy stays.
    graft!(data, . "users" [0] . "role" = "Captain"); // "role" is added.
    
    // 4. Acquire (Safe Read with Result)
    // Returns Result<&Value, Error>.
    let name_res = acquire!(data, . "users" [0] . "name").unwrap();
    assert_eq!(name_res.as_str().unwrap(), "Luffy");
}
```

### 2. Strict Mode (Precision Surgery)
Strictly operates on *existing* paths. Returns `Option`. Safe for data validation and probing unknown JSON.

```rust
use opejson::strict::{biopsy, incise, amputate};

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

    // 3. Amputate (Excise / Delete) -> Option<Value>
    let removed = amputate!(data, . "meta");
}
```

#### 🎯 Dynamic Keys & Indices
You can use variables (expressions) for keys and indices by wrapping them in parentheses `( )`.

```rust
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

| Capability | Function | Description | Core Concept |
|:---|:---|:---|:---|
| **SCAN** | `scan!` | Read value (Option) | `biopsy` |
| **RADIO KNIFE** | `radio_knife!` | Write strictly | `incise` |
| **AMPUTATE** | `amputate!` | Delete value | `amputate` |
| **TAKT** | `takt!` | Force write (Overwrite) | `suture` |
| **MES** | `mes!` | Extract result | `acquire` |
| **SHAMBLES** | `shambles!` | Safe write (Void fill) | `graft` |
| **ROOM** | `room!` | Pre-allocate N-dim memory | `scaffold` |

```rust
use opejson::law::*;

fn main() {
    let mut target = json!(null);

    // "ROOM." (Expand N-dimensional Space)
    room!(target, [3][3][3]);

    // "TAKT." (Lift/Create object)
    // Forcefully creates the object.
    takt!(target, [1][1][1] = "Heart");

    // "SHAMBLES." (Teleport/Inject Object)
    // Injects only into void/null spaces (Safe).
    let switch_obj = json!({"status": "Swapped"});
    shambles!(target, [0][0][0] = switch_obj);
    
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

* **Indices are Cast Raw:** Array indices are cast using `as usize`. Passing negative values will wrap to huge numbers, potentially causing massive allocations (OOM) in Genesis Mode.
* **Handle with Care:** You are the surgeon. Verify your dynamic inputs before operating.

---

## 🧠 Design Philosophy: Minimal Intervention

`opejson` is a **precision instrument**, not a safety-guarded machine. It follows a "Zero-Overhead" doctrine.

### 1. Performance First
Macros expand at compile time. Adding complex, hidden validation inside expansion would increase branching and reduce throughput. `opejson` prioritizes **raw execution speed**.

### 2. Explicit Responsibility
Error handling should be visible and under the caller’s control. Macros are a sealed environment; they should not silently reinterpret or absorb structural errors. **Responsibility belongs to the surgeon (the programmer).**

### 3. "Programmer Errors" are not "Library Errors"
Misusing indices (e.g., negative wrapping via `as usize`) or logical flaws in your loops are treated as **programmer mistakes**. Rust provides the tools to prevent these; `opejson` will not add runtime bloat to compensate for them.

### 4. Genesis vs. Strict
- **Genesis Mode**: Permissive, creative, and dangerously fast.
- **Strict Mode**: Safe, non-destructive, and explicit.

Choose your instrument according to the risk of the operation.

---

## ⚡ Comparison: The Pain vs. Opejson

**Without Opejson (Standard Serde):**
```rust
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
```rust
// Surgical Precision
suture!(data, . "users" [0] . "name" = "Chopper");
```

---

## 📊 Testing & Performance

### Comprehensive Validation
- **123 test cases** covering all macro patterns, dynamic keys, and structural edge cases.
- Validated up to **100 levels of deep nesting** with zero runtime overhead.

### Performance Benchmark
Measurements taken from `tests/performance_limit.rs` (Release mode):

- **256-Level Deep Penetration**: `366.767µs`
- **100,000 Massive Sutures**: `29.638ms`
- **Overall Throughput**: **3,341,248 operations/sec**

> **Zero runtime overhead for path parsing.** > The macro expands directly into raw pointer access during compilation, making complex JSON manipulation as fast as native struct access.

---

## License

MIT
