//! # coordtransform
//!
//! 提供百度坐标系(BD09)、火星坐标系(国测局坐标系、GCJ02)、WGS84坐标系的相互转换，基于 Rust 语言，无特殊依赖。
//! Provides mutual conversion between Baidu Coordinate System (BD09), Mars Coordinate System (GCJ02), and WGS84 Coordinate System, implemented in Rust with no special dependencies.
//!
//! ## 坐标系说明
//! ## Coordinate System Description
//!
//! - **WGS84坐标系**：即地球坐标系，国际上通用的坐标系
//! - **WGS84 Coordinate System**: The Earth coordinate system, commonly used internationally.
//! - **GCJ02坐标系**：即火星坐标系，WGS84坐标系经加密后的坐标系。Google Maps，高德在用
//! - **GCJ02 Coordinate System**: Also known as the Mars coordinate system, an encrypted version of the WGS84 coordinate system. Used by Google Maps and Amap.
//! - **BD09坐标系**：即百度坐标系，GCJ02坐标系经加密后的坐标系
//! - **BD09 Coordinate System**: Also known as the Baidu coordinate system, an encrypted version of the GCJ02 coordinate system.
//!
//! ## Usage Example 使用示例
//!
//! ```rust
//! use coordtransform::*;
//!
//! // bd09百度坐标系 -> gcj02火星坐标系
//! let (lon, lat) = bd09_to_gcj02(116.404, 39.915);
//!
//! // bd09火星坐标系 -> gcj02百度坐标系
//! let (lon, lat) = gcj02_to_bd09(116.404, 39.915);
//!
//! // WGS84坐标系 -> gcj02火星坐标系
//! let (lon, lat) = wgs84_to_gcj02(116.404, 39.915);
//!
//! // gcj02火星坐标系 -> WGS84坐标系
//! let (lon, lat) = gcj02_to_wgs84(116.404, 39.915);
//!
//! // bd09百度坐标系 -> WGS84坐标系
//! let (lon, lat) = bd09_to_wgs84(116.404, 39.915);
//!
//! // WGS84坐标系 -> bd09百度坐标系
//! let (lon, lat) = wgs84_to_bd09(116.404, 39.915);
//! ```

use std::f64::consts::PI;

/// X_PI constant 常量
///
const X_PI: f64 = PI * 3000.0 / 180.0;

/// Offset constant 偏移量常量
const OFFSET: f64 = 0.00669342162296594323;

/// 地球长半轴
/// Earth's semi-major axis
const AXIS: f64 = 6378245.0;

/// 百度坐标系 -> 火星坐标系
/// Baidu Coordinate System -> Mars Coordinate System
///
/// # Parameters 参数
///
/// * `lon` - 经度 Longitude
/// * `lat` - 纬度 Latitude
///
/// # 返回值 Return Value
///
/// 返回转换后的 (经度, 纬度) 元组 Returns a tuple of (longitude, latitude) after conversion
///
/// # 示例 Example
///
/// ```rust
/// use coordtransform::bd09_to_gcj02;
///
/// let (lon, lat) = bd09_to_gcj02(116.404, 39.915);
/// ```
pub fn bd09_to_gcj02(lon: f64, lat: f64) -> (f64, f64) {
    let x = lon - 0.0065;
    let y = lat - 0.006;

    let z = (x * x + y * y).sqrt() - 0.00002 * (y * X_PI).sin();
    let theta = y.atan2(x) - 0.000003 * (x * X_PI).cos();

    let g_lon = z * theta.cos();
    let g_lat = z * theta.sin();

    (g_lon, g_lat)
}

/// gcj02火星坐标系 -> bd09百度坐标系
///
/// # Parameters 参数
///
/// * `lon` - 经度 Longitude
/// * `lat` - 纬度 Latitude
///
/// # 返回值 Return Value
///
/// 返回转换后的 (经度, 纬度) 元组 Returns a tuple of (longitude, latitude) after conversion
///
/// # 示例 Example
///
/// ```rust
/// use coordtransform::gcj02_to_bd09;
///
/// let (lon, lat) = gcj02_to_bd09(116.404, 39.915);
/// ```
pub fn gcj02_to_bd09(lon: f64, lat: f64) -> (f64, f64) {
    let z = (lon * lon + lat * lat).sqrt() + 0.00002 * (lat * X_PI).sin();
    let theta = lat.atan2(lon) + 0.000003 * (lon * X_PI).cos();

    let bd_lon = z * theta.cos() + 0.0065;
    let bd_lat = z * theta.sin() + 0.006;

    (bd_lon, bd_lat)
}

