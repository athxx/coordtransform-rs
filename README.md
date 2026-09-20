# coordtransform-rs

[![Crates.io](https://img.shields.io/crates/v/coordtransform.svg)](https://crates.io/crates/coordtransform)
[![Documentation](https://docs.rs/coordtransform/badge.svg)](https://docs.rs/coordtransform)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Provides coordinate transformations between BD09, GCJ02, WGS84, and EPSG:3857, implemented in Rust with no third-party dependencies.

This is the Rust port of the [Go version](https://github.com/qichengzx/coordtransform).

## Coordinate System Description

- **WGS84**: Global longitude/latitude coordinate system used by GPS/GNSS and most GIS datasets.
- **GCJ02**: Offset coordinate system commonly used by map services in mainland China.
- **BD09**: Baidu coordinate system derived from GCJ02.
- **EPSG:3857**: Web Mercator projection widely used by web mapping services.

## Features

- [x] GCJ02 -> BD09 (`gcj02_to_bd09`)
- [x] BD09 -> GCJ02 (`bd09_to_gcj02`)
- [x] BD09 -> GCJ02 fast mode (`bd09_to_gcj02_fast`)
- [x] WGS84 -> GCJ02 (`wgs84_to_gcj02`)
- [x] GCJ02 -> WGS84 (`gcj02_to_wgs84`)
- [x] GCJ02 -> WGS84 fast mode (`gcj02_to_wgs84_fast`)
- [x] WGS84 -> BD09 (`wgs84_to_bd09`)
- [x] BD09 -> WGS84 (`bd09_to_wgs84`)
- [x] WGS84 -> EPSG:3857 (`wgs84_to_epsg3857`)
- [x] EPSG:3857 -> WGS84 (`epsg3857_to_wgs84`)
- [x] GCJ02 -> EPSG:3857 (`gcj02_to_epsg3857`)
- [x] EPSG:3857 -> GCJ02 (`epsg3857_to_gcj02`)
- [x] BD09 -> EPSG:3857 (`bd09_to_epsg3857`)
- [x] EPSG:3857 -> BD09 (`epsg3857_to_bd09`)
- [x] Mainland China GCJ02 region check (`is_in_gcj02_region`)
- [x] High-performance mainland conversion APIs
- [x] In-place batch conversion APIs

GCJ02/BD09 offsets are only applied inside the mainland China transform region. Outside the region, geographic coordinates remain WGS84.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
coordtransform = "0.3.0"
```

## Quick Start

```rust
use coordtransform::*;

fn main() {
    let (lon, lat) = bd09_to_gcj02(116.404, 39.915);
    println!("BD09 to GCJ02: ({}, {})", lon, lat);

    let (lon, lat) = gcj02_to_bd09(116.404, 39.915);
    println!("GCJ02 to BD09: ({}, {})", lon, lat);

    let (lon, lat) = wgs84_to_gcj02(116.404, 39.915);
    println!("WGS84 to GCJ02: ({}, {})", lon, lat);

    let (lon, lat) = gcj02_to_wgs84(116.404, 39.915);
    println!("GCJ02 to WGS84: ({}, {})", lon, lat);

    let (lon, lat) = wgs84_to_bd09(116.404, 39.915);
    println!("WGS84 to BD09: ({}, {})", lon, lat);

    let (lon, lat) = bd09_to_wgs84(116.404, 39.915);
    println!("BD09 to WGS84: ({}, {})", lon, lat);

    let (x, y) = wgs84_to_epsg3857(116.404, 39.915);
    println!("WGS84 to EPSG:3857: ({}, {})", x, y);

    let (lon, lat) = epsg3857_to_wgs84(12958752.0, 4825923.0);
    println!("EPSG:3857 to WGS84: ({}, {})", lon, lat);

    let (x, y) = gcj02_to_epsg3857(116.404, 39.915);
    println!("GCJ02 to EPSG:3857: ({}, {})", x, y);

    let (lon, lat) = epsg3857_to_gcj02(12958752.0, 4825923.0);
    println!("EPSG:3857 to GCJ02: ({}, {})", lon, lat);

    let (x, y) = bd09_to_epsg3857(116.404, 39.915);
    println!("BD09 to EPSG:3857: ({}, {})", x, y);

    let (lon, lat) = epsg3857_to_bd09(12958752.0, 4825923.0);
    println!("EPSG:3857 to BD09: ({}, {})", lon, lat);
}
```

## Other APIs

```rust
use coordtransform::*;

let inside = is_in_gcj02_region(116.404, 39.915);

let gcj = wgs84_to_gcj02_in_mainland(116.404, 39.915);
let wgs = gcj02_to_wgs84_in_mainland(gcj.0, gcj.1);

let _ = gcj02_to_wgs84_fast(116.404, 39.915);
let _ = bd09_to_gcj02_fast(116.404, 39.915);

let mut points = [(116.404, 39.915), (121.4737, 31.2304)];
wgs84_to_gcj02_in_place(&mut points);
```

Batch APIs:

- `wgs84_to_gcj02_in_place`
- `gcj02_to_wgs84_in_place`
- `wgs84_to_gcj02_in_mainland_in_place`
- `gcj02_to_wgs84_in_mainland_in_place`
- `wgs84_to_bd09_in_place`
- `bd09_to_wgs84_in_place`

Strict EPSG:3857 APIs:

- `try_wgs84_to_epsg3857`
- `try_epsg3857_to_wgs84`
- `wgs84_to_epsg3857_clamped`

## Testing

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The embedded mainland China transform-region data is derived from OpenStreetMap data and remains subject to ODbL 1.0.

OpenStreetMap contributors: https://www.openstreetmap.org/copyright

## Related Projects

- [Go version](https://github.com/qichengzx/coordtransform)
- [Python version](https://github.com/wandergis/coordTransform_py)
- [JavaScript version](https://github.com/wandergis/coordtransform)
- [Command Line](https://github.com/wandergis/coordtransform-cli)

---

# 中文

提供 BD09、GCJ02、WGS84、EPSG:3857 坐标系之间的相互转换，基于 Rust 实现，无第三方依赖。

这是 [Go版本](https://github.com/qichengzx/coordtransform) 的 Rust 移植版本。

## 坐标系说明

- **WGS84坐标系**：GPS/GNSS 和大多数 GIS 数据使用的全球经纬度坐标系。
- **GCJ02坐标系**：中国大陆地图服务常用的偏移坐标系。
- **BD09坐标系**：百度坐标系，基于 GCJ02 进行转换。
- **EPSG:3857坐标系**：Web Mercator 投影坐标系，广泛用于 Web 地图服务。

## 功能特性

- [x] GCJ02 -> BD09 (`gcj02_to_bd09`)
- [x] BD09 -> GCJ02 (`bd09_to_gcj02`)
- [x] BD09 -> GCJ02 快速模式 (`bd09_to_gcj02_fast`)
- [x] WGS84 -> GCJ02 (`wgs84_to_gcj02`)
- [x] GCJ02 -> WGS84 (`gcj02_to_wgs84`)
- [x] GCJ02 -> WGS84 快速模式 (`gcj02_to_wgs84_fast`)
- [x] WGS84 -> BD09 (`wgs84_to_bd09`)
- [x] BD09 -> WGS84 (`bd09_to_wgs84`)
- [x] WGS84 -> EPSG:3857 (`wgs84_to_epsg3857`)
- [x] EPSG:3857 -> WGS84 (`epsg3857_to_wgs84`)
- [x] GCJ02 -> EPSG:3857 (`gcj02_to_epsg3857`)
- [x] EPSG:3857 -> GCJ02 (`epsg3857_to_gcj02`)
- [x] BD09 -> EPSG:3857 (`bd09_to_epsg3857`)
- [x] EPSG:3857 -> BD09 (`epsg3857_to_bd09`)
- [x] 中国大陆 GCJ02 转换区域判断 (`is_in_gcj02_region`)
- [x] 中国大陆高性能转换 API
- [x] 原地批量转换 API

GCJ02/BD09 偏移只在中国大陆转换区域内应用。区域外的地理坐标保持 WGS84。

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
coordtransform = "0.3.0"
```

## 快速开始

```rust
use coordtransform::*;

fn main() {
    let (lon, lat) = bd09_to_gcj02(116.404, 39.915);
    println!("BD09 to GCJ02: ({}, {})", lon, lat);

    let (lon, lat) = gcj02_to_bd09(116.404, 39.915);
    println!("GCJ02 to BD09: ({}, {})", lon, lat);

    let (lon, lat) = wgs84_to_gcj02(116.404, 39.915);
    println!("WGS84 to GCJ02: ({}, {})", lon, lat);

    let (lon, lat) = gcj02_to_wgs84(116.404, 39.915);
    println!("GCJ02 to WGS84: ({}, {})", lon, lat);

    let (lon, lat) = wgs84_to_bd09(116.404, 39.915);
    println!("WGS84 to BD09: ({}, {})", lon, lat);

    let (lon, lat) = bd09_to_wgs84(116.404, 39.915);
    println!("BD09 to WGS84: ({}, {})", lon, lat);

    let (x, y) = wgs84_to_epsg3857(116.404, 39.915);
    println!("WGS84 to EPSG:3857: ({}, {})", x, y);

    let (lon, lat) = epsg3857_to_wgs84(12958752.0, 4825923.0);
    println!("EPSG:3857 to WGS84: ({}, {})", lon, lat);

    let (x, y) = gcj02_to_epsg3857(116.404, 39.915);
    println!("GCJ02 to EPSG:3857: ({}, {})", x, y);

    let (lon, lat) = epsg3857_to_gcj02(12958752.0, 4825923.0);
    println!("EPSG:3857 to GCJ02: ({}, {})", lon, lat);

    let (x, y) = bd09_to_epsg3857(116.404, 39.915);
    println!("BD09 to EPSG:3857: ({}, {})", x, y);

    let (lon, lat) = epsg3857_to_bd09(12958752.0, 4825923.0);
    println!("EPSG:3857 to BD09: ({}, {})", lon, lat);
}
```

## 其他 API

```rust
use coordtransform::*;

// 判断是否位于 GCJ02 转换区域
let inside = is_in_gcj02_region(116.404, 39.915);

// 已知坐标位于中国大陆时，可以跳过区域判断
let gcj = wgs84_to_gcj02_in_mainland(116.404, 39.915);
let wgs = gcj02_to_wgs84_in_mainland(gcj.0, gcj.1);

// 快速逆转换
let _ = gcj02_to_wgs84_fast(116.404, 39.915);
let _ = bd09_to_gcj02_fast(116.404, 39.915);

// 原地批量转换
let mut points = [(116.404, 39.915), (121.4737, 31.2304)];
wgs84_to_gcj02_in_place(&mut points);
```

批量转换 API：

- `wgs84_to_gcj02_in_place`
- `gcj02_to_wgs84_in_place`
- `wgs84_to_gcj02_in_mainland_in_place`
- `gcj02_to_wgs84_in_mainland_in_place`
- `wgs84_to_bd09_in_place`
- `bd09_to_wgs84_in_place`

严格 EPSG:3857 API：

- `try_wgs84_to_epsg3857`
- `try_epsg3857_to_wgs84`
- `wgs84_to_epsg3857_clamped`

## 测试

```bash
cargo test
```

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

内嵌的中国大陆转换区域数据基于 OpenStreetMap 数据简化生成，相关地理数据遵循 ODbL 1.0。

OpenStreetMap contributors: https://www.openstreetmap.org/copyright

## 相关项目

- [Go版本](https://github.com/qichengzx/coordtransform)
- [Python版本](https://github.com/wandergis/coordTransform_py)
- [JavaScript版本](https://github.com/wandergis/coordtransform)
- [命令行版本](https://github.com/wandergis/coordtransform-cli)
