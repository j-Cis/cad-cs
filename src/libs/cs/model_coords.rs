// 📃 ./src/libs/cs/model_coords.rs

use crate::libs::angle::Angle;

/// 📚 【 POL】: Współrzędne 2D w układzie kartezjańskim.
/// 📚 【 ENG】: 2D coordinates in the Cartesian system.
/// 💡 Może reprezentować dowolną płaszczyznę rzutowania (XY, XZ, YZ) traktowaną jako przestrzeń dwuwymiarowa.
/// 💡 Can represent any projection plane (XY, XZ, YZ) treated as a two-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsXy {
	pub x: f64,
	pub y: f64,
}

/// 📚 【 POL】: Współrzędne 3D w układzie kartezjańskim (Przestrzeń XYZ).
/// 📚 【 ENG】: 3D coordinates in the Cartesian system (XYZ Space).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsXyz {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

/// 📚 【 POL】: Współrzędne 2D w układzie biegunowym (Promień, Azymut).
/// 📚 【 ENG】: 2D coordinates in the Polar system (Radius, Azimuth).
/// 💡 Może reprezentować dowolną płaszczyznę rzutowania (XY, XZ, YZ) traktowaną jako przestrzeń dwuwymiarowa.
/// 💡 Can represent any projection plane (XY, XZ, YZ) treated as a two-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsPolar {
	pub r_d2: f64,
	pub f_y_x: f64, // w radianach
}

/// 📚 【 POL】: Współrzędne 3D w układzie cylindrycznym względem osi Z.
/// 📚 【 ENG】: 3D coordinates in the Cylindrical system relative to the Z-axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsCylindricalZ {
	pub r_d2: f64,
	pub f_y_x: f64, // w radianach
	pub z: f64,
}

/// 📚 【 POL】: Współrzędne 3D w układzie cylindrycznym względem osi Y.
/// 📚 【 ENG】: 3D coordinates in the Cylindrical system relative to the Y-axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsCylindricalY {
	pub r_d2: f64,
	pub f_z_x: f64, // w radianach
	pub y: f64,
}

/// 📚 【 POL】: Współrzędne 3D w układzie cylindrycznym względem osi X.
/// 📚 【 ENG】: 3D coordinates in the Cylindrical system relative to the X-axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsCylindricalX {
	pub r_d2: f64,
	pub f_z_y: f64, // w radianach
	pub x: f64,
}

/// 📚 【 POL】: Współrzędne 3D w układzie sferycznym (Promień, Azymut, Inklinacja).
/// 📚 【 ENG】: 3D coordinates in the Spherical system (Radius, Azimuth, Inclination).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsSpherical {
	pub r_d3: f64,
	pub f_y_x: f64, // w radianach (Azymut w płaszczyźnie XY)
	pub t_z_r: f64, // w radianach (Inklinacja od osi Z)
}

// 📚 【 POL】: Współrzędne geograficzne w formacie DMS (Stopnie, Minuty, Sekundy).
/// 📚 【 ENG】: Geographic coordinates in DMS format (Degrees, Minutes, Seconds).
/// 🗺️ Orientacja: Nz90Ex0 (Oś Z: Biegun N, Oś X: Greenwich).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsSphericalEcefSnWeDms {
	pub sn_lat_d: i8,  // Szerokość stopnie (-90 do 90)
	pub sn_lat_m: u8,  // Szerokość minuty (0 do 59)
	pub sn_lat_s: f32, // Szerokość sekundy (0.0 do 59.99999)
	pub we_lon_d: i16, // Długość stopnie (-180 do 180)
	pub we_lon_m: u8,  // Długość minuty (0 do 59)
	pub we_lon_s: f32, // Długość sekundy (0.0 do 59.99999)
	                   //pub alt: f64,    // Wysokość nad poziomem morza (m)
}

/// 📚 【 POL】: Współrzędne geograficzne w formacie DDD (Stopnie Dziesiętne).
/// 📚 【 ENG】: Geographic coordinates in DDD format (Decimal Degrees).
/// 🗺️ Orientacja: Nz90Ex0 (Oś Z: Biegun N, Oś X: Greenwich).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordsSphericalEcefSnWeDdd {
	pub sn_lat: Angle, // Szerokość (-90.0 do 90.0)
	pub we_lon: Angle, // Długość (-180.0 do 180.0)
	                   //pub alt: f64,    // Wysokość nad poziomem morza (m)
}
