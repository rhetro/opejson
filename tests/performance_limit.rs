use opejson::prelude::*;

#[test]
fn benchmark_opejson_limit_break() {
    use serde_json::json;
    use std::time::Instant;

    println!("\n🚀 --- Opejson Performance Benchmark ---");

    // 1. Vertical Penetration (256 Levels)
    let mut data = json!(null);
    let start_vp = Instant::now();
    let mut temp = &mut data;
    for i in 0..256 {
        let key = format!("v{}", i);
        suture!(*temp, .(&key) = json!({}));
        temp = temp.get_mut(&key).unwrap();
    }
    suture!(*temp, .abyss = "reached");
    let duration_vp = start_vp.elapsed();
    println!("📍 256-Level Penetration: {:?}", duration_vp);

    // 2. Massive Stitching (100,000 Sutures)
    let mut data_massive = json!({});
    let start_ms = Instant::now();
    for i in 0..100_000 {
        let k1 = (i % 100).to_string();
        let k2 = (i / 100 % 100).to_string();
        suture!(data_massive, .(k1) .(k2) = i);
    }
    let duration_ms = start_ms.elapsed();
    println!("🧵 100,000 Massive Sutures: {:?}", duration_ms);

    // 3. Calculation for X (Total Score)
    let total_ops = 100_000 + 256;
    let total_time = duration_vp + duration_ms;
    let ops_per_sec = (total_ops as f64) / total_time.as_secs_f64();

    println!("-----------------------------------------");
    println!("🏆 FINAL SCORE: {:.2} operations/sec", ops_per_sec);
    println!("-----------------------------------------\n");
}
