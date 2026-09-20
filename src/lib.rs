//! High-accuracy, dependency-free coordinate transforms for WGS 84, GCJ-02,
//! BD-09, and Web Mercator (EPSG:3857).
//!
//! The crate is designed for mobile applications, navigation, track processing,
//! and other high-frequency workloads. It has no third-party crate dependencies
//! and performs no heap allocation on the scalar conversion hot path.
//!
//! # Coordinate systems
//!
//! - **WGS 84**: global longitude/latitude coordinates used by GNSS and most GIS data.
//! - **GCJ-02**: the coordinate system commonly used by map services in mainland China.
//! - **BD-09**: Baidu's coordinate system, derived from GCJ-02.
//! - **EPSG:3857**: Web Mercator coordinates in metres.
//!
//! # GCJ-02 transform region
//!
//! The traditional implementation only checks a large rectangle and therefore
//! applies GCJ-02 offsets to locations in nearby countries. This implementation
//! instead embeds a small, simplified, OSM-derived geographic mask directly in
//! the binary. No boundary crate, parser, file I/O, lazy initialization, or heap
//! allocation is needed at runtime.
//!
//! The embedded mask is deliberately approximate. It is intended only to decide
//! whether the GCJ-02 transform should be dispatched for ordinary land-based map
//! and navigation positions. It is not a legal, cadastral, maritime, or surveying
//! boundary. Hong Kong and Macau are carved out of the mask, and Taiwan is outside
//! the main ring.
//!
//! The embedded ring is a simplified and quantized transform-dispatch mask based
//! on the OpenStreetMap China boundary relation (relation 270056), simplified and
//! quantized offline for transform dispatch. Remote offshore geometry south of 18 N
//! is intentionally ignored because this crate targets ordinary mainland/Hainan app
//! navigation rather than maritime boundary classification. The mask is intentionally
//! approximate near borders and must not be used for administrative decisions.
//! The embedded geographic data remains subject to the Open Database License (ODbL);
//! attribution: (c) OpenStreetMap contributors. See
//! <https://www.openstreetmap.org/copyright>.
//!
//! # Accuracy
//!
//! The commonly published GCJ-02 formula does not provide an exact inverse.
//! [`gcj02_to_wgs84`] uses a fast inverse seed followed by fixed numerical
//! refinement. [`bd09_to_gcj02`] similarly refines the conventional analytical
//! approximation against the forward BD-09 formula.
//!
//! Fast single-pass variants are also provided when throughput is more important
//! than the smallest possible round-trip residual.
//!
//! # Performance
//!
//! Region lookup uses three stages:
//!
//! 1. A broad bounding-box rejection.
//! 2. A precomputed 1-degree raster. Interior and exterior cells return in O(1).
//! 3. Only cells touched by the simplified boundary run a point-in-polygon test.
//!
//! The polygon itself is stored as quantized `u16` coordinates at 0.001-degree
//! resolution. The region data is static, requires no initialization, and is only
//! a few kilobytes. Callers that already know a stream stays inside the transform
//! region can use the `*_in_mainland` functions and skip region lookup entirely.
//!
//! # Quick start
//!
//! ```
//! use coordtransform::{gcj02_to_wgs84, wgs84_to_gcj02};
//!
//! let wgs = (116.404, 39.915);
//! let gcj = wgs84_to_gcj02(wgs.0, wgs.1);
//! let restored = gcj02_to_wgs84(gcj.0, gcj.1);
//!
//! assert!((restored.0 - wgs.0).abs() < 1e-9);
//! assert!((restored.1 - wgs.1).abs() < 1e-9);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::f64::consts::PI;
use std::fmt;

const DEG_TO_RAD: f64 = PI / 180.0;
const RAD_TO_DEG: f64 = 180.0 / PI;

const GCJ_AXIS: f64 = 6_378_245.0;
const GCJ_ECCENTRICITY_SQUARED: f64 = 0.006_693_421_622_965_943;
const GCJ_DEGREE_SCALE: f64 = RAD_TO_DEG / GCJ_AXIS;
const GCJ_INVERSE_REFINEMENTS: usize = 2;

const X_PI: f64 = PI * 3000.0 / 180.0;
const BD_LON_OFFSET: f64 = 0.0065;
const BD_LAT_OFFSET: f64 = 0.006;
const BD_RADIAL_EPSILON: f64 = 0.00002;
const BD_ANGULAR_EPSILON: f64 = 0.000003;
const BD_INVERSE_REFINEMENTS: usize = 2;

const WEB_MERCATOR_RADIUS: f64 = 6_378_137.0;
const WEB_MERCATOR_MAX_COORDINATE: f64 = PI * WEB_MERCATOR_RADIUS;

const GCJ_REGION_MIN_LON: f64 = 73.0;
const GCJ_REGION_MAX_LON: f64 = 135.2;
const GCJ_REGION_MIN_LAT: f64 = 18.0;
const GCJ_REGION_MAX_LAT: f64 = 54.0;
const GCJ_RING_LON_ORIGIN: f64 = 70.0;
const GCJ_RING_SCALE: f64 = 1000.0;

/// Maximum absolute latitude representable in the canonical finite EPSG:3857 world.
pub const MAX_LATITUDE: f64 = 85.051_128_779_806_6;

/// Attribution text for the embedded OpenStreetMap-derived transform-region mask.
///
/// Applications redistributing the embedded boundary data should retain appropriate
/// OpenStreetMap attribution and make the ODbL terms available to users.
pub const OSM_ATTRIBUTION: &str =
    "Boundary mask derived from OpenStreetMap data (c) OpenStreetMap contributors, ODbL 1.0";

