// 📃 ./src/libs/consts.rs

/// 📚 【 POL】: Średni promień Ziemi w metrach (model sferyczny R1).
/// 📚 【 ENG】: Mean Earth radius in meters (spherical model R1).
pub const EARTH_MEAN_RADIUS_METERS: f64 = 6_371_000.0;

/// 📚 【 POL】: Przyśpieszenie ziemskie (standardowe) w m/s².
/// 📚 【 ENG】: Standard Earth gravity in m/s².
pub const G_ACCELERATION: f64 = 9.80665;

/// 📚 【 POL】: Tryb promienia używany w obliczeniach łukowych i konwersjach DMS.
/// 📚 【 ENG】: Radius mode used in arc calculations and DMS conversions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadiusMode {
	/// 📚 【 POL】: Średni promień Ziemi (6,371,000 m).
	/// 📚 【 ENG】: Mean Earth radius (6,371,000 m).
	Mean,
	/// 📚 【 POL】: Jedność (1.0) - przydatne do obliczeń znormalizowanych.
	/// 📚 【 ENG】: Unity (1.0) - useful for normalized calculations.
	Unity,
	/// 📚 【 POL】: Własny promień podany w metrach.
	/// 📚 【 ENG】: Custom radius provided in meters.
	Custom(f64),
}

impl RadiusMode {
	/// 📚 【 POL】: Zwraca liczbową wartość promienia dla wybranego trybu.
	/// 📚 【 ENG】: Returns the numerical radius value for the selected mode.
	#[inline]
	pub fn val(&self) -> f64 {
		match self {
			Self::Mean => EARTH_MEAN_RADIUS_METERS,
			Self::Unity => 1.0,
			Self::Custom(r) => *r,
		}
	}
}
