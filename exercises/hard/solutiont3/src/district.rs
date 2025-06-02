use std::collections::HashMap;
use std::fs;
use serde_json::Value;

pub fn count_provinces() -> String {
    let file_path = "/Users/administrator/rsproject/Rust-Professional/exercises/hard/solutiont3/district.json";
    let data_str = std::fs::read_to_string(file_path).expect("读取文件失败");

    // 解析为 Value 类型，支持任意合法 JSON 结构
    let json: Value = serde_json::from_str(&data_str).expect("JSON 解析失败");

    // 遍历每个区域（"1", "2", ...）
    for (region_key, region_value) in json.as_object().unwrap() {
        println!("Region: {}", region_key);
        for (city, neighbors) in region_value.as_object().unwrap() {
            println!("  City: {}, Neighbors: {:?}", city, neighbors);
        }
    }

    "省份数量".to_string()
}