// Boundary snapshot prepared 2026-09-21.
// Embedded geographic data: simplified/quantized from OpenStreetMap relation 270056.
// Data attribution: (c) OpenStreetMap contributors, ODbL 1.0.
// This is a transform-dispatch mask, not a legal administrative boundary.
// Coordinates are stored as ((longitude - 70) * 1000, latitude * 1000).
const GCJ_REGION_RING: [(u16, u16); 158] = [
    (47935, 23341),
    (47402, 22960),
    (47291, 15644),
    (48437, 15328),
    (47476, 14385),
    (40667, 14274),
    (38926, 18023),
    (38405, 18382),
    (38315, 19326),
    (39070, 20139),
    (39424, 20681),
    (38774, 21293),
    (38111, 21178),
    (37996, 21544),
    (37542, 21581),
    (36995, 21827),
    (36536, 22354),
    (36591, 22609),
    (36317, 22843),
    (35867, 22901),
    (35334, 23299),
    (34873, 23086),
    (34239, 22748),
    (33650, 22738),
    (33207, 22556),
    (32518, 22618),
    (31771, 22339),
    (31939, 21188),
    (31181, 21127),
    (30606, 21355),
    (29100, 22076),
    (29363, 23004),
    (28806, 23122),
    (28138, 23996),
    (27649, 23745),
    (27429, 23921),
    (27629, 24175),
    (27629, 25125),
    (28000, 25408),
    (28671, 26621),
    (28587, 27470),
    (28045, 28081),
    (27354, 28166),
    (26904, 28327),
    (26121, 28960),
    (25271, 29017),
    (24334, 28926),
    (23281, 28453),
    (22226, 27822),
    (21337, 28030),
    (20029, 28119),
    (19139, 27623),
    (19070, 27236),
    (18756, 27466),
    (18705, 28053),
    (17126, 27731),
    (15952, 27887),
    (15099, 28309),
    (14162, 28816),
    (13168, 29493),
    (12106, 29990),
    (11660, 30327),
    (10167, 30495),
    (9375, 30938),
    (8486, 31979),
    (8722, 32794),
    (9185, 32621),
    (8621, 33596),
    (8562, 34107),
    (7921, 35363),
    (6477, 35767),
    (5849, 36035),
    (5412, 36632),
    (4429, 36990),
    (4481, 37944),
    (3733, 38541),
    (3418, 39439),
    (4337, 40209),
    (5189, 40554),
    (5814, 40427),
    (6823, 41113),
    (8116, 41473),
    (10034, 42149),
    (10340, 43131),
    (10301, 44076),
    (9809, 44979),
    (12485, 45285),
    (12969, 47301),
    (15508, 48253),
    (16887, 49234),
    (18693, 48278),
    (20080, 47983),
    (21148, 46732),
    (20974, 45311),
    (23520, 45066),
    (25448, 44104),
    (27170, 42895),
    (30878, 42771),
    (34621, 41750),
    (37424, 42552),
    (40362, 42855),
    (41310, 44354),
    (41939, 45178),
    (43616, 44847),
    (46125, 45780),
    (47381, 46673),
    (49805, 46782),
    (48511, 47895),
    (46266, 47773),
    (46663, 49929),
    (49200, 50160),
    (50015, 51753),
    (50854, 53390),
    (53521, 53656),
    (55666, 53163),
    (56637, 52186),
    (57007, 51321),
    (57312, 50751),
    (57694, 49765),
    (58608, 49612),
    (60250, 48886),
    (61052, 47784),
    (64174, 48398),
    (64803, 48393),
    (64683, 48114),
    (64073, 46781),
    (63422, 45560),
    (63167, 45114),
    (61873, 45262),
    (61401, 44055),
    (61062, 42845),
    (60644, 42423),
    (60056, 42962),
    (59716, 42424),
    (58500, 41991),
    (58265, 41678),
    (57349, 41454),
    (56603, 41563),
    (56030, 40885),
    (55416, 40618),
    (54735, 40317),
    (54488, 40170),
    (54400, 40114),
    (54386, 40041),
    (54378, 39985),
    (54152, 39532),
    (53849, 35084),
    (55013, 30505),
    (51553, 26249),
    (50705, 26631),
    (49787, 26205),
    (49894, 25782),
    (50047, 25388),
    (48602, 24461),
    (48442, 24553),
    (48228, 24495),
    (48104, 24362),
    (48198, 24345),
];

const GCJ_INTERIOR_ROWS: [u64; 54] = [
    0x0000000000000000, // lat 00..01
    0x0000000000000000, // lat 01..02
    0x0000000000000000, // lat 02..03
    0x0000000000000000, // lat 03..04
    0x0000000000000000, // lat 04..05
    0x0000000000000000, // lat 05..06
    0x0000000000000000, // lat 06..07
    0x0000000000000000, // lat 07..08
    0x0000000000000000, // lat 08..09
    0x0000000000000000, // lat 09..10
    0x0000000000000000, // lat 10..11
    0x0000000000000000, // lat 11..12
    0x0000000000000000, // lat 12..13
    0x0000000000000000, // lat 13..14
    0x0000000000000000, // lat 14..15
    0x00001f8000000000, // lat 15..16
    0x00001fc000000000, // lat 16..17
    0x00001fc000000000, // lat 17..18
    0x00001fe000000000, // lat 18..19
    0x00001fe000000000, // lat 19..20
    0x00001fc000000000, // lat 20..21
    0x00001fc000000000, // lat 21..22
    0x00001ff810000000, // lat 22..23
    0x00001ffcf0000000, // lat 23..24
    0x00003ffffc000000, // lat 24..25
    0x00007ffff8000000, // lat 25..26
    0x00007ffff8000000, // lat 26..27
    0x0003fffff8000000, // lat 27..28
    0x0003fffff800c000, // lat 28..29
    0x0007ffffff7ff000, // lat 29..30
    0x000ffffffffff800, // lat 30..31
    0x000fffffffffff00, // lat 31..32
    0x000fffffffffff00, // lat 32..33
    0x000fffffffffff80, // lat 33..34
    0x0007ffffffffff80, // lat 34..35
    0x0007ffffffffff80, // lat 35..36
    0x0007fffffffffff0, // lat 36..37
    0x0007fffffffffff8, // lat 37..38
    0x000ffffffffffff8, // lat 38..39
    0x000ffffffffffff8, // lat 39..40
    0x000fffffffffffe0, // lat 40..41
    0x003ffffc7fffff00, // lat 41..42
    0x00ffff8001fffe00, // lat 42..43
    0x07ffff00007ffe00, // lat 43..44
    0x07fff800001ffe00, // lat 44..45
    0x07ffe0000003f800, // lat 45..46
    0x1fff00000003f800, // lat 46..47
    0x23ff00000003e000, // lat 47..48
    0x01ffe00000000000, // lat 48..49
    0x007fc00000000000, // lat 49..50
    0x007f000000000000, // lat 50..51
    0x003e000000000000, // lat 51..52
    0x001e000000000000, // lat 52..53
    0x0000000000000000, // lat 53..54
];

const GCJ_BORDER_ROWS: [u64; 54] = [
    0x0000000000000000, // lat 00..01
    0x0000000000000000, // lat 01..02
    0x0000000000000000, // lat 02..03
    0x0000000000000000, // lat 03..04
    0x0000000000000000, // lat 04..05
    0x0000000000000000, // lat 05..06
    0x0000000000000000, // lat 06..07
    0x0000000000000000, // lat 07..08
    0x0000000000000000, // lat 08..09
    0x0000000000000000, // lat 09..10
    0x0000000000000000, // lat 10..11
    0x0000000000000000, // lat 11..12
    0x0000000000000000, // lat 12..13
    0x0000000000000000, // lat 13..14
    0x00007fc000000000, // lat 14..15
    0x0000606000000000, // lat 15..16
    0x0000202000000000, // lat 16..17
    0x0000203000000000, // lat 17..18
    0x0000201000000000, // lat 18..19
    0x0000201000000000, // lat 19..20
    0x0000203000000000, // lat 20..21
    0x0000203c38000000, // lat 21..22
    0x00002007e8000000, // lat 22..23
    0x000060030e000000, // lat 23..24
    0x0000c00002000000, // lat 24..25
    0x0001800006000000, // lat 25..26
    0x0007800004000000, // lat 26..27
    0x00040000041be000, // lat 27..28
    0x000c000007ff3800, // lat 28..29
    0x0018000000800c00, // lat 29..30
    0x0030000000000780, // lat 30..31
    0x00100000000000c0, // lat 31..32
    0x00100000000000c0, // lat 32..33
    0x0010000000000040, // lat 33..34
    0x0018000000000040, // lat 34..35
    0x0008000000000078, // lat 35..36
    0x000800000000000c, // lat 36..37
    0x0018000000000004, // lat 37..38
    0x0010000000000006, // lat 38..39
    0x0010000000000006, // lat 39..40
    0x007000000000001c, // lat 40..41
    0x01c00003800000f0, // lat 41..42
    0x0f00007efe000180, // lat 42..43
    0x080000c003800100, // lat 43..44
    0x0800078000e00180, // lat 44..45
    0x38001d80003c0780, // lat 45..46
    0x6000f000000c0400, // lat 46..47
    0x5c00f00000041c00, // lat 47..48
    0x760010000007f000, // lat 48..49
    0x038030000000c000, // lat 49..50
    0x0080e00000000000, // lat 50..51
    0x00c1800000000000, // lat 51..52
    0x0061000000000000, // lat 52..53
    0x003f000000000000, // lat 53..54
];