/// WGS84坐标系 -> 火星坐标系
/// WGS84 Coordinate System -> Mars Coordinate System
///
/// # Parameters 参数
///
/// * `lon` - 经度 Longitude
/// * `lat` - 纬度 Latitude
///
/// # 返回值 Return Value
///
/// 返回转换后的 (经度, 纬度) 元组 Returns a tuple of (longitude, latitude) after conversion
///
/// # 示例 Example
///
/// ```rust
/// use coordtransform::wgs84_to_gcj02;
///
/// let (lon, lat) = wgs84_to_gcj02(116.404, 39.915);
/// ```
pub fn wgs84_to_gcj02(lon: f64, lat: f64) -> (f64, f64) {
    if is_out_of_china(lon, lat) {
        return (lon, lat);
    }

    delta(lon, lat)
}

/// gcj02火星坐标系 -> WGS84坐标系
///
/// # Parameters 参数
///
/// * `lon` - 经度 Longitude
/// * `lat` - 纬度 Latitude
///
/// # Return Value 返回值
///
/// 返回转换后的 (经度, 纬度) 元组 Returns a tuple of (longitude, latitude) after conversion
///
/// # 示例 Example
///
/// ```rust
/// use coordtransform::gcj02_to_wgs84;
///
/// let (lon, lat) = gcj02_to_wgs84(116.404, 39.915);
/// ```
pub fn gcj02_to_wgs84(lon: f64, lat: f64) -> (f64, f64) {
    if is_out_of_china(lon, lat) {
        return (lon, lat);
    }

    let (mg_lon, mg_lat) = delta(lon, lat);

    (lon * 2.0 - mg_lon, lat * 2.0 - mg_lat)
}

/// 百度坐标系 -> WGS84坐标系
/// Baidu Coordinate System -> WGS84 Coordinate System
///
/// # Parameters 参数
///
/// * `lon` - 经度 Longitude
/// * `lat` - 纬度 Latitude
///
/// # Return Value 返回值
///
/// 返回转换后的 (经度, 纬度) 元组 Returns a tuple of (longitude, latitude) after conversion
///
/// # 示例 Example
///
/// ```rust
/// use coordtransform::bd09_to_wgs84;
///
/// let (lon, lat) = bd09_to_wgs84(116.404, 39.915);
/// ```
pub fn bd09_to_wgs84(lon: f64, lat: f64) -> (f64, f64) {
    let (lon, lat) = bd09_to_gcj02(lon, lat);
    gcj02_to_wgs84(lon, lat)
}

/// WGS84坐标系 -> 百度坐标系
/// WGS84 Coordinate System -> Baidu Coordinate System
///
/// # Parameters 参数
///
/// * `lon` - 经度 Longitude
/// * `lat` - 纬度 Latitude
///
/// # Return Value 返回值
///
/// 返回转换后的 (经度, 纬度) 元组 Returns a tuple of (longitude, latitude) after conversion
///
/// # Example 示例
///
/// ```rust
/// use coordtransform::wgs84_to_bd09;
///
/// let (lon, lat) = wgs84_to_bd09(116.404, 39.915);
/// ```
pub fn wgs84_to_bd09(lon: f64, lat: f64) -> (f64, f64) {
    let (lon, lat) = wgs84_to_gcj02(lon, lat);
    gcj02_to_bd09(lon, lat)
}

