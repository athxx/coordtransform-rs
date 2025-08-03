use coordtransform::*;

fn main() {
    println!("EPSG:3857 (Web Mercator) Coordinate Transformation Examples");
    println!("===========================================================");

    // 测试坐标：北京天安门
    // Test coordinates: Tiananmen Square, Beijing
    let beijing_lon = 116.404;
    let beijing_lat = 39.915;

    println!("Original coordinates (Beijing): ({}, {})", beijing_lon, beijing_lat);
    println!();

    // WGS84 <-> EPSG:3857
    println!("WGS84 <-> EPSG:3857 transformations:");
    let (x, y) = wgs84_to_epsg3857(beijing_lon, beijing_lat);
    println!("WGS84 -> EPSG:3857: ({:.2}, {:.2})", x, y);
    
    let (lon, lat) = epsg3857_to_wgs84(x, y);
    println!("EPSG:3857 -> WGS84: ({:.10}, {:.10})", lon, lat);
    println!();

    // GCJ02 <-> EPSG:3857
    println!("GCJ02 <-> EPSG:3857 transformations:");
    let (x, y) = gcj02_to_epsg3857(beijing_lon, beijing_lat);
    println!("GCJ02 -> EPSG:3857: ({:.2}, {:.2})", x, y);
    
    let (lon, lat) = epsg3857_to_gcj02(x, y);
    println!("EPSG:3857 -> GCJ02: ({:.10}, {:.10})", lon, lat);
    println!();

    // BD09 <-> EPSG:3857
    println!("BD09 <-> EPSG:3857 transformations:");
    let (x, y) = bd09_to_epsg3857(beijing_lon, beijing_lat);
    println!("BD09 -> EPSG:3857: ({:.2}, {:.2})", x, y);
    
    let (lon, lat) = epsg3857_to_bd09(x, y);
    println!("EPSG:3857 -> BD09: ({:.10}, {:.10})", lon, lat);
    println!();

    // 测试不同地区的坐标
    // Test coordinates from different regions
    println!("Testing coordinates from different regions:");
    
    let test_coords = [
        ("Shanghai", 121.4737, 31.2304),
        ("Guangzhou", 113.2644, 23.1291),
        ("Shenzhen", 114.0579, 22.5431),
        ("London", 0.1276, 51.5074),
        ("New York", -74.0060, 40.7128),
    ];

    for (city, lon, lat) in test_coords.iter() {
        let (x, y) = wgs84_to_epsg3857(*lon, *lat);
        println!("{}: WGS84({:.4}, {:.4}) -> EPSG:3857({:.2}, {:.2})", 
                 city, lon, lat, x, y);
    }
    println!();

    // 演示精度测试
    // Demonstrate precision test
    println!("Precision test (round-trip conversion):");
    let original_coords = [(116.404, 39.915), (121.4737, 31.2304), (-74.0060, 40.7128)];
    
    for (orig_lon, orig_lat) in original_coords.iter() {
        let (x, y) = wgs84_to_epsg3857(*orig_lon, *orig_lat);
        let (back_lon, back_lat) = epsg3857_to_wgs84(x, y);
        let lon_diff = (back_lon - orig_lon).abs();
        let lat_diff = (back_lat - orig_lat).abs();
        
        println!("Original: ({:.10}, {:.10})", orig_lon, orig_lat);
        println!("Round-trip: ({:.10}, {:.10})", back_lon, back_lat);
        println!("Difference: ({:.2e}, {:.2e})", lon_diff, lat_diff);
        println!();
    }
}