const HONG_KONG_EXCLUSION_RING: [(u16, u16); 14] = [
    (43830, 22540),
    (43920, 22540),
    (44030, 22525),
    (44130, 22530),
    (44220, 22550),
    (44310, 22560),
    (44430, 22500),
    (44470, 22380),
    (44400, 22280),
    (44320, 22200),
    (44180, 22170),
    (44050, 22200),
    (43910, 22180),
    (43840, 22270),
];

const MACAU_EXCLUSION_RING: [(u16, u16); 8] = [
    (43527, 22215),
    (43550, 22222),
    (43575, 22218),
    (43603, 22194),
    (43595, 22145),
    (43584, 22111),
    (43548, 22108),
    (43531, 22151),
];

/// Compatibility no-op retained from versions that lazily initialized a boundary index.
///
/// The current implementation embeds all region data as static constants, so no
/// warm-up is necessary and this function has no runtime effect.
#[inline]
pub fn warm_up() {}

/// Returns `true` when a WGS 84 coordinate belongs to the approximate GCJ-02
/// transform region used by this crate.
///
/// The check is optimized for high-frequency use. Most points are classified by
/// a precomputed one-degree raster; only boundary cells require a polygon test.
/// Hong Kong and Macau are explicitly excluded. The mask is suitable for ordinary
/// application dispatch, not legal or surveying boundary decisions.
///
/// # Examples
///
/// ```
/// use coordtransform::is_in_gcj02_region;
///
/// assert!(is_in_gcj02_region(116.4074, 39.9042));
/// assert!(is_in_gcj02_region(121.4737, 31.2304));
/// assert!(!is_in_gcj02_region(114.1694, 22.3193));
/// assert!(!is_in_gcj02_region(126.9780, 37.5665));
/// ```
#[inline]
pub fn is_in_gcj02_region(lon: f64, lat: f64) -> bool {
    // This comparison form rejects NaN as well as coordinates outside the mask.
    if !(lon >= GCJ_REGION_MIN_LON
        && lon < GCJ_REGION_MAX_LON
        && lat >= GCJ_REGION_MIN_LAT
        && lat < GCJ_REGION_MAX_LAT)
    {
        return false;
    }

    // These small exclusions are checked first because their surrounding raster
    // cells are otherwise well inside the broad China land mask.
    if lon >= 113.80
        && lon <= 114.50
        && lat >= 22.05
        && lat <= 22.60
        && point_in_quantized_ring(lon, lat, &HONG_KONG_EXCLUSION_RING)
    {
        return false;
    }
    if lon >= 113.50
        && lon <= 113.62
        && lat >= 22.08
        && lat <= 22.24
        && point_in_quantized_ring(lon, lat, &MACAU_EXCLUSION_RING)
    {
        return false;
    }

    let row = lat as usize;
    let col = lon as usize - 72;
    let bit = 1_u64 << col;

    if GCJ_BORDER_ROWS[row] & bit != 0 {
        point_in_quantized_ring(lon, lat, &GCJ_REGION_RING)
    } else {
        GCJ_INTERIOR_ROWS[row] & bit != 0
    }
}

/// Returns `true` when a WGS 84 coordinate is inside the approximate mainland
/// GCJ-02 transform region.
///
/// This is a compatibility alias for [`is_in_gcj02_region`].
#[inline]
pub fn is_in_mainland_china(lon: f64, lat: f64) -> bool {
    is_in_gcj02_region(lon, lat)
}

/// Returns `true` when a coordinate is outside this crate's GCJ-02 transform region.
#[inline]
pub fn is_out_of_china(lon: f64, lat: f64) -> bool {
    !is_in_gcj02_region(lon, lat)
}

#[inline]
fn point_in_quantized_ring(lon: f64, lat: f64, ring: &[(u16, u16)]) -> bool {
    let x = (lon - GCJ_RING_LON_ORIGIN) * GCJ_RING_SCALE;
    let y = lat * GCJ_RING_SCALE;
    let mut inside = false;
    let mut previous = ring[ring.len() - 1];

    for &current in ring {
        let xi = current.0 as f64;
        let yi = current.1 as f64;
        let xj = previous.0 as f64;
        let yj = previous.1 as f64;

        if (yi > y) != (yj > y) {
            let crossing_x = (xj - xi) * (y - yi) / (yj - yi) + xi;
            if x < crossing_x {
                inside = !inside;
            }
        }

        previous = current;
    }

    inside
}

/// Converts GCJ-02 longitude/latitude to BD-09 longitude/latitude.
///
/// This is a low-level coordinate-system transform and does not perform a
/// geographic region check.
///
/// # Examples
///
/// ```
/// use coordtransform::gcj02_to_bd09;
///
/// let (lon, lat) = gcj02_to_bd09(116.404, 39.915);
/// assert!((lon - 116.41036949371029).abs() < 1e-12);
/// assert!((lat - 39.92133699351022).abs() < 1e-12);
/// ```
#[inline]
pub fn gcj02_to_bd09(lon: f64, lat: f64) -> (f64, f64) {
    gcj02_to_bd09_unchecked(lon, lat)
}

/// Converts BD-09 longitude/latitude to GCJ-02 longitude/latitude with numerical
/// refinement for tight forward/reverse consistency.
///
/// This is a low-level coordinate-system transform and does not perform a
/// geographic region check. For the traditional single-pass approximation, use
/// [`bd09_to_gcj02_fast`].
///
/// # Examples
///
/// ```
/// use coordtransform::{bd09_to_gcj02, gcj02_to_bd09};
///
/// let original = (116.404, 39.915);
/// let bd = gcj02_to_bd09(original.0, original.1);
/// let restored = bd09_to_gcj02(bd.0, bd.1);
/// assert!((restored.0 - original.0).abs() < 1e-9);
/// assert!((restored.1 - original.1).abs() < 1e-9);
/// ```
#[inline]
pub fn bd09_to_gcj02(lon: f64, lat: f64) -> (f64, f64) {
    let mut gcj = bd09_to_gcj02_approx(lon, lat);

    // Two fixed refinements reduce the inverse residual to far below practical
    // mapping accuracy while keeping runtime predictable for batch workloads.
    for _ in 0..BD_INVERSE_REFINEMENTS {
        let forward = gcj02_to_bd09_unchecked(gcj.0, gcj.1);
        gcj.0 -= forward.0 - lon;
        gcj.1 -= forward.1 - lat;
    }

    gcj
}

/// Converts BD-09 longitude/latitude to GCJ-02 using the traditional single-pass
/// inverse approximation.
///
/// This function is faster and matches the output style of many existing GCJ/BD
/// implementations, but its round-trip residual is larger than [`bd09_to_gcj02`].
#[inline]
pub fn bd09_to_gcj02_fast(lon: f64, lat: f64) -> (f64, f64) {
    bd09_to_gcj02_approx(lon, lat)
}

