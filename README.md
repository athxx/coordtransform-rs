# coordtransform-rs

[![Crates.io](https://img.shields.io/crates/v/coordtransform.svg)](https://crates.io/crates/coordtransform)
[![Documentation](https://docs.rs/coordtransform/badge.svg)](https://docs.rs/coordtransform)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

提供百度坐标系(BD09)、火星坐标系(国测局坐标系、GCJ02)、WGS84坐标系的相互转换，基于 Rust 语言，无特殊依赖。
Provides mutual conversion between Baidu Coordinate System (BD09), Mars Coordinate System (GCJ02), and WGS84 Coordinate System, implemented in Rust with no special dependencies.

这是 [Go版本](https://github.com/qichengzx/coordtransform) 的 Rust 移植版本。
This is the Rust port of the [Go version](https://github.com/qichengzx/coordtransform).

## 坐标系说明 Coordinate System Description

- **WGS84坐标系**：即地球坐标系，国际上通用的坐标系
- **WGS84 Coordinate System**: The Earth coordinate system, commonly used internationally.
- **GCJ02坐标系**：即火星坐标系，WGS84坐标系经加密后的坐标系。Google Maps，高德在用
- **GCJ02 Coordinate System**: Also known as the Mars coordinate system, an encrypted version of the WGS84 coordinate system. Used by Google Maps and Amap.
- **BD09坐标系**：即百度坐标系，GCJ02坐标系经加密后的坐标系
- **BD09 Coordinate System**: Also known as the Baidu coordinate system, an encrypted version of the GCJ02 coordinate system.

## 功能特性 Features

- [x] 火星坐标系 -> 百度坐标系 (`gcj02_to_bd09`)
- [x] 百度坐标系 -> 火星坐标系 (`bd09_to_gcj02`)
- [x] WGS84坐标系 -> 火星坐标系 (`wgs84_to_gcj02`)
- [x] 火星坐标系 -> WGS84坐标系 (`gcj02_to_wgs84`)
- [x] WGS84坐标系 -> 百度坐标系 (`wgs84_to_bd09`)
- [x] 百度坐标系 -> WGS84坐标系 (`bd09_to_wgs84`)

## 安装 Installation

在你的 `Cargo.toml` 中添加 Add the following to your `Cargo.toml`：

```toml
[dependencies]
coordtransform = "0.1.0"
```

## 快速开始 Quick Start

```rust
use coordtransform::*;

fn main() {
    // 百度坐标系 -> 火星坐标系
    let (lon, lat) = bd09_to_gcj02(116.404, 39.915);
    println!("BD09 to GCJ02: ({}, {})", lon, lat);

    // 火星坐标系 -> 百度坐标系
    let (lon, lat) = gcj02_to_bd09(116.404, 39.915);
    println!("GCJ02 to BD09: ({}, {})", lon, lat);

    // WGS84坐标系 -> 火星坐标系
    let (lon, lat) = wgs84_to_gcj02(116.404, 39.915);
    println!("WGS84 to GCJ02: ({}, {})", lon, lat);

    // 火星坐标系 -> WGS84坐标系 gcj02 -> WGS84
    let (lon, lat) = gcj02_to_wgs84(116.404, 39.915);
    println!("GCJ02 to WGS84: ({}, {})", lon, lat);

    // 百度坐标系 -> WGS84坐标系
    let (lon, lat) = bd09_to_wgs84(116.404, 39.915);
    println!("BD09 to WGS84: ({}, {})", lon, lat);

    // WGS84坐标系 -> 百度坐标系
    let (lon, lat) = wgs84_to_bd09(116.404, 39.915);
    println!("WGS84 to BD09: ({}, {})", lon, lat);
}
```

## 基准测试 Benchmarking

运行基准测试 Run the benchmark tests：

```bash
cargo bench
```

## 测试 Testing

运行测试 Run the tests：

```bash
cargo test
```

## 许可证 License

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 相关项目 Related Projects

- [Go版本](https://github.com/qichengzx/coordtransform)
- [Python版本](https://github.com/wandergis/coordTransform_py)
- [JavaScript版本](https://github.com/wandergis/coordtransform)
- [命令行版本Command Line](https://github.com/wandergis/coordtransform-cli)
