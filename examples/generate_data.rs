use mmss_core::export::arrow::write_records_to_file;
use mmss_core::structex_bridge::{MmssRecord, PatternMatcher};
use serde_json::json;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = (0..100).map(|i| {
        let metric_type = match i % 4 {
            0 => "cpu",
            1 => "memory",
            2 => "network",
            _ => "disk",
        };
        MmssRecord {
            id: i as u64,
            kind: metric_type.to_string(),
            timestamp: 1732400000 + (i as i64 * 60),
            payload: json!({
                "value": rand::random::<f64>() * 100.0,
                "unit": if metric_type == "network" { "MB/s" } else { "%" },
                "host": format!("host-{}", rand::random::<u8>() % 5 + 1),
            }),
        }
    }).collect::<Vec<_>>();

    write_records_to_file(Path::new("data.arrow"), &records)?;
    Ok(())
}