/// Converts WGS 84 longitude/latitude to GCJ-02 longitude/latitude.
///
/// The GCJ-02 offset is only applied when the WGS 84 position is inside the
/// embedded GCJ-02 transform region. Outside that region, the input is
/// returned unchanged.
///
/// # Examples
///
/// ```
/// use coordtransform::wgs84_to_gcj02;
///
/// let (lon, lat) = wgs84_to_gcj02(116.404, 39.915);
/// assert!((lon - 116.41024449916938).abs() < 1e-12);
/// assert!((lat - 39.91640428150164).abs() < 1e-12);
/// ```
#[inline]
pub fn wgs84_to_gcj02(lon: f64, lat: f64) -> (f64, f64) {
    if !is_in_mainland_china(lon, lat) {
        return (lon, lat);
    }
    wgs84_to_gcj02_unchecked(lon, lat)
}

/// Converts a WGS 84 position known to be in the mainland transform region to
/// GCJ-02 without performing the region lookup.
///
/// Use this only when the caller already guarantees that the coordinate belongs
/// to the transform region. It is useful for high-frequency navigation streams
/// that never leave mainland China.
#[inline]
pub fn wgs84_to_gcj02_in_mainland(lon: f64, lat: f64) -> (f64, f64) {
    wgs84_to_gcj02_unchecked(lon, lat)
}

/// Converts GCJ-02 longitude/latitude to WGS 84 longitude/latitude using a
/// refined numerical inverse.
///
/// A rough WGS 84 candidate is recovered first and used for the embedded
/// region decision. Coordinates outside the mainland transform region are
/// returned unchanged.
///
/// # Examples
///
/// ```
/// use coordtransform::{gcj02_to_wgs84, wgs84_to_gcj02};
///
/// let original = (121.4737, 31.2304);
/// let gcj = wgs84_to_gcj02(original.0, original.1);
/// let restored = gcj02_to_wgs84(gcj.0, gcj.1);
/// assert!((restored.0 - original.0).abs() < 1e-9);
/// assert!((restored.1 - original.1).abs() < 1e-9);
/// ```
#[inline]
pub fn gcj02_to_wgs84(lon: f64, lat: f64) -> (f64, f64) {
    gcj02_to_wgs84_if_mainland(lon, lat, GCJ_INVERSE_REFINEMENTS).unwrap_or((lon, lat))
}

/// Converts GCJ-02 longitude/latitude to WGS 84 using only the traditional
/// one-step inverse approximation after the region check.
///
/// This is useful when throughput matters more than sub-metre inverse accuracy.
/// For the high-accuracy default, use [`gcj02_to_wgs84`].
#[inline]
pub fn gcj02_to_wgs84_fast(lon: f64, lat: f64) -> (f64, f64) {
    gcj02_to_wgs84_if_mainland(lon, lat, 0).unwrap_or((lon, lat))
}

/// Converts a GCJ-02 position known to belong to the mainland transform region
/// to WGS 84 without performing the region lookup.
///
/// This uses the same refined inverse as [`gcj02_to_wgs84`].
#[inline]
pub fn gcj02_to_wgs84_in_mainland(lon: f64, lat: f64) -> (f64, f64) {
    let seed = gcj02_inverse_seed(lon, lat);
    refine_gcj_inverse(lon, lat, seed, GCJ_INVERSE_REFINEMENTS)
}

/// Converts WGS 84 longitude/latitude to BD-09 longitude/latitude.
///
/// The transform is only applied inside the embedded GCJ-02 transform
/// region. Outside that region, the input is returned unchanged.
#[inline]
pub fn wgs84_to_bd09(lon: f64, lat: f64) -> (f64, f64) {
    if !is_in_mainland_china(lon, lat) {
        return (lon, lat);
    }

    let gcj = wgs84_to_gcj02_unchecked(lon, lat);
    gcj02_to_bd09_unchecked(gcj.0, gcj.1)
}

/// Converts BD-09 longitude/latitude to WGS 84 longitude/latitude.
///
/// The BD-09 coordinate is first converted to GCJ-02, then a WGS 84 candidate
/// is recovered and checked against the embedded GCJ-02 transform region.
/// Outside that region, the original BD-09 input is returned unchanged.
#[inline]
pub fn bd09_to_wgs84(lon: f64, lat: f64) -> (f64, f64) {
    if !lon.is_finite() || !lat.is_finite() {
        return (lon, lat);
    }

    let gcj = bd09_to_gcj02(lon, lat);
    match gcj02_to_wgs84_if_mainland(gcj.0, gcj.1, GCJ_INVERSE_REFINEMENTS) {
        Some(wgs) => wgs,
        None => (lon, lat),
    }
}

/// Converts WGS 84 longitude/latitude to Web Mercator (EPSG:3857) metres.
///
/// Latitude is clamped to [`MAX_LATITUDE`], matching common slippy-map behavior.
/// Longitude is not clamped so callers can intentionally use wrapped worlds.
/// For strict validation, use [`try_wgs84_to_epsg3857`].
#[inline]
pub fn wgs84_to_epsg3857(lon: f64, lat: f64) -> (f64, f64) {
    let lat = lat.clamp(-MAX_LATITUDE, MAX_LATITUDE);
    web_mercator_forward(lon, lat)
}

/// Converts Web Mercator (EPSG:3857) metres to WGS 84 longitude/latitude.
///
/// Northing is clamped to the canonical finite Web Mercator world extent.
/// Easting is not clamped so wrapped worlds remain representable. For strict
/// validation, use [`try_epsg3857_to_wgs84`].
#[inline]
pub fn epsg3857_to_wgs84(x: f64, y: f64) -> (f64, f64) {
    let y = y.clamp(-WEB_MERCATOR_MAX_COORDINATE, WEB_MERCATOR_MAX_COORDINATE);
    web_mercator_inverse(x, y)
}

/// Converts GCJ-02 longitude/latitude to Web Mercator metres through WGS 84.
#[inline]
pub fn gcj02_to_epsg3857(lon: f64, lat: f64) -> (f64, f64) {
    let wgs = gcj02_to_wgs84(lon, lat);
    wgs84_to_epsg3857(wgs.0, wgs.1)
}

/// Converts Web Mercator metres to GCJ-02 longitude/latitude through WGS 84.
#[inline]
pub fn epsg3857_to_gcj02(x: f64, y: f64) -> (f64, f64) {
    let wgs = epsg3857_to_wgs84(x, y);
    wgs84_to_gcj02(wgs.0, wgs.1)
}

/// Converts BD-09 longitude/latitude to Web Mercator metres through WGS 84.
#[inline]
pub fn bd09_to_epsg3857(lon: f64, lat: f64) -> (f64, f64) {
    let wgs = bd09_to_wgs84(lon, lat);
    wgs84_to_epsg3857(wgs.0, wgs.1)
}

/// Converts Web Mercator metres to BD-09 longitude/latitude through WGS 84.
#[inline]
pub fn epsg3857_to_bd09(x: f64, y: f64) -> (f64, f64) {
    let wgs = epsg3857_to_wgs84(x, y);
    wgs84_to_bd09(wgs.0, wgs.1)
}