/// Calculate coordinate offset 计算坐标偏移量
fn delta(lon: f64, lat: f64) -> (f64, f64) {
    let (dlat, dlon) = transform(lon - 105.0, lat - 35.0);
    let radlat = lat / 180.0 * PI;
    let magic = radlat.sin();
    let magic = 1.0 - OFFSET * magic * magic;
    let sqrtmagic = magic.sqrt();

    let dlat = (dlat * 180.0) / ((AXIS * (1.0 - OFFSET)) / (magic * sqrtmagic) * PI);
    let dlon = (dlon * 180.0) / (AXIS / sqrtmagic * radlat.cos() * PI);

    let mg_lat = lat + dlat;
    let mg_lon = lon + dlon;

    (mg_lon, mg_lat)
}

/// Coordinate transformation function 坐标变换函数
fn transform(lon: f64, lat: f64) -> (f64, f64) {
    let lonlat = lon * lat;
    let abs_x = lon.abs().sqrt();
    let lon_pi = lon * PI;
    let lat_pi = lat * PI;
    let d = 20.0 * (6.0 * lon_pi).sin() + 20.0 * (2.0 * lon_pi).sin();

    let mut x = d;
    let mut y = d;

    x += 20.0 * lat_pi.sin() + 40.0 * (lat_pi / 3.0).sin();
    y += 20.0 * lon_pi.sin() + 40.0 * (lon_pi / 3.0).sin();
    x += 160.0 * (lat_pi / 12.0).sin() + 320.0 * (lat_pi / 30.0).sin();
    y += 150.0 * (lon_pi / 12.0).sin() + 300.0 * (lon_pi / 30.0).sin();

    x *= 2.0 / 3.0;
    y *= 2.0 / 3.0;

    x += -100.0 + 2.0 * lon + 3.0 * lat + 0.2 * lat * lat + 0.1 * lonlat + 0.2 * abs_x;
    y += 300.0 + lon + 2.0 * lat + 0.1 * lon * lon + 0.1 * lonlat + 0.1 * abs_x;

    (x, y)
}

/// Determine whether the coordinates are outside of China 判断坐标是否在中国境外
fn is_out_of_china(lon: f64, lat: f64) -> bool {
    !(lon > 72.004 && lon < 135.05 && lat > 3.86 && lat < 53.55)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bd09_to_gcj02() {
        let (lon, lat) = bd09_to_gcj02(116.404, 39.915);
        assert!((lon - 116.39762729119315).abs() < 1e-10);
        assert!((lat - 39.90865673957631).abs() < 1e-10);
    }

    #[test]
    fn test_gcj02_to_bd09() {
        let (lon, lat) = gcj02_to_bd09(116.404, 39.915);
        assert!((lon - 116.41036949371029).abs() < 1e-10);
        assert!((lat - 39.92133699351022).abs() < 1e-10);
    }

    #[test]
    fn test_wgs84_to_gcj02() {
        let (lon, lat) = wgs84_to_gcj02(116.404, 39.915);
        assert!((lon - 116.41024449916938).abs() < 1e-10);
        assert!((lat - 39.91640428150164).abs() < 1e-10);
    }

    #[test]
    fn test_gcj02_to_wgs84() {
        let (lon, lat) = gcj02_to_wgs84(116.404, 39.915);
        assert!((lon - 116.39775550083061).abs() < 1e-10);
        assert!((lat - 39.91359571849836).abs() < 1e-10);
    }

    #[test]
    fn test_bd09_to_wgs84() {
        let (lon, lat) = bd09_to_wgs84(116.404, 39.915);
        assert!((lon - 116.39138369954496).abs() < 1e-10);
        assert!((lat - 39.90725321448524).abs() < 1e-10);
    }

    #[test]
    fn test_wgs84_to_bd09() {
        let (lon, lat) = wgs84_to_bd09(116.404, 39.915);
        assert!((lon - 116.41662724378733).abs() < 1e-10);
        assert!((lat - 39.922699552216216).abs() < 1e-10);
    }

    #[test]
    fn test_out_of_china() {
        // 测试中国境外坐标，应该直接返回原坐标
        // Test coordinates outside of China, should directly return the original coordinates
        let (lon, lat) = wgs84_to_gcj02(0.0, 0.0);
        assert_eq!(lon, 0.0);
        assert_eq!(lat, 0.0);
    }
}
