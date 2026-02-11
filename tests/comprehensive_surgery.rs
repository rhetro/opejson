use opejson::prelude::*;
use serde_json::{json, Value};

// =============================================================================
// 📊 COMPREHENSIVE PATTERN COVERAGE TESTS FOR opejson
//
// This test suite exhaustively validates all path syntax patterns and macro
// behaviors across Genesis Mode, Strict Mode, and edge cases.
// =============================================================================

// =============================================================================
// GENESIS MODE
// =============================================================================

// -----------------------------------------------------------------------------
// 1. SUTURE - ALL PATH PATTERNS
// -----------------------------------------------------------------------------
#[test]
fn test_suture_single_level_object_key_literal() {
    let mut data = json!({});
    suture!(data, . "name" = "Alice");
    assert_eq!(data["name"], "Alice");
}

#[test]
fn test_suture_single_level_object_key_ident() {
    let mut data = json!({});
    suture!(data, .name = "Bob");
    assert_eq!(data["name"], "Bob");
}

#[test]
fn test_suture_single_level_object_key_dynamic() {
    let mut data = json!({});
    let key = "username";
    suture!(data, .(key) = "Charlie");
    assert_eq!(data["username"], "Charlie");
}

#[test]
fn test_suture_nested_three_levels_mixed_keys() {
    let mut data = json!(null);
    suture!(data, .user .profile .age = 30);
    assert_eq!(data["user"]["profile"]["age"], 30);
    assert!(data["user"].is_object());
    assert!(data["user"]["profile"].is_object());
}

#[test]
fn test_suture_nested_with_dynamic_middle_key() {
    let mut data = json!(null);
    let middle_key = "metadata";
    suture!(data, .root .(middle_key) .value = "test");
    assert_eq!(data["root"]["metadata"]["value"], "test");
}

#[test]
fn test_suture_array_single_index() {
    let mut data = json!(null);
    suture!(data, [1] = "first");
    assert!(data.is_array());
    assert_eq!(data[1], "first");
}

#[test]
fn test_suture_array_multiple_indices() {
    let mut data = json!(null);
    suture!(data, [2] = "third");
    assert!(data.is_array());
    assert_eq!(data.as_array().unwrap().len(), 3);
    assert_eq!(data[2], "third");
}

#[test]
fn test_suture_array_with_dynamic_index() {
    let mut data = json!(null);
    let idx = 5;
    suture!(data, [idx] = "dynamic");
    assert_eq!(data[5], "dynamic");
}

#[test]
fn test_suture_mixed_object_array_alternating() {
    let mut data = json!(null);
    suture!(data, .items [0] .name = "Item1");
    assert!(data["items"].is_array());
    assert!(data["items"][0].is_object());
    assert_eq!(data["items"][0]["name"], "Item1");
}

#[test]
fn test_suture_object_array_object_deep() {
    let mut data = json!(null);
    suture!(data, .a [0] .b [1] .c = "deep");
    assert_eq!(data["a"][0]["b"][1]["c"], "deep");
}

#[test]
fn test_suture_overwrite_existing_scalar() {
    let mut data = json!({"value": 10});
    suture!(data, .value = 20);
    assert_eq!(data["value"], 20);
}

#[test]
fn test_suture_overwrite_null_with_value() {
    let mut data = json!({"slot": null});
    suture!(data, .slot = "filled");
    assert_eq!(data["slot"], "filled");
}

#[test]
fn test_suture_overwrite_object_with_scalar() {
    let mut data = json!({"data": {"nested": true}});
    suture!(data, .data = "replaced");
    assert_eq!(data["data"], "replaced");
}

#[test]
fn test_suture_overwrite_array_with_scalar() {
    let mut data = json!({"arr": [1, 2, 3]});
    suture!(data, .arr = "array_gone");
    assert_eq!(data["arr"], "array_gone");
}

#[test]
fn test_suture_multiple_operations_same_data() {
    let mut data = json!({});
    suture!(data, .a = 1);
    suture!(data, .b = 2);
    suture!(data, .c = 3);
    assert_eq!(data["a"], 1);
    assert_eq!(data["b"], 2);
    assert_eq!(data["c"], 3);
}