/// Errors returned by strict Web Mercator conversion functions.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum ProjectionError {
    /// At least one supplied coordinate is not finite.
    NonFiniteCoordinate,
    /// Longitude is outside `[-180, 180]` degrees.
    LongitudeOutOfRange {
        /// The invalid longitude.
        longitude: f64,
    },
    /// Latitude is outside `[-90, 90]` degrees.
    LatitudeOutOfRange {
        /// The invalid latitude.
        latitude: f64,
    },
    /// Latitude is valid WGS 84 latitude but outside the finite Web Mercator domain.
    LatitudeOutsideWebMercator {
        /// The unsupported latitude.
        latitude: f64,
    },
    /// Easting is outside the canonical finite Web Mercator world extent.
    XOutsideWebMercator {
        /// The unsupported easting in metres.
        x: f64,
    },
    /// Northing is outside the canonical finite Web Mercator world extent.
    YOutsideWebMercator {
        /// The unsupported northing in metres.
        y: f64,
    },
}

impl fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteCoordinate => write!(f, "coordinate must be finite"),
            Self::LongitudeOutOfRange { longitude } => {
                write!(f, "longitude {longitude} is outside [-180, 180] degrees")
            }
            Self::LatitudeOutOfRange { latitude } => {
                write!(f, "latitude {latitude} is outside [-90, 90] degrees")
            }
            Self::LatitudeOutsideWebMercator { latitude } => write!(
                f,
                "latitude {latitude} is outside the finite EPSG:3857 latitude domain"
            ),
            Self::XOutsideWebMercator { x } => write!(
                f,
                "Web Mercator x coordinate {x} is outside the canonical world extent"
            ),
            Self::YOutsideWebMercator { y } => write!(
                f,
                "Web Mercator y coordinate {y} is outside the canonical world extent"
            ),
        }
    }
}

impl std::error::Error for ProjectionError {}

/// Strictly converts WGS 84 longitude/latitude to Web Mercator metres.
///
/// Unlike [`wgs84_to_epsg3857`], this function never clamps. It rejects
/// non-finite coordinates, longitude outside `[-180, 180]`, latitude outside
/// `[-90, 90]`, and latitude outside the finite EPSG:3857 domain.
///
/// # Errors
///
/// Returns [`ProjectionError`] when the input is outside the accepted domain.
#[inline]
pub fn try_wgs84_to_epsg3857(lon: f64, lat: f64) -> Result<(f64, f64), ProjectionError> {
    validate_wgs84(lon, lat)?;
    if lat.abs() > MAX_LATITUDE {
        return Err(ProjectionError::LatitudeOutsideWebMercator { latitude: lat });
    }
    Ok(web_mercator_forward(lon, lat))
}

/// Validates a WGS 84 coordinate and converts it to Web Mercator with explicit
/// latitude clamping.
///
/// This preserves the validated clamping behavior used by earlier versions while
/// keeping the compatibility [`wgs84_to_epsg3857`] and strict
/// [`try_wgs84_to_epsg3857`] APIs separate.
///
/// # Errors
///
/// Returns [`ProjectionError`] for non-finite coordinates or longitude/latitude
/// outside the WGS 84 degree ranges.
#[inline]
pub fn wgs84_to_epsg3857_clamped(lon: f64, lat: f64) -> Result<(f64, f64), ProjectionError> {
    validate_wgs84(lon, lat)?;
    Ok(web_mercator_forward(
        lon,
        lat.clamp(-MAX_LATITUDE, MAX_LATITUDE),
    ))
}

/// Strictly converts Web Mercator metres to WGS 84 longitude/latitude.
///
/// # Errors
///
/// Returns [`ProjectionError`] for non-finite values or coordinates outside the
/// canonical finite EPSG:3857 world extent.
#[inline]
pub fn try_epsg3857_to_wgs84(x: f64, y: f64) -> Result<(f64, f64), ProjectionError> {
    if !x.is_finite() || !y.is_finite() {
        return Err(ProjectionError::NonFiniteCoordinate);
    }
    if x.abs() > WEB_MERCATOR_MAX_COORDINATE {
        return Err(ProjectionError::XOutsideWebMercator { x });
    }
    if y.abs() > WEB_MERCATOR_MAX_COORDINATE {
        return Err(ProjectionError::YOutsideWebMercator { y });
    }
    Ok(web_mercator_inverse(x, y))
}

/// Converts a mutable slice of WGS 84 points to GCJ-02 in place.
///
/// No allocation is performed. Each point still receives the normal OSM-derived
/// transform-region check.
#[inline]
pub fn wgs84_to_gcj02_in_place(points: &mut [(f64, f64)]) {
    for point in points {
        *point = wgs84_to_gcj02(point.0, point.1);
    }
}

/// Converts a mutable slice of GCJ-02 points to WGS 84 in place.
#[inline]
pub fn gcj02_to_wgs84_in_place(points: &mut [(f64, f64)]) {
    for point in points {
        *point = gcj02_to_wgs84(point.0, point.1);
    }
}

/// Converts WGS 84 points known to be inside the mainland transform region to
/// GCJ-02 in place without running region lookup for each point.
///
/// This is the highest-throughput batch path for tracks that are already known
/// to stay inside the transform region. No allocation is performed.
#[inline]
pub fn wgs84_to_gcj02_in_mainland_in_place(points: &mut [(f64, f64)]) {
    for point in points {
        *point = wgs84_to_gcj02_unchecked(point.0, point.1);
    }
}

/// Converts GCJ-02 points known to belong to the mainland transform region to
/// WGS 84 in place without running region lookup for each point.
///
/// The same refined inverse as [`gcj02_to_wgs84`] is used. No allocation is
/// performed.
#[inline]
pub fn gcj02_to_wgs84_in_mainland_in_place(points: &mut [(f64, f64)]) {
    for point in points {
        *point = gcj02_to_wgs84_in_mainland(point.0, point.1);
    }
}

/// Converts a mutable slice of WGS 84 points to BD-09 in place.
#[inline]
pub fn wgs84_to_bd09_in_place(points: &mut [(f64, f64)]) {
    for point in points {
        *point = wgs84_to_bd09(point.0, point.1);
    }
}

/// Converts a mutable slice of BD-09 points to WGS 84 in place.
#[inline]
pub fn bd09_to_wgs84_in_place(points: &mut [(f64, f64)]) {
    for point in points {
        *point = bd09_to_wgs84(point.0, point.1);
    }
}

#[inline]
fn gcj02_to_bd09_unchecked(lon: f64, lat: f64) -> (f64, f64) {
    let z = (lon * lon + lat * lat).sqrt() + BD_RADIAL_EPSILON * (lat * X_PI).sin();
    let theta = lat.atan2(lon) + BD_ANGULAR_EPSILON * (lon * X_PI).cos();
    let (sin_theta, cos_theta) = theta.sin_cos();

    (z * cos_theta + BD_LON_OFFSET, z * sin_theta + BD_LAT_OFFSET)
}

#[inline]
fn bd09_to_gcj02_approx(lon: f64, lat: f64) -> (f64, f64) {
    let x = lon - BD_LON_OFFSET;
    let y = lat - BD_LAT_OFFSET;
    let z = (x * x + y * y).sqrt() - BD_RADIAL_EPSILON * (y * X_PI).sin();
    let theta = y.atan2(x) - BD_ANGULAR_EPSILON * (x * X_PI).cos();
    let (sin_theta, cos_theta) = theta.sin_cos();

    (z * cos_theta, z * sin_theta)
}

