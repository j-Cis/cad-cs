// 📃 ./src/libs/cs/math/d3.rs

use crate::libs::{
	cs::{model::Cs, utils::SignStrExt},
	tolerance,
};

impl Cs<3> {
	/// 📚 【 POL】: Długość rzutu (promień) na płaszczyznę XY.
	/// 📚 【 ENG】: Projection length (radius) on the XY plane.
	#[rustfmt::skip] #[inline]	pub fn rxy(&self)  -> f64 { self.0[0].hypot(self.0[1]) }

	/// 📚 【 POL】: Długość rzutu (promień) na płaszczyznę XZ.
	/// 📚 【 ENG】: Projection length (radius) on the XZ plane.
	#[rustfmt::skip] #[inline]	pub fn rxz(&self)  -> f64 { self.0[0].hypot(self.0[2]) }

	/// 📚 【 POL】: Długość rzutu (promień) na płaszczyznę YZ.
	/// 📚 【 ENG】: Projection length (radius) on the YZ plane.
	#[rustfmt::skip] #[inline]	pub fn ryz(&self)  -> f64 { self.0[1].hypot(self.0[2]) }

	/// 📚 【 POL】: Pełna długość wektora w przestrzeni XYZ.
	/// 📚 【 ENG】: Full vector length in XYZ space.
	#[rustfmt::skip] #[inline]	pub fn rxyz(&self) -> f64 { self.0[0].hypot(self.0[1]).hypot(self.0[2]) }

	// --- KĄTY PŁASKIE (AZYMUTY MATEMATYCZNE CCW) ---

	/// 📚 【 POL】: Azymut na płaszczyźnie XY (od X do Y).
	/// 📚 【 ENG】: Azimuth on the XY plane (from X to Y).
	#[rustfmt::skip] #[inline]	pub fn arctan_y_x(&self) -> f64 { self.0[1].atan2(self.0[0]) }

	/// 📚 【 POL】: Azymut na płaszczyźnie XZ (od X do Z).
	/// 📚 【 ENG】: Azimuth on the XZ plane (from X to Z).
	#[rustfmt::skip] #[inline]	pub fn arctan_z_x(&self) -> f64 { self.0[2].atan2(self.0[0]) }

	/// 📚 【 POL】: Azymut na płaszczyźnie YZ (od Y do Z).
	/// 📚 【 ENG】: Azimuth on the YZ plane (from Y to Z).
	#[rustfmt::skip] #[inline]	pub fn arctan_z_y(&self) -> f64 { self.0[2].atan2(self.0[1]) }

	// --- KĄTY KOMPASOWE / MAPOWE (CW, 0° NA OSI PIONOWEJ) ---

	/// 📚 【 POL】: Azymut kompasowy na XY (0° na osi Y).
	/// 📚 【 ENG】: Compass azimuth on XY (0° on Y-axis).
	#[rustfmt::skip] #[inline]	pub fn arctan_x_y(&self) -> f64 { self.0[0].atan2(self.0[1]) }

	/// 📚 【 POL】: Azymut kompasowy na XZ (0° na osi Z).
	/// 📚 【 ENG】: Compass azimuth on XZ (0° on Z-axis).
	#[rustfmt::skip] #[inline]	pub fn arctan_x_z(&self) -> f64 { self.0[0].atan2(self.0[2]) }

	/// 📚 【 POL】: Azymut kompasowy na YZ (0° na osi Z).
	/// 📚 【 ENG】: Compass azimuth on YZ (0° on Z-axis).
	#[rustfmt::skip] #[inline]	pub fn arctan_y_z(&self) -> f64 { self.0[1].atan2(self.0[2]) }

	// --- KĄTY PRZESTRZENNE (INKLINACJA) ---

	/// 📚 【 POL】: Kąt między wektorem a osią X.
	/// 📚 【 ENG】: Angle between the vector and the X-axis.
	#[rustfmt::skip] #[inline]	pub fn arccos_x_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[0] / r).clamp(-1.0, 1.0).acos() } }

	/// 📚 【 POL】: Kąt między wektorem a osią Y.
	/// 📚 【 ENG】: Angle between the vector and the Y-axis.
	#[rustfmt::skip] #[inline]	pub fn arccos_y_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[1] / r).clamp(-1.0, 1.0).acos() } }

	/// 📚 【 POL】: Kąt między wektorem a osią Z (Inklinacja sferyczna).
	/// 📚 【 ENG】: Angle between the vector and the Z-axis (Spherical inclination).
	#[rustfmt::skip] #[inline]	pub fn arccos_z_rxyz(&self) -> f64 { let r = self.rxyz(); if tolerance::is_zero(r) { 0.0 } else { (self.0[2] / r).clamp(-1.0, 1.0).acos() } }

	// --- KONWERSJE 3D -> 2D (RZUTY BIEGUNOWE) ---

	/// 📚 【 POL】: Rzutuje wektor 3D na płaszczyznę XY w formacie biegunowym [R, Φ].
	/// 📚 【 ENG】: Projects a 3D vector onto the XY plane in polar format [R, Φ].
	#[rustfmt::skip] #[inline]	pub fn to_rf_from_xy(&self) -> Cs<2> { Cs([self.rxy(), self.arctan_y_x()]) }

