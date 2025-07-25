use coordtransform::*;

fn main() {
    println!("坐标转换示例");
    println!("=============");
    
    let test_lon = 116.404;
    let test_lat = 39.915;
    
    println!("原始坐标: ({}, {})", test_lon, test_lat);
    println!();
    
    // 百度坐标系 -> 火星坐标系
    let (lon, lat) = bd09_to_gcj02(test_lon, test_lat);
    println!("BD09 -> GCJ02: ({:.10}, {:.10})", lon, lat);
    
    // 火星坐标系 -> 百度坐标系
    let (lon, lat) = gcj02_to_bd09(test_lon, test_lat);
    println!("GCJ02 -> BD09: ({:.10}, {:.10})", lon, lat);
    
    // WGS84坐标系 -> 火星坐标系
    let (lon, lat) = wgs84_to_gcj02(test_lon, test_lat);
    println!("WGS84 -> GCJ02: ({:.10}, {:.10})", lon, lat);
    
    // 火星坐标系 -> WGS84坐标系
    let (lon, lat) = gcj02_to_wgs84(test_lon, test_lat);
    println!("GCJ02 -> WGS84: ({:.10}, {:.10})", lon, lat);
    
    // 百度坐标系 -> WGS84坐标系
    let (lon, lat) = bd09_to_wgs84(test_lon, test_lat);
    println!("BD09 -> WGS84: ({:.10}, {:.10})", lon, lat);
    
    // WGS84坐标系 -> 百度坐标系
    let (lon, lat) = wgs84_to_bd09(test_lon, test_lat);
    println!("WGS84 -> BD09: ({:.10}, {:.10})", lon, lat);
    
    println!();
    println!("测试中国境外坐标（应该保持不变）:");
    let (lon, lat) = wgs84_to_gcj02(0.0, 0.0);
    println!("WGS84 -> GCJ02 (境外): ({}, {})", lon, lat);
}