#[inline]
fn gcj02_to_wgs84_if_mainland(
    lon: f64,
    lat: f64,
    refinements_after_seed: usize,
) -> Option<(f64, f64)> {
    if !lon.is_finite() || !lat.is_finite() {
        return None;
    }

    let seed = gcj02_inverse_seed(lon, lat);

    // Near the transform boundary, either the observed GCJ-02 coordinate or the
    // recovered WGS 84 seed can fall on the opposite side by a small offset. The
    // two-way check makes dispatch more stable without adding a second lookup for
    // ordinary interior points.
    if !is_in_mainland_china(seed.0, seed.1) && !is_in_mainland_china(lon, lat) {
        return None;
    }

    Some(refine_gcj_inverse(lon, lat, seed, refinements_after_seed))
}

#[inline]
fn gcj02_inverse_seed(lon: f64, lat: f64) -> (f64, f64) {
    let forward = wgs84_to_gcj02_unchecked(lon, lat);
    (lon * 2.0 - forward.0, lat * 2.0 - forward.1)
}

#[inline]
fn refine_gcj_inverse(
    gcj_lon: f64,
    gcj_lat: f64,
    mut wgs: (f64, f64),
    refinements: usize,
) -> (f64, f64) {
    for _ in 0..refinements {
        let forward = wgs84_to_gcj02_unchecked(wgs.0, wgs.1);
        wgs.0 -= forward.0 - gcj_lon;
        wgs.1 -= forward.1 - gcj_lat;
    }
    wgs
}

#[inline]
fn wgs84_to_gcj02_unchecked(lon: f64, lat: f64) -> (f64, f64) {
    let x = lon - 105.0;
    let y = lat - 35.0;
    let (raw_lat, raw_lon) = transform_delta(x, y);

    let rad_lat = lat * DEG_TO_RAD;
    let (sin_lat, cos_lat) = rad_lat.sin_cos();
    let magic = 1.0 - GCJ_ECCENTRICITY_SQUARED * sin_lat * sin_lat;
    let sqrt_magic = magic.sqrt();

    let delta_lat =
        raw_lat * GCJ_DEGREE_SCALE * magic * sqrt_magic / (1.0 - GCJ_ECCENTRICITY_SQUARED);
    let delta_lon = raw_lon * GCJ_DEGREE_SCALE * sqrt_magic / cos_lat;

    (lon + delta_lon, lat + delta_lat)
}

#[inline]
fn transform_delta(x: f64, y: f64) -> (f64, f64) {
    let xy = x * y;
    let sqrt_abs_x = x.abs().sqrt();
    let x_pi = x * PI;
    let y_pi = y * PI;

    // One sin_cos at x*pi/3 yields the x*pi, 2*x*pi, and 6*x*pi terms
    // through angle-multiplication identities.
    let (sin_x_third, cos_x_third) = (x_pi / 3.0).sin_cos();
    let sin_x = sin_triple(sin_x_third);
    let cos_x = cos_triple(cos_x_third);
    let sin_2x = 2.0 * sin_x * cos_x;
    let sin_6x = sin_triple(sin_2x);
    let shared = 20.0 * sin_6x + 20.0 * sin_2x;

    // The y*pi term is the triple-angle value of y*pi/3.
    let sin_y_third = (y_pi / 3.0).sin();
    let sin_y = sin_triple(sin_y_third);

    // Using a base angle of pi/60 gives both /30 and /12 terms from one
    // sin_cos call for each axis.
    let (sin_x_thirtieth, sin_x_twelfth) = sin_double_and_quintuple(x_pi / 60.0);
    let (sin_y_thirtieth, sin_y_twelfth) = sin_double_and_quintuple(y_pi / 60.0);

    let delta_lat = -100.0
        + 2.0 * x
        + 3.0 * y
        + 0.2 * y * y
        + 0.1 * xy
        + 0.2 * sqrt_abs_x
        + (shared
            + 20.0 * sin_y
            + 40.0 * sin_y_third
            + 160.0 * sin_y_twelfth
            + 320.0 * sin_y_thirtieth)
            * (2.0 / 3.0);

    let delta_lon = 300.0
        + x
        + 2.0 * y
        + 0.1 * x * x
        + 0.1 * xy
        + 0.1 * sqrt_abs_x
        + (shared
            + 20.0 * sin_x
            + 40.0 * sin_x_third
            + 150.0 * sin_x_twelfth
            + 300.0 * sin_x_thirtieth)
            * (2.0 / 3.0);

    (delta_lat, delta_lon)
}

#[inline]
fn sin_triple(sin_angle: f64) -> f64 {
    sin_angle * (3.0 - 4.0 * sin_angle * sin_angle)
}

#[inline]
fn cos_triple(cos_angle: f64) -> f64 {
    cos_angle * (4.0 * cos_angle * cos_angle - 3.0)
}

#[inline]
fn sin_double_and_quintuple(angle: f64) -> (f64, f64) {
    let (sin_a, cos_a) = angle.sin_cos();
    let sin_2a = 2.0 * sin_a * cos_a;
    let cos_2a = cos_a * cos_a - sin_a * sin_a;
    let sin_4a = 2.0 * sin_2a * cos_2a;
    let cos_4a = cos_2a * cos_2a - sin_2a * sin_2a;
    let sin_5a = sin_4a * cos_a + cos_4a * sin_a;
    (sin_2a, sin_5a)
}

#[inline]
fn validate_wgs84(lon: f64, lat: f64) -> Result<(), ProjectionError> {
    if !lon.is_finite() || !lat.is_finite() {
        return Err(ProjectionError::NonFiniteCoordinate);
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(ProjectionError::LongitudeOutOfRange { longitude: lon });
    }
    if !(-90.0..=90.0).contains(&lat) {
        return Err(ProjectionError::LatitudeOutOfRange { latitude: lat });
    }
    Ok(())
}

#[inline]
fn web_mercator_forward(lon: f64, lat: f64) -> (f64, f64) {
    let lon_rad = lon * DEG_TO_RAD;
    let lat_rad = lat * DEG_TO_RAD;
    let x = lon_rad * WEB_MERCATOR_RADIUS;
    let y = lat_rad.tan().asinh() * WEB_MERCATOR_RADIUS;
    (x, y)
}

