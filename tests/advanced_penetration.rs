use opejson::prelude::*;
use serde_json::json;

#[test]
fn test_abyssal_nesting_fixed() {
    let mut data = json!(null);

    // --- 1. Creation of 100-level deep nesting ---
    let mut temp = &mut data;
    for i in 0..100 {
        let key = format!("level_{}", i);

        // [ADVANCED TECHNIQUE: DEREFERENCING FOR MACRO ALIGNMENT]
        // Opejson's Entry Point Logic:
        // suture! (and other macros) automatically wrap the input in (&mut $val)
        // before passing it to the internal implementation.
        // When your variable is already a &mut Value, this creates a double-reference (&mut &mut Value).
        // Using *temp cancels out the extra reference, ensuring the macro operates directly on the Value anatomy.
        //
        // [OWNERSHIP MANAGEMENT: BORROWING THE KEY]
        // Using `(&key)` prevents the macro from consuming (moving) the String.
        // This ensures the key remains available for the subsequent `get_mut` call.
        suture!(*temp, .(&key) = json!({}));

        // Navigate deeper into the surgical site
        temp = temp.get_mut(&key).unwrap();
    }

    // Finalizing the deep-tissue operation
    suture!(*temp, .goal = "reached");

    // --- 2. Verification ---
    // `biopsy!` only requires an immutable reference, which is easily handled.
    assert_eq!(biopsy!(temp, .goal), Some(&json!("reached")));
    println!("100 levels of nesting: Success");
}

#[test]
fn test_high_frequency_surgery_fixed() {
    use std::time::Instant;
    let mut data = json!({});
    let start = Instant::now();

    // --- High-frequency "Suture" Stress Test (10,000 operations) ---
    for i in 0..10000 {
        let key = format!("key_{}", i % 100);
        let sub_key = format!("sub_{}", i / 100);

        // When operating directly on the root `Value` (not a reference),
        // we can use the standard syntax. The macro handles the borrowing.
        suture!(data, .(key) .(sub_key) = i);
    }

    let duration = start.elapsed();
    // This benchmark showcases the near-zero overhead of opejson's compile-time expansion.
    println!("10,000 sutures took: {:?}", duration);
}
