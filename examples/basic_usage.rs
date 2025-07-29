use coordtransform::*;

fn main() {
    println!("Coordinates transformation examples");
    println!("=============");

    let test_lon = 116.404;
    let test_lat = 39.915;

    println!("Original Coordinates: ({}, {})", test_lon, test_lat);
    println!();

    // BD09 -> GCJ02
    let (lon, lat) = bd09_to_gcj02(test_lon, test_lat);
    println!("BD09 -> GCJ02: ({:.10}, {:.10})", lon, lat);

    // GCJ02 -> BD09
    let (lon, lat) = gcj02_to_bd09(test_lon, test_lat);
    println!("GCJ02 -> BD09: ({:.10}, {:.10})", lon, lat);

    // WGS84 -> GCJ02
    let (lon, lat) = wgs84_to_gcj02(test_lon, test_lat);
    println!("WGS84 -> GCJ02: ({:.10}, {:.10})", lon, lat);

    // GCJ02 -> WGS84
    let (lon, lat) = gcj02_to_wgs84(test_lon, test_lat);
    println!("GCJ02 -> WGS84: ({:.10}, {:.10})", lon, lat);

    // BD09 -> WGS84
    let (lon, lat) = bd09_to_wgs84(test_lon, test_lat);
    println!("BD09 -> WGS84: ({:.10}, {:.10})", lon, lat);

    // WGS84 -> BD09
    let (lon, lat) = wgs84_to_bd09(test_lon, test_lat);
    println!("WGS84 -> BD09: ({:.10}, {:.10})", lon, lat);

    println!();
    println!("Test coordinates outside of China (should remain unchanged):");
    let (lon, lat) = wgs84_to_gcj02(0.0, 0.0);
    println!("WGS84 -> GCJ02 (outside China): ({}, {})", lon, lat);
}