#[inline]
fn web_mercator_inverse(x: f64, y: f64) -> (f64, f64) {
    let lon = x / WEB_MERCATOR_RADIUS * RAD_TO_DEG;
    let lat = (y / WEB_MERCATOR_RADIUS).sinh().atan() * RAD_TO_DEG;
    (lon, lat)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual:.15}, expected={expected:.15}, tolerance={tolerance:e}"
        );
    }

    fn assert_point_close(actual: (f64, f64), expected: (f64, f64), tolerance: f64) {
        assert_close(actual.0, expected.0, tolerance);
        assert_close(actual.1, expected.1, tolerance);
    }

    #[test]
    fn mainland_region_accepts_representative_locations() {
        let inside = [
            (116.4074, 39.9042), // Beijing
            (121.4737, 31.2304), // Shanghai
            (106.5516, 29.5630), // Chongqing
            (111.7490, 40.8426), // Hohhot, Inner Mongolia
            (113.2644, 23.1291), // Guangzhou
            (87.6168, 43.8256),  // Urumqi
            (91.1322, 29.6604),  // Lhasa
            (109.5119, 18.2528), // Sanya
        ];

        for point in inside {
            assert!(
                is_in_gcj02_region(point.0, point.1),
                "expected mainland transform region: {point:?}"
            );
        }
    }

    #[test]
    fn mainland_region_accepts_border_area_chinese_cities() {
        let inside = [
            (124.3947, 40.1253), // Dandong
            (97.8550, 24.0128),  // Ruili
            (127.4990, 50.2496), // Heihe
            (80.4208, 44.2017),  // Khorgos
            (107.9718, 21.5478), // Dongxing
            (114.0579, 22.5431), // Shenzhen
            (113.5767, 22.2707), // Zhuhai
            (118.0894, 24.4798), // Xiamen
        ];

        for point in inside {
            assert!(
                is_in_gcj02_region(point.0, point.1),
                "expected mainland transform region near boundary: {point:?}"
            );
        }
    }

    #[test]
    fn mainland_region_rejects_special_regions_and_nearby_countries() {
        let outside = [
            (114.1694, 22.3193), // Hong Kong
            (113.5439, 22.1987), // Macau
            (121.5654, 25.0330), // Taipei
            (139.6917, 35.6895), // Tokyo
            (126.9780, 37.5665), // Seoul
            (106.9057, 47.8864), // Ulaanbaatar
            (85.3240, 27.7172),  // Kathmandu
            (105.8342, 21.0278), // Hanoi
            (103.8500, 22.4800), // Lao Cai, Vietnam
            (124.4072, 40.1028), // Sinuiju, North Korea
            (112.3000, 16.5000), // Remote offshore point below the app dispatch mask
            (172.6362, -43.5321),
        ];

        for point in outside {
            assert!(
                !is_in_gcj02_region(point.0, point.1),
                "expected outside transform region: {point:?}"
            );
        }
    }

    #[test]
    fn hong_kong_and_macau_exclusions_do_not_swallow_nearby_mainland_centres() {
        assert!(!is_in_gcj02_region(114.1694, 22.3193)); // Hong Kong
        assert!(!is_in_gcj02_region(113.5439, 22.1987)); // Macau
        assert!(is_in_gcj02_region(114.0579, 22.5431)); // Shenzhen
        assert!(is_in_gcj02_region(113.5767, 22.2707)); // Zhuhai
    }

    #[test]
    fn invalid_region_inputs_are_rejected() {
        assert!(!is_in_gcj02_region(f64::NAN, 30.0));
        assert!(!is_in_gcj02_region(110.0, f64::NAN));
        assert!(!is_in_gcj02_region(f64::INFINITY, 30.0));
        assert!(!is_in_gcj02_region(110.0, f64::NEG_INFINITY));
    }

    fn reference_wgs84_to_gcj02_unchecked(lon: f64, lat: f64) -> (f64, f64) {
        let x = lon - 105.0;
        let y = lat - 35.0;
        let xy = x * y;
        let sqrt_abs_x = x.abs().sqrt();
        let x_pi = x * PI;
        let y_pi = y * PI;

        let shared = 20.0 * (6.0 * x_pi).sin() + 20.0 * (2.0 * x_pi).sin();

        let raw_lat = -100.0
            + 2.0 * x
            + 3.0 * y
            + 0.2 * y * y
            + 0.1 * xy
            + 0.2 * sqrt_abs_x
            + (shared
                + 20.0 * y_pi.sin()
                + 40.0 * (y_pi / 3.0).sin()
                + 160.0 * (y_pi / 12.0).sin()
                + 320.0 * (y_pi / 30.0).sin())
                * (2.0 / 3.0);

        let raw_lon = 300.0
            + x
            + 2.0 * y
            + 0.1 * x * x
            + 0.1 * xy
            + 0.1 * sqrt_abs_x
            + (shared
                + 20.0 * x_pi.sin()
                + 40.0 * (x_pi / 3.0).sin()
                + 150.0 * (x_pi / 12.0).sin()
                + 300.0 * (x_pi / 30.0).sin())
                * (2.0 / 3.0);

        let rad_lat = lat * DEG_TO_RAD;
        let sin_lat = rad_lat.sin();
        let magic = 1.0 - GCJ_ECCENTRICITY_SQUARED * sin_lat * sin_lat;
        let sqrt_magic = magic.sqrt();

        let delta_lat =
            raw_lat * GCJ_DEGREE_SCALE * magic * sqrt_magic / (1.0 - GCJ_ECCENTRICITY_SQUARED);
        let delta_lon = raw_lon * GCJ_DEGREE_SCALE * sqrt_magic / rad_lat.cos();

        (lon + delta_lon, lat + delta_lat)
    }

    #[test]
    fn optimized_gcj_forward_matches_straightforward_reference_formula() {
        let longitudes = [74.0, 87.6, 104.1, 113.3, 116.4, 121.5, 126.6, 134.0];
        let latitudes = [18.3, 23.1, 29.6, 31.2, 39.9, 43.8, 45.8, 52.0];

        for lon in longitudes {
            for lat in latitudes {
                let optimized = wgs84_to_gcj02_unchecked(lon, lat);
                let reference = reference_wgs84_to_gcj02_unchecked(lon, lat);
                assert_point_close(optimized, reference, 1e-12);
            }
        }
    }

    #[test]
    fn beijing_wgs84_to_gcj02_matches_reference_formula() {
        let actual = wgs84_to_gcj02(116.404, 39.915);
        assert_point_close(
            actual,
            (116.410_244_499_169_38, 39.916_404_281_501_64),
            1e-12,
        );
    }

    #[test]
    fn shanghai_wgs84_to_gcj02_is_not_accidentally_identity() {
        let original = (121.4737, 31.2304);
        let converted = wgs84_to_gcj02(original.0, original.1);
        assert_ne!(converted, original);
        assert_point_close(
            converted,
            (121.478_223_059_276_93, 31.228_457_737_577_27),
            1e-12,
        );
    }

    #[test]
    fn chongqing_and_inner_mongolia_are_not_accidentally_identity() {
        for original in [(106.5516, 29.5630), (111.7490, 40.8426)] {
            assert_ne!(
                wgs84_to_gcj02(original.0, original.1),
                original,
                "point={original:?}"
            );
        }
    }

    #[test]
    fn gcj_wgs_round_trip_is_tight_across_mainland() {
        let samples = [
            (116.4040, 39.9150),
            (121.4737, 31.2304),
            (106.5516, 29.5630),
            (113.2644, 23.1291),
            (104.0665, 30.5728),
            (87.6168, 43.8256),
            (126.6424, 45.7567),
            (109.5119, 18.2528),
        ];

        for original in samples {
            let gcj = wgs84_to_gcj02(original.0, original.1);
            let restored = gcj02_to_wgs84(gcj.0, gcj.1);
            assert_point_close(restored, original, 5e-10);
        }
    }

    #[test]
    fn refined_gcj_inverse_is_more_accurate_than_fast_inverse() {
        let original = (116.404, 39.915);
        let gcj = wgs84_to_gcj02(original.0, original.1);
        let fast = gcj02_to_wgs84_fast(gcj.0, gcj.1);
        let precise = gcj02_to_wgs84(gcj.0, gcj.1);

        let fast_error = (fast.0 - original.0).abs().max((fast.1 - original.1).abs());
        let precise_error = (precise.0 - original.0)
            .abs()
            .max((precise.1 - original.1).abs());

        assert!(precise_error < fast_error);
        assert!(precise_error < 5e-10);
    }

    #[test]
    fn gcj_bd_reference_forward_value_is_preserved() {
        let actual = gcj02_to_bd09(116.404, 39.915);
        assert_point_close(
            actual,
            (116.410_369_493_710_29, 39.921_336_993_510_22),
            1e-12,
        );
    }

    #[test]
    fn fast_bd_inverse_preserves_traditional_reference_value() {
        let actual = bd09_to_gcj02_fast(116.404, 39.915);
        assert_point_close(
            actual,
            (116.397_627_291_193_15, 39.908_656_739_576_31),
            1e-12,
        );
    }

    #[test]
    fn refined_bd_inverse_round_trip_is_tight() {
        let samples = [
            (116.4040, 39.9150),
            (121.4737, 31.2304),
            (113.2644, 23.1291),
            (104.0665, 30.5728),
            (87.6168, 43.8256),
        ];

        for original in samples {
            let bd = gcj02_to_bd09(original.0, original.1);
            let restored = bd09_to_gcj02(bd.0, bd.1);
            assert_point_close(restored, original, 5e-9);
        }
    }

    #[test]
    fn refined_bd_inverse_beats_single_pass_approximation() {
        let original = (113.2644, 23.1291);
        let bd = gcj02_to_bd09(original.0, original.1);
        let fast = bd09_to_gcj02_fast(bd.0, bd.1);
        let precise = bd09_to_gcj02(bd.0, bd.1);

        let fast_error = (fast.0 - original.0).abs().max((fast.1 - original.1).abs());
        let precise_error = (precise.0 - original.0)
            .abs()
            .max((precise.1 - original.1).abs());

        assert!(precise_error < fast_error);
        assert!(precise_error < 5e-9);
    }

    #[test]
    fn wgs_bd_round_trip_is_tight_inside_region() {
        let samples = [
            (116.4040, 39.9150),
            (121.4737, 31.2304),
            (113.2644, 23.1291),
            (104.0665, 30.5728),
        ];

        for original in samples {
            let bd = wgs84_to_bd09(original.0, original.1);
            let restored = bd09_to_wgs84(bd.0, bd.1);
            assert_point_close(restored, original, 5e-9);
        }
    }

    #[test]
    fn guarded_china_transforms_are_identity_outside_region() {
        let samples = [
            (0.0, 0.0),
            (-73.9857, 40.7484),
            (2.3522, 48.8566),
            (139.6917, 35.6895),
            (114.1694, 22.3193),
            (113.5439, 22.1987),
            (121.5654, 25.0330),
        ];

        for original in samples {
            assert_eq!(wgs84_to_gcj02(original.0, original.1), original);
            assert_eq!(gcj02_to_wgs84(original.0, original.1), original);
            assert_eq!(wgs84_to_bd09(original.0, original.1), original);
            assert_eq!(bd09_to_wgs84(original.0, original.1), original);
        }
    }

    #[test]
    fn direct_gcj_bd_functions_remain_low_level_math_transforms() {
        let outside = (2.3522, 48.8566);
        assert_ne!(gcj02_to_bd09(outside.0, outside.1), outside);
        assert_ne!(bd09_to_gcj02(outside.0, outside.1), outside);
    }

    #[test]
    fn web_mercator_known_value_matches_reference() {
        let (x, y) = wgs84_to_epsg3857(116.404, 39.915);
        assert_close(x, 12_958_034.006_300_215, 1e-6);
        assert_close(y, 4_853_597.988_299_838, 1e-6);
    }

    #[test]
    fn web_mercator_round_trip_is_tight_worldwide() {
        let samples = [
            (0.0, 0.0),
            (116.404, 39.915),
            (-73.9857, 40.7484),
            (151.2093, -33.8688),
            (172.6362, -43.5321),
            (-0.1276, 51.5072),
        ];

        for original in samples {
            let projected = wgs84_to_epsg3857(original.0, original.1);
            let restored = epsg3857_to_wgs84(projected.0, projected.1);
            assert_point_close(restored, original, 1e-12);
        }
    }

    #[test]
    fn web_mercator_clamps_only_in_compatibility_api() {
        let (_, north) = wgs84_to_epsg3857(0.0, 90.0);
        assert_close(north, WEB_MERCATOR_MAX_COORDINATE, 1e-6);

        assert!(matches!(
            try_wgs84_to_epsg3857(0.0, 90.0),
            Err(ProjectionError::LatitudeOutsideWebMercator { .. })
        ));
    }

    #[test]
    fn strict_web_mercator_rejects_invalid_inputs() {
        assert!(matches!(
            try_wgs84_to_epsg3857(181.0, 0.0),
            Err(ProjectionError::LongitudeOutOfRange { .. })
        ));
        assert!(matches!(
            try_wgs84_to_epsg3857(0.0, f64::NAN),
            Err(ProjectionError::NonFiniteCoordinate)
        ));
        assert!(matches!(
            try_epsg3857_to_wgs84(WEB_MERCATOR_MAX_COORDINATE + 1.0, 0.0),
            Err(ProjectionError::XOutsideWebMercator { .. })
        ));
        assert!(matches!(
            try_epsg3857_to_wgs84(0.0, WEB_MERCATOR_MAX_COORDINATE + 1.0),
            Err(ProjectionError::YOutsideWebMercator { .. })
        ));
    }

    #[test]
    fn strict_and_compatibility_mercator_match_for_valid_inputs() {
        let samples = [
            (0.0, 0.0),
            (116.404, 39.915),
            (-73.9857, 40.7484),
            (151.2093, -33.8688),
        ];

        for point in samples {
            assert_eq!(
                try_wgs84_to_epsg3857(point.0, point.1).unwrap(),
                wgs84_to_epsg3857(point.0, point.1)
            );
        }
    }

    #[test]
    fn in_place_gcj_batch_matches_scalar_conversion() {
        let original = [(116.404, 39.915), (121.4737, 31.2304), (172.6362, -43.5321)];
        let expected = original.map(|point| wgs84_to_gcj02(point.0, point.1));
        let mut actual = original;
        wgs84_to_gcj02_in_place(&mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn in_place_bd_batch_round_trips() {
        let original = [(116.404, 39.915), (121.4737, 31.2304), (172.6362, -43.5321)];
        let mut points = original;
        wgs84_to_bd09_in_place(&mut points);
        bd09_to_wgs84_in_place(&mut points);

        for (actual, expected) in points.into_iter().zip(original) {
            assert_point_close(actual, expected, 5e-9);
        }
    }

    #[test]
    fn mainland_in_place_fast_path_matches_scalar_fast_path() {
        let original = [(116.404, 39.915), (121.4737, 31.2304), (113.2644, 23.1291)];

        let mut gcj = original;
        wgs84_to_gcj02_in_mainland_in_place(&mut gcj);
        for (actual, point) in gcj.into_iter().zip(original) {
            assert_eq!(actual, wgs84_to_gcj02_in_mainland(point.0, point.1));
        }

        let mut restored = original.map(|point| wgs84_to_gcj02_in_mainland(point.0, point.1));
        gcj02_to_wgs84_in_mainland_in_place(&mut restored);
        for (actual, expected) in restored.into_iter().zip(original) {
            assert_point_close(actual, expected, 5e-10);
        }
    }

    #[test]
    fn mainland_fast_path_matches_guarded_path_for_known_inside_points() {
        for point in [(116.404, 39.915), (121.4737, 31.2304), (113.2644, 23.1291)] {
            assert_eq!(
                wgs84_to_gcj02_in_mainland(point.0, point.1),
                wgs84_to_gcj02(point.0, point.1)
            );
        }
    }
}