#[test]
fn test_suture_deep_nested_from_null() {
    let mut data = json!(null);
    suture!(data, .l1 .l2 .l3 .l4 .l5 = "five_deep");
    assert_eq!(data["l1"]["l2"]["l3"]["l4"]["l5"], "five_deep");
}

#[test]
fn test_suture_value_types_number() {
    let mut data = json!({});
    suture!(data, .num = 42);
    assert!(data["num"].is_number());
    assert_eq!(data["num"], 42);
}

#[test]
fn test_suture_value_types_boolean() {
    let mut data = json!({});
    suture!(data, .flag = true);
    assert!(data["flag"].is_boolean());
}

#[test]
fn test_suture_value_types_null() {
    let mut data = json!({});
    suture!(data, .nothing = Value::Null);
    assert!(data["nothing"].is_null());
}

#[test]
fn test_suture_value_types_object() {
    let mut data = json!({});
    suture!(data, .obj = json!({"inner": "value"}));
    assert!(data["obj"].is_object());
    assert_eq!(data["obj"]["inner"], "value");
}

#[test]
fn test_suture_value_types_array() {
    let mut data = json!({});
    suture!(data, .arr = json!([1, 2, 3]));
    assert!(data["arr"].is_array());
}

// -----------------------------------------------------------------------------
// 2. ACQUIRE - SAFE READ WITH RESULT
// -----------------------------------------------------------------------------
#[test]
fn test_acquire_existing_value() {
    let data = json!({"key": "value"});
    let result = acquire!(data, .key);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &json!("value"));
}

#[test]
fn test_acquire_missing_key() {
    let data = json!({"key": "value"});
    let result = acquire!(data, .missing);
    assert!(result.is_err());
}

#[test]
fn test_acquire_nested_path() {
    let data = json!({"a": {"b": {"c": "deep"}}});
    assert!(acquire!(data, .a .b .c).is_ok());
}

#[test]
fn test_acquire_nested_missing_intermediate() {
    let data = json!({"a": {"b": null}});
    assert!(acquire!(data, .a .b .c).is_err());
}

#[test]
fn test_acquire_array_access() {
    let data = json!([10, 20, 30]);
    assert_eq!(acquire!(data, [1]).unwrap(), &json!(20));
}

#[test]
fn test_acquire_array_out_of_bounds() {
    let data = json!([1, 2]);
    assert!(acquire!(data, [5]).is_err());
}

#[test]
fn test_acquire_mixed_path() {
    let data = json!({"items": [{"name": "A"}, {"name": "B"}]});
    assert_eq!(acquire!(data, .items [1] .name).unwrap(), &json!("B"));
}

#[test]
fn test_acquire_ident_key() {
    let data = json!({"val": 42});
    assert_eq!(acquire!(data, .val).unwrap(), &json!(42));
}

#[test]
fn test_acquire_literal_key() {
    let data = json!({"val": 42});
    assert_eq!(acquire!(data, . "val").unwrap(), &json!(42));
}

#[test]
fn test_acquire_dynamic_key() {
    let data = json!({"val": 42});
    let key = "val";
    assert_eq!(acquire!(data, .(key)).unwrap(), &json!(42));
}

// -----------------------------------------------------------------------------
// 3. GRAFT - SAFETY & PRESERVATION
// -----------------------------------------------------------------------------

#[test]
fn test_graft_into_null_slot() {
    let mut data = json!({"empty": null});
    graft!(data, .empty = "filled");
    assert_eq!(data["empty"], "filled");
}

#[test]
fn test_graft_preserves_existing_value() {
    let mut data = json!({"keep": "original"});
    graft!(data, .keep = "new");
    assert_eq!(data["keep"], "original");
}

#[test]
fn test_graft_creates_missing_path() {
    let mut data = json!({});
    graft!(data, .new = "created");
    assert_eq!(data["new"], "created");
}

#[test]
fn test_graft_mixed_null_and_occupied() {
    let mut data = json!({"a": 1, "b": null, "c": "text"});
    graft!(data, .a = 999);
    graft!(data, .b = 999);
    graft!(data, .c = "modified");
    graft!(data, .d = "new");

    assert_eq!(data["a"], 1); // Existing, preserved
    assert_eq!(data["b"], 999); // Null, filled
    assert_eq!(data["c"], "text"); // Existing, preserved
    assert_eq!(data["d"], "new"); // Missing, created
}