	/// 📚 【 POL】: Rzutuje wektor 3D na płaszczyznę XZ w formacie biegunowym [R, Φ].
	/// 📚 【 ENG】: Projects a 3D vector onto the XZ plane in polar format [R, Φ].
	#[rustfmt::skip] #[inline]	pub fn to_rf_from_xz(&self) -> Cs<2> { Cs([self.rxz(), self.arctan_z_x()]) }

	/// 📚 【 POL】: Rzutuje wektor 3D na płaszczyznę YZ w formacie biegunowym [R, Φ].
	/// 📚 【 ENG】: Projects a 3D vector onto the YZ plane in polar format [R, Φ].
	#[rustfmt::skip] #[inline]	pub fn to_rf_from_yz(&self) -> Cs<2> { Cs([self.ryz(), self.arctan_z_y()]) }

	// --- KONWERSJE 3D -> 3D (UKŁADY CYLINDRYCZNE I SFERYCZNE) ---

	/// 📚 【 POL】: Konwertuje XYZ na układ cylindryczny względem osi X [R_yz, Φ_zy, X].
	/// 📚 【 ENG】: Converts XYZ to a cylindrical system relative to the X-axis [R_yz, Φ_zy, X].
	#[rustfmt::skip] #[inline]	pub fn to_rfx_from_xyz(&self) -> Cs<3> { Cs([self.ryz(),  self.arctan_z_y(), self.0[0]]) }

	/// 📚 【 POL】: Konwertuje XYZ na układ cylindryczny względem osi Y [R_xz, Φ_zx, Y].
	/// 📚 【 ENG】: Converts XYZ to a cylindrical system relative to the Y-axis [R_xz, Φ_zx, Y].
	#[rustfmt::skip] #[inline]	pub fn to_rfy_from_xyz(&self) -> Cs<3> { Cs([self.rxz(),  self.arctan_z_x(), self.0[1]]) }

	/// 📚 【 POL】: Konwertuje XYZ na układ cylindryczny względem osi Z [R_xy, Φ_yx, Z].
	/// 📚 【 ENG】: Converts XYZ to a cylindrical system relative to the Z-axis [R_xy, Φ_yx, Z].
	#[rustfmt::skip] #[inline]	pub fn to_rfz_from_xyz(&self) -> Cs<3> { Cs([self.rxy(),  self.arctan_y_x(), self.0[2]]) }

	/// 📚 【 POL】: Konwertuje XYZ na pełny układ sferyczny [R_xyz, Φ_yx, Θ_zr].
	/// 📚 【 ENG】: Converts XYZ to a full spherical system [R_xyz, Φ_yx, Θ_zr].
	#[rustfmt::skip] #[inline]	pub fn to_rft_from_xyz(&self) -> Cs<3> { Cs([self.rxyz(), self.arctan_y_x(), self.arccos_z_rxyz()]) }

	// --- GEODEZJA ---

	/// 📚 【 POL】: Tworzy wektor ECEF (XYZ) bezpośrednio z danych DMS i promienia.
	/// 📚 【 ENG】: Creates an ECEF vector (XYZ) directly from DMS data and radius.
	#[rustfmt::skip]
	pub fn to_ecef_from_dms_sn_we(
		sn_d: i16, sn_m: u8, sn_s: f32,
		we_d: i16, we_m: u8, we_s: f32,
		r: f64
	) -> Self {
		use crate::libs::angle::Angle;
		let lat_rad = Angle::from_dms(sn_d, sn_m, sn_s).rad();
		let lon_rad = Angle::from_dms(we_d, we_m, we_s).rad();

		let (sin_lat, cos_lat) = lat_rad.sin_cos();
		let (sin_lon, cos_lon) = lon_rad.sin_cos();

		Cs([
			r * cos_lat * cos_lon,
			r * cos_lat * sin_lon,
			r * sin_lat
		])
	}

	/// 📚 【 POL】: Konwertuje wektor XYZ na geodezyjny format DMS (Szerokość, Długość).
	/// 📚 【 ENG】: Converts a XYZ vector to geodetic DMS format (Latitude, Longitude).
	#[rustfmt::skip]
	pub fn to_dms_sn_we_from_xyz(&self) -> crate::libs::cs::model_coords::CoordsSphericalEcefSnWeDms {
		use crate::libs::angle::Angle;
		use crate::libs::tolerance;

		let r = self.rxyz();

		// Zabezpieczenie przed osobliwością w samym jądrze Ziemi (r = 0)
		let (lat_rad, lon_rad) = if tolerance::is_zero(r) {
			(0.0, 0.0)
		} else {
			(
				(self.0[2] / r).clamp(-1.0, 1.0).asin(), // Szerokość z osi Z
				self.0[1].atan2(self.0[0])               // Długość z płaszczyzny XY
			)
		};

		// Rzutujemy radiany na nasz izolator Angle
		let lat_angle = Angle::from_rad(lat_rad);
		let lon_angle = Angle::from_rad(lon_rad);

		// Dekodujemy do składowych (Stopnie, Minuty, Sekundy)
		let (lat_d, lat_m, lat_s) = lat_angle.to_dms();
		let (lon_d, lon_m, lon_s) = lon_angle.to_dms();

		// Pakujemy w DTO
		crate::libs::cs::model_coords::CoordsSphericalEcefSnWeDms {
			sn_lat_d: lat_d as i8,
			sn_lat_m: lat_m,
			sn_lat_s: lat_s,
			we_lon_d: lon_d,
			we_lon_m: lon_m,
			we_lon_s: lon_s,
		}
	}

