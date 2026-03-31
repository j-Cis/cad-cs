// use crate::libs::angle::Angle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dXy {
    pub x: f64,
    pub y: f64,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dXyPolar {
    pub r_xy: f64,
    pub f_yx: f64, // w radianach
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dXyCylindricalZ {
    pub r_xy: f64,
    pub f_yx: f64, // w radianach
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dXz {
    pub x: f64,
    pub z: f64,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dXzPolar {
    pub r_xz: f64,
    pub f_zx: f64, // w radianach
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dXzCylindricalY {
    pub r_xz: f64,
    pub f_zx: f64, // w radianach
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dYz {
    pub y: f64,
    pub z: f64,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dYzPolar {
    pub r_yz: f64,
    pub f_zy: f64, // w radianach
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dYzCylindricalX {
    pub r_yz: f64,
    pub f_zy: f64, // w radianach
    pub x: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dXyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dXyzSpherical {
    pub r_xyz: f64,
    pub f_yx: f64, // w radianach
    pub t_zr: f64, // w radianach
}

/*
 * // NARAZIE POMIJAMY
 * #[derive(Debug, Clone, Copy, PartialEq)]
 * pub struct CoordsDddNz90Ex0 {
 *     pub sn_lat: Angle, // Szerokość (-90.0 do 90.0)
 *     pub we_lon: Angle, // Długość (-180.0 do 180.0)
 *     //pub alt: f64,    // Wysokość nad poziomem morza (m)
 * }
 *
 */

// WYMAGA Cs<6> i Cs<7> aby w przyszłości ułatwić operacje
/// Współrzędne w formacie DMS (Stopnie, Minuty, Sekundy).
/// Orientacja Nz90Ex0:
/// - Oś Z: 90°N 0°E
/// - Oś X: 0°N 0°E (Greenwich)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsDmsNz90Ex0 {
    pub sn_lat_d: i8,  // Szerokość stopnie (-90 do 90)
    pub sn_lat_m: u8,  // Szerokość minuty (0 do 59)
    pub sn_lat_s: f32, // Szerokość sekundy (0.0 do 59.99999)
    pub we_lon_d: i16, // Długość stopnie (-180 do 180)
    pub we_lon_m: u8,  // Długość minuty (0 do 59)
    pub we_lon_s: f32, // Długość sekundy (0.0 do 59.99999)
                       //pub alt: f64,    // Wysokość nad poziomem morza (m)
}