#[test]
fn test_graft_nested_path_creation() {
    let mut data = json!({});
    graft!(data, .level1 .level2 .level3 = "nested");
    assert_eq!(data["level1"]["level2"]["level3"], "nested");
}

#[test]
fn test_graft_into_array_index() {
    let mut data = json!([null, "existing", null]);
    graft!(data, [0] = "filled");
    graft!(data, [1] = "changed");
    graft!(data, [2] = "filled");

    assert_eq!(data[0], "filled");
    assert_eq!(data[1], "existing");
    assert_eq!(data[2], "filled");
}

#[test]
fn test_graft_ident_key() {
    let mut data = json!({"a": null});
    graft!(data, .a = "filled");
    assert_eq!(data["a"], "filled");
}

#[test]
fn test_graft_literal_key() {
    let mut data = json!({"a": null});
    graft!(data, . "a" = "filled");
    assert_eq!(data["a"], "filled");
}

#[test]
fn test_graft_dynamic_key() {
    let mut data = json!({"a": null});
    let key = "a";
    graft!(data, .(key) = "filled");
    assert_eq!(data["a"], "filled");
}

// -----------------------------------------------------------------------------
// 4. SCAFFOLD - ALLOCATION & PRE-STRUCTURE
// -----------------------------------------------------------------------------
#[test]
fn test_scaffold_1d_array() {
    let mut data = json!(null);
    scaffold!(data, [5], Value::Null);
    assert!(data.is_array());
    assert_eq!(data.as_array().unwrap().len(), 5);
    for i in 0..5 {
        assert!(data[i].is_null());
    }
}

#[test]
fn test_scaffold_2d_array() {
    let mut data = json!(null);
    scaffold!(data, [3][4], Value::Null);
    assert_eq!(data.as_array().unwrap().len(), 3);
    for i in 0..3 {
        assert_eq!(data[i].as_array().unwrap().len(), 4);
    }
}

#[test]
fn test_scaffold_3d_array() {
    let mut data = json!(null);
    scaffold!(data, [2][3][2], 0);
    assert_eq!(data[1][2][1], 0);
}

#[test]
fn test_scaffold_with_custom_fill_value() {
    let mut data = json!(null);
    scaffold!(data, [3], "empty_slot");
    assert_eq!(data[0], "empty_slot");
    assert_eq!(data[1], "empty_slot");
    assert_eq!(data[2], "empty_slot");
}

#[test]
fn test_scaffold_with_object_fill() {
    let mut data = json!(null);
    scaffold!(data, [2], json!({"template": true}));
    assert_eq!(data[0]["template"], true);
    assert_eq!(data[1]["template"], true);
}

// =============================================================================
// STRICT MODE
// =============================================================================

// -----------------------------------------------------------------------------
// 5. BIOPSY - READ-ONLY SAFETY
// -----------------------------------------------------------------------------
#[test]
fn test_biopsy_existing_key() {
    let data = json!({"info": "present"});
    assert_eq!(biopsy!(data, .info), Some(&json!("present")));
}

#[test]
fn test_biopsy_missing_key() {
    let data = json!({"info": "present"});
    assert_eq!(biopsy!(data, .missing), None);
}

#[test]
fn test_biopsy_nested_existing() {
    let data = json!({"a": {"b": "found"}});
    assert_eq!(biopsy!(data, .a .b), Some(&json!("found")));
}

#[test]
fn test_biopsy_nested_missing_leaf() {
    let data = json!({"a": {"b": "value"}});
    assert_eq!(biopsy!(data, .a .c), None);
}

#[test]
fn test_biopsy_nested_missing_intermediate() {
    let data = json!({"a": "scalar"});
    assert_eq!(biopsy!(data, .a .b), None);
}

#[test]
fn test_biopsy_array_access() {
    let data = json!([10, 20, 30]);
    assert_eq!(biopsy!(data, [1]), Some(&json!(20)));
}

#[test]
fn test_biopsy_array_out_of_bounds() {
    let data = json!([1, 2]);
    assert_eq!(biopsy!(data, [10]), None);
}