	// --- ANALIZA PRZESTRZENNA ---

	/// 📚 【 POL】: Zwraca numer oktantu (1-8) w którym znajduje się wektor.
	/// 📚 【 ENG】: Returns the octant number (1-8) where the vector is located.
	#[rustfmt::skip] #[inline]
	pub fn q(&self) -> u8 {
		match (self.0[0] >= 0.0, self.0[1] >= 0.0, self.0[2] >= 0.0) {
			(true, true, true)   => 1, (false, true, true)  => 2, (false, false, true) => 3, (true, false, true)  => 4,
			(true, true, false)  => 5, (false, true, false) => 6, (false, false, false)=> 7, (true, false, false) => 8,
		}
	}

	/// 📚 【 POL】: Zwraca znaki kierunkowe osi XYZ (np. ["+", "-", "+"]).
	/// 📚 【 ENG】: Returns the directional signs of the XYZ axes (e.g., ["+", "-", "+"]).
	#[rustfmt::skip] #[inline]
	pub fn q_sign(&self) -> [&'static str; 3] {
		[self.0[0].sign_str(), self.0[1].sign_str(), self.0[2].sign_str()]
	}

	/// 📚 【 POL】: Kwadrat pełnej długości wektora 3D.
	/// 📚 【 ENG】: Squared full length of the 3D vector.
	#[rustfmt::skip] #[inline]	pub fn rxyz_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2] }

	/// 📚 【 POL】: Kwadrat długości rzutu na płaszczyznę XY.
	/// 📚 【 ENG】: Squared projection length on the XY plane.
	#[rustfmt::skip] #[inline]	pub fn rxy_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[1] * self.0[1] }

	/// 📚 【 POL】: Kwadrat długości rzutu na płaszczyznę XZ.
	/// 📚 【 ENG】: Squared projection length on the XZ plane.
	#[rustfmt::skip] #[inline]	pub fn rxz_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[2] * self.0[2] }

	/// 📚 【 POL】: Kwadrat długości rzutu na płaszczyznę YZ.
	/// 📚 【 ENG】: Squared projection length on the YZ plane.
	#[rustfmt::skip] #[inline]	pub fn ryz_sq(&self) -> f64 { self.0[1] * self.0[1] + self.0[2] * self.0[2] }

	// --- NORMALIZACJE RZUTOWE ---

	/// 📚 【 POL】: Normalizuje rzut XY do długości 1.0, zachowując składową Z.
	/// 📚 【 ENG】: Normalizes the XY projection to length 1.0, preserving the Z component.
	#[rustfmt::skip] #[inline]
	pub fn normalize_rxy_projection(&self) -> Cs<3> {
		let r = self.rxy();
		if tolerance::is_zero(r) { Cs([0.0, 0.0, self.0[2]]) } else { Cs([self.0[0] / r, self.0[1] / r, self.0[2]]) }
	}

	/// 📚 【 POL】: Normalizuje rzut XZ do długości 1.0, zachowując składową Y.
	/// 📚 【 ENG】: Normalizes the XZ projection to length 1.0, preserving the Y component.
	#[rustfmt::skip] #[inline]
	pub fn normalize_rxz_projection(&self) -> Cs<3> {
		let r = self.rxz();
		if tolerance::is_zero(r) { Cs([0.0, self.0[1], 0.0]) } else { Cs([self.0[0] / r, self.0[1], self.0[2] / r]) }
	}

	/// 📚 【 POL】: Normalizuje rzut YZ do długości 1.0, zachowując składową X.
	/// 📚 【 ENG】: Normalizes the YZ projection to length 1.0, preserving the X component.
	#[rustfmt::skip] #[inline]
	pub fn normalize_ryz_projection(&self) -> Cs<3> {
		let r = self.ryz();
		if tolerance::is_zero(r) { Cs([self.0[0], 0.0, 0.0]) } else { Cs([self.0[0], self.0[1] / r, self.0[2] / r]) }
	}

	/// 📚 【 POL】: Iloczyn wektorowy (Cross product). Zwraca wektor ortogonalny.
	/// 📚 【 ENG】: Cross product. Returns an orthogonal vector.
	#[rustfmt::skip] #[inline]
	pub fn cross(&self, other: &Cs<3>) -> Cs<3> {
		Cs([
			self.0[1] * other.0[2] - self.0[2] * other.0[1],
			self.0[2] * other.0[0] - self.0[0] * other.0[2],
			self.0[0] * other.0[1] - self.0[1] * other.0[0]
		])
	}
}