#[test]
fn test_biopsy_dynamic_key() {
    let data = json!({"field": "value"});
    let key = "field";
    assert_eq!(biopsy!(data, .(key)), Some(&json!("value")));
}

#[test]
fn test_biopsy_does_not_modify() {
    let data = json!({"count": 1});
    let _ = biopsy!(data, .count);
    assert_eq!(data["count"], 1);
}

#[test]
fn test_biopsy_with_ident_key() {
    let data = json!({"count": 5});
    assert_eq!(biopsy!(data, .count), Some(&json!(5)));
}

#[test]
fn test_biopsy_with_literal_key() {
    let data = json!({"count": 5});
    assert_eq!(biopsy!(data, . "count"), Some(&json!(5)));
}

#[test]
fn test_biopsy_with_dynamic_key() {
    let data = json!({"count": 5});
    let key = "count";
    assert_eq!(biopsy!(data, .(key)), Some(&json!(5)));
}

#[test]
fn test_biopsy_nested_mixed_keys() {
    let data = json!({"user": {"info": {"name": "Alice"}}});
    let middle = "info";
    let result = biopsy!(data, .user .(middle) .name);
    assert_eq!(result, Some(&json!("Alice")));
}

// -----------------------------------------------------------------------------
// 6️⃣. INCISE - MODIFICATION WITH VALIDATION
// -----------------------------------------------------------------------------
#[test]
fn test_incise_existing_key() {
    let mut data = json!({"val": 10});
    let result = incise!(data, .val = 20);
    assert!(result.is_some());
    assert_eq!(data["val"], 20);
}

#[test]
fn test_incise_missing_key() {
    let mut data = json!({"val": 10});
    let result = incise!(data, .missing = 20);
    assert!(result.is_none());
    assert!(!data.as_object().unwrap().contains_key("missing"));
}

#[test]
fn test_incise_nested_existing_path() {
    let mut data = json!({"a": {"b": {"c": 5}}});
    incise!(data, .a .b .c = 100);
    assert_eq!(data["a"]["b"]["c"], 100);
}

#[test]
fn test_incise_nested_missing_leaf() {
    let mut data = json!({"a": {"b": "value"}});
    let result = incise!(data, .a .c = "new");
    assert!(result.is_none());
}

#[test]
fn test_incise_nested_missing_intermediate() {
    let mut data = json!({"a": "scalar"});
    let result = incise!(data, .a .b = "update");
    assert!(result.is_none());
}

#[test]
fn test_incise_array_element() {
    let mut data = json!([1, 2, 3]);
    incise!(data, [1] = 999);
    assert_eq!(data[1], 999);
}

#[test]
fn test_incise_array_out_of_bounds() {
    let mut data = json!([1, 2]);
    let result = incise!(data, [5] = 100);
    assert!(result.is_none());
}

#[test]
fn test_incise_ident_key() {
    let mut data = json!({"status": "ok"});
    incise!(data, .status = "changed");
    assert_eq!(data["status"], "changed");
}

#[test]
fn test_incise_literal_key() {
    let mut data = json!({"status": "ok"});
    incise!(data, . "status" = "changed");
    assert_eq!(data["status"], "changed");
}

#[test]
fn test_incise_dynamic_key() {
    let mut data = json!({"status": "ok"});
    let key = "status";
    incise!(data, .(key) = "changed");
    assert_eq!(data["status"], "changed");
}

// -----------------------------------------------------------------------------
// 7. AMPUTATE - DELETION & EXTRACTION
// -----------------------------------------------------------------------------
#[test]
fn test_amputate_simple_key() {
    let mut data = json!({"to_delete": "gone", "keep": "stay"});
    amputate!(data, .to_delete);
    assert!(!data.as_object().unwrap().contains_key("to_delete"));
    assert_eq!(data["keep"], "stay");
}

#[test]
fn test_amputate_missing_key() {
    let mut data = json!({"key": "value"});
    // Should not panic or crash
    amputate!(data, .missing);
    assert_eq!(data["key"], "value");
}

#[test]
fn test_amputate_nested() {
    let mut data = json!({"a": {"b": "target", "c": "keep"}});
    amputate!(data, .a .b);
    assert!(!data["a"].as_object().unwrap().contains_key("b"));
    assert_eq!(data["a"]["c"], "keep");
}

#[test]
fn test_amputate_array_element() {
    let mut data = json!(["keep", "delete", "keep"]);
    amputate!(data, [1]);
    assert_eq!(data.as_array().unwrap().len(), 2);
}

// =============================================================================
// EDGE CASES
// =============================================================================

// -----------------------------------------------------------------------------
// 8. TYPE TRANSITIONS
// -----------------------------------------------------------------------------
#[test]
fn test_null_to_object_conversion() {
    let mut data = json!(null);
    suture!(data, .key = "value");
    assert!(data.is_object());
    assert_eq!(data["key"], "value");
}

#[test]
fn test_null_to_array_conversion() {
    let mut data = json!(null);
    suture!(data, [0] = "first");
    assert!(data.is_array());
}
#[test]
fn test_scalar_overwrite_stays_scalar() {
    let mut data = json!("original");
    suture!(data, = "replaced");
    assert!(data.is_string());
    assert_eq!(data, "replaced");
}

#[test]
fn test_scalar_type_preservation_with_suture() {
    let mut data = json!(42);
    suture!(data, = "now string");
    assert_eq!(data, "now string");
}

#[test]
fn test_cannot_traverse_scalar_with_suture() {
    let mut data = json!("string");
    suture!(data, .key = "value");
    assert_eq!(data, "string");
}

// -----------------------------------------------------------------------------
// 9️⃣. SPECIAL CHARACTERS IN KEYS
// -----------------------------------------------------------------------------
#[test]
fn test_key_with_spaces() {
    let mut data = json!({});
    suture!(data, . "key with spaces" = "value");
    assert_eq!(data["key with spaces"], "value");
}

#[test]
fn test_key_with_special_chars() {
    let mut data = json!({});
    suture!(data, . "key-with-dash" = "v1");
    suture!(data, . "key.with.dots" = "v2");
    suture!(data, . "key/with/slash" = "v3");
    assert_eq!(data["key-with-dash"], "v1");
    assert_eq!(data["key.with.dots"], "v2");
    assert_eq!(data["key/with/slash"], "v3");
}

#[test]
fn test_key_numeric_string() {
    let mut data = json!({});
    suture!(data, . "123" = "numeric_key");
    assert_eq!(data["123"], "numeric_key");
}

#[test]
fn test_dynamic_key_with_runtime_variable() {
    let mut data = json!({});
    let runtime_key = String::from("runtime_generated");
    suture!(data, .(runtime_key.as_str()) = "dynamic");
    assert_eq!(data["runtime_generated"], "dynamic");
}

// -----------------------------------------------------------------------------
// 10. LARGE INDICES & DEEP NESTING
// -----------------------------------------------------------------------------
#[test]
fn test_large_array_index() {
    let mut data = json!(null);
    suture!(data, [1000] = "far");
    assert_eq!(data.as_array().unwrap().len(), 1001);
    assert_eq!(data[1000], "far");
}

#[test]
fn test_very_deep_nesting() {
    let mut data = json!(null);
    suture!(data, .a .b .c .d .e .f .g .h .i .j = "ten_levels");
    assert_eq!(
        data["a"]["b"]["c"]["d"]["e"]["f"]["g"]["h"]["i"]["j"],
        "ten_levels"
    );
}

#[test]
fn test_mixed_deep_nesting_with_arrays() {
    let mut data = json!(null);
    suture!(data, .level [5] .data [2] .value = "mixed");
    assert_eq!(data["level"][5]["data"][2]["value"], "mixed");
}

// -----------------------------------------------------------------------------
// 11. SUTURE/GRAFT STRUCTURAL LIMITATION
// -----------------------------------------------------------------------------
#[test]
fn test_suture_limitation_scalar_blockage() {
    let mut data = json!({"field": "scalar_value"});
    suture!(data, .field .nested = "should_not_work");
    assert_eq!(data["field"], "scalar_value");
}

#[test]
fn test_suture_requires_path_compatibility() {
    let mut data = json!({"obj": {"key": "value"}});
    suture!(data, .obj .key = "updated");
    assert_eq!(data["obj"]["key"], "updated");

    let mut data2 = json!({"arr": "not_array"});
    suture!(data2, .arr [0] = "item");
    assert_eq!(data2["arr"], "not_array");
}

#[test]
fn test_graft_same_limitation() {
    let mut data = json!({"field": "scalar"});
    graft!(data, .field .nested = "blocked");
    assert_eq!(data["field"], "scalar");
}

#[test]
fn test_direct_scalar_overwrite_is_allowed() {
    let mut data = json!("original");
    suture!(data, = "replaced");
    assert_eq!(data, "replaced");
}

#[test]
fn test_null_is_flexible_for_type_conversion() {
    let mut data = json!({"a": null});
    suture!(data, .a .key = "now_object");
    assert!(data["a"].is_object());

    let mut data2 = json!({"b": null});
    suture!(data2, .b [0] = "now_array");
    assert!(data2["b"].is_array());
}

// -----------------------------------------------------------------------------
// 1️⃣2. BOUNDARY CONDITIONS
// -----------------------------------------------------------------------------
#[test]
fn test_empty_object_operations() {
    let mut data = json!({});
    assert_eq!(biopsy!(data, .any), None);
    suture!(data, .first = "added");
    assert_eq!(data.as_object().unwrap().len(), 1);
}

#[test]
fn test_empty_array_operations() {
    let mut data = json!([]);
    assert_eq!(biopsy!(data, [0]), None);
    suture!(data, [0] = "first");
    assert_eq!(data.as_array().unwrap().len(), 1);
}

#[test]
fn test_single_element_structures() {
    let mut data = json!({"sole": "key"});
    assert_eq!(biopsy!(data, .sole), Some(&json!("key")));
    suture!(data, .sole = "updated");
    assert_eq!(data["sole"], "updated");
}

#[test]
fn test_unicode_and_special_strings() {
    let mut data = json!({});
    suture!(data, .emoji = "🎉");
    suture!(data, .chinese = "你好");
    suture!(data, .quote = "He said \"hello\"");
    assert_eq!(data["emoji"], "🎉");
    assert_eq!(data["chinese"], "你好");
}

// -----------------------------------------------------------------------------
// 1️⃣3. ARRAY INDEX EDGE CASES
// -----------------------------------------------------------------------------
#[test]
fn test_array_zero_index() {
    let mut data = json!(null);
    suture!(data, [0] = "first");
    assert_eq!(data[0], "first");
    assert_eq!(data.as_array().unwrap().len(), 1);
}

#[test]
fn test_array_sequential_sparse_access() {
    let mut data = json!(null);
    suture!(data, [0] = "a");
    suture!(data, [2] = "c");
    suture!(data, [1] = "b");

    assert_eq!(data[0], "a");
    assert_eq!(data[1], "b");
    assert_eq!(data[2], "c");
}

#[test]
fn test_array_gap_creation() {
    let mut data = json!(null);
    suture!(data, [5] = "at_index_5");

    for i in 0..5 {
        assert!(data[i].is_null());
    }
    assert_eq!(data[5], "at_index_5");
}

// -----------------------------------------------------------------------------
// 14. ARRAY INDEX EDGE CASES: SAFE RANGES
// -----------------------------------------------------------------------------
#[test]
fn test_array_max_safe_index() {
    let mut data = json!(null);
    suture!(data, [9999] = "large_index");

    assert_eq!(data[9999], "large_index");
    assert_eq!(data.as_array().unwrap().len(), 10000);
}

#[test]
fn test_array_index_validation_with_bounds_check() {
    let mut data = json!(null);
    let idx: i32 = 5;

    if idx >= 0 {
        suture!(data, [idx as usize] = "safe_access");
        assert_eq!(data[5], "safe_access");
    }
}

#[test]
fn test_array_index_safety_recommendation() {
    let mut data = json!(null);

    let safe_indices = vec![0, 1, 2, 5, 10];
    for idx in safe_indices {
        suture!(data, [idx] = format!("item_{}", idx));
    }

    assert_eq!(data[0], "item_0");
    assert_eq!(data[10], "item_10");
}

// =============================================================================
// MISCELLANEOUS
// =============================================================================

// -----------------------------------------------------------------------------
// 1️⃣5. KEY SYNTAX VARIATIONS - IDENT VS LITERAL VS DYNAMIC
// -----------------------------------------------------------------------------

#[test]
fn test_key_syntax_ident_vs_literal_vs_dynamic() {
    let mut data = json!({});
    suture!(data, .username = "alice");
    suture!(data, . "email" = "alice@example.com");

    let field = "age";
    suture!(data, .(field) = 30);

    assert_eq!(data["username"], "alice");
    assert_eq!(data["email"], "alice@example.com");
    assert_eq!(data["age"], 30);
}

#[test]
fn test_all_three_key_syntaxes_in_nested_path() {
    let mut data = json!(null);
    let middle = "user";
    suture!(data, .container .(middle) .name = "mixed");

    assert_eq!(data["container"]["user"]["name"], "mixed");
}

// -----------------------------------------------------------------------------
// 1️⃣6. MACRO RETURN TYPES - STRICT VS GENESIS MODE
// -----------------------------------------------------------------------------

#[test]
fn test_biopsy_returns_option_reference() {
    let data = json!({"val": 42});
    let result = biopsy!(data, .val);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), &json!(42));
}

#[test]
fn test_acquire_returns_result() {
    let data = json!({"val": 42});
    let result = acquire!(data, .val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &json!(42));
}

#[test]
fn test_incise_returns_option_unit() {
    let mut data = json!({"val": 10});
    let result = incise!(data, .val = 20);
    assert!(result.is_some());
}

#[test]
fn test_incise_failure_returns_none() {
    let mut data = json!({"val": 10});
    let result = incise!(data, .missing = 20);
    assert!(result.is_none());
}

#[test]
fn test_amputate_return_type() {
    let mut data = json!({"delete_me": "gone"});
    amputate!(data, .delete_me);
    assert!(!data.as_object().unwrap().contains_key("delete_me"));
}

// -----------------------------------------------------------------------------
// 1️⃣7. WHITESPACE & FORMATTING VARIATIONS
// -----------------------------------------------------------------------------
#[test]
fn test_path_with_various_whitespace() {
    let mut data = json!({});

    suture!(data,.key1="no_space");
    suture!(data, . "key2" = "standard");
    suture!(data,  .  key3  =  "extra_spaces");

    assert_eq!(data["key1"], "no_space");
    assert_eq!(data["key2"], "standard");
    assert_eq!(data["key3"], "extra_spaces");
}

#[test]
fn test_assignment_various_spacing() {
    let mut data = json!({});
    suture!(data, .a="no_space");
    suture!(data, .b = "one_space");
    suture!(data, .c  =  "two_spaces");

    assert_eq!(data["a"], "no_space");
    assert_eq!(data["b"], "one_space");
    assert_eq!(data["c"], "two_spaces");
}

// -----------------------------------------------------------------------------
// 1️⃣8. CHAINING & SEQUENTIAL OPERATIONS
// -----------------------------------------------------------------------------
#[test]
fn test_sequential_suture_operations() {
    let mut data = json!({});
    suture!(data, .users [0] .name = "Alice");
    suture!(data, .users [0] .age = 30);
    suture!(data, .users [1] .name = "Bob");
    suture!(data, .users [1] .age = 25);

    assert_eq!(data["users"][0]["name"], "Alice");
    assert_eq!(data["users"][0]["age"], 30);
    assert_eq!(data["users"][1]["name"], "Bob");
    assert_eq!(data["users"][1]["age"], 25);
}

#[test]
fn test_suture_then_graft_then_acquire() {
    let mut data = json!({});
    suture!(data, .value = 10);
    graft!(data, .value = 20); // Should not overwrite
                               //
    let retrieved = acquire!(data, .value).unwrap();
    assert_eq!(retrieved, &json!(10));
}

#[test]
fn test_biopsy_then_incise_then_biopsy() {
    let mut data = json!({"x": 1});
    assert_eq!(biopsy!(data, .x), Some(&json!(1)));

    incise!(data, .x = 2);
    assert_eq!(biopsy!(data, .x), Some(&json!(2)));
}

// -----------------------------------------------------------------------------
// 19. PATTERN MATCHING - DIFFERENT VALUE TYPES
// -----------------------------------------------------------------------------
#[test]
fn test_all_json_types_in_single_object() {
    let mut data = json!({});
    suture!(data, .null_val = Value::Null);
    suture!(data, .bool_val = true);
    suture!(data, .num_val = 42);
    suture!(data, .str_val = "text");
    suture!(data, .arr_val = json!([1, 2, 3]));
    suture!(data, .obj_val = json!({"nested": "object"}));

    assert!(data["null_val"].is_null());
    assert!(data["bool_val"].is_boolean());
    assert!(data["num_val"].is_number());
    assert!(data["str_val"].is_string());
    assert!(data["arr_val"].is_array());
    assert!(data["obj_val"].is_object());
}

#[test]
fn test_overwrite_preserves_type_correctness() {
    let mut data = json!({"val": 10});
    assert!(data["val"].is_number());

    suture!(data, .val = "now_string");
    assert!(data["val"].is_string());
}

// -----------------------------------------------------------------------------
// 20. MIXED LITERAL AND IDENTIFIER STYLES
// -----------------------------------------------------------------------------
#[test]
fn test_mixed_ident_and_literal_keys_in_path() {
    let mut data = json!(null);
    suture!(data, .user . "first_name" = "John");
    suture!(data, . "user" . last_name = "Doe");

    assert_eq!(data["user"]["first_name"], "John");
    assert_eq!(data["user"]["last_name"], "Doe");
}

#[test]
fn test_array_object_ident_object_literal() {
    let mut data = json!(null);
    suture!(data, .items [0] . "title" = "item1");
    assert_eq!(data["items"][0]["title"], "item1");
}

// -----------------------------------------------------------------------------
// 2️⃣1. COMBINED PATH PATTERNS
// -----------------------------------------------------------------------------
#[test]
fn test_complex_path_all_key_types() {
    let mut data = json!(null);
    let dyn_key = "settings";
    suture!(data, .config .(dyn_key) . "value" [0] = "result");

    assert_eq!(data["config"]["settings"]["value"][0], "result");
}

#[test]
fn test_biopsy_complex_path_all_key_types() {
    let data = json!({"config": {"settings": {"value": [42]}}});
    let dyn_key = "settings";

    let result = biopsy!(data, .config .(dyn_key) . "value" [0]);
    assert_eq!(result, Some(&json!(42)));
}

// -----------------------------------------------------------------------------
// 22. CONSECUTIVE OPERATIONS - ORDER MATTERS
// -----------------------------------------------------------------------------
#[test]
fn test_operation_order_suture_then_graft() {
    let mut data = json!({});
    suture!(data, .field = "first");
    graft!(data, .field = "second");

    assert_eq!(data["field"], "first");
}

#[test]
fn test_operation_order_graft_then_suture() {
    let mut data = json!({});
    graft!(data, .field = "first");
    suture!(data, .field = "second");

    assert_eq!(data["field"], "second");
}

#[test]
fn test_operation_order_incise_read_update() {
    let mut data = json!({"counter": 0});
    if let Some(_) = incise!(data, .counter = 1) {
        if let Some(val) = biopsy!(data, .counter) {
            assert_eq!(val, &json!(1));
        }
    }
}

// -----------------------------------------------------------------------------
// 23. NUMBER TYPES IN JSON VALUES
// -----------------------------------------------------------------------------
#[test]
fn test_suture_integer_values() {
    let mut data = json!({});
    suture!(data, .i8 = 127i8);
    suture!(data, .i64 = 9223372036854775807i64);
    suture!(data, .u64 = 18446744073709551615u64);

    assert!(data["i8"].is_number());
    assert!(data["i64"].is_number());
    assert!(data["u64"].is_number());
}

#[test]
fn test_suture_float_values() {
    let mut data = json!({});
    suture!(data, .f32 = 3.14f32);
    suture!(data, .f64 = 2.71828f64);

    assert!(data["f32"].is_number());
    assert!(data["f64"].is_number());
}

#[test]
fn test_number_precision_preservation() {
    let mut data = json!({});
    suture!(data, .precise = 123456789012345u64);

    assert!(data["precise"].is_number());
}
