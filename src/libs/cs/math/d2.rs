// 📃 ./src/libs/cs/math/d2.rs


use crate::libs::cs::{model::Cs, utils::SignStrExt};

impl Cs<2> {
	/// 📚 【 POL】: Długość rzutu (promień) na płaszczyznę XY.
	/// 📚 【 ENG】: Projection length (radius) on the XY plane.
	#[rustfmt::skip] #[inline]	pub fn rxy(&self) -> f64 { self.0[0].hypot(self.0[1]) }

	/// 📚 【 POL】: Azymut matematyczny (od osi X w stronę Y, CCW).
	/// 📚 【 ENG】: Mathematical azimuth (from X-axis towards Y, CCW).
	#[rustfmt::skip] #[inline]	pub fn arctan_y_x(&self) -> f64 { self.0[1].atan2(self.0[0]) }

	/// 📚 【 POL】: Azymut kompasowy/geodezyjny (od osi Y w stronę X, CW).
	/// 📚 【 ENG】: Compass/geodetic azimuth (from Y-axis towards X, CW).
	#[rustfmt::skip] #[inline]	pub fn arctan_x_y(&self) -> f64 { self.0[0].atan2(self.0[1]) }

	/// 📚 【 POL】: Konwertuje wektor kartezjański [X, Y] na wektor biegunowy [R, Φ] w obrębie Cs2.
	/// 📚 【 ENG】: Converts a Cartesian vector [X, Y] to a polar vector [R, Φ] within Cs2.
	#[rustfmt::skip] #[inline]	pub fn to_rf_from_xy(&self) -> Cs<2> { Cs([self.rxy(), self.arctan_y_x()]) }

	/// 📚 【 POL】: Przekształca współrzędne geograficzne [Szerokość, Długość] w radianach na wektor 3D ECEF (X, Y, Z).
	/// 📚 【 ENG】: Transforms geographic coordinates [Latitude, Longitude] in radians to a 3D ECEF vector (X, Y, Z).
	/// ⚙️ 【 POL】: Wykorzystuje zadany promień 'r' (np. promień Ziemi).
	/// ⚙️ 【 ENG】: Uses the specified radius 'r' (e.g., Earth's radius).
	pub fn to_ecef_from_rad_sn_we(&self, r: f64) -> Cs<3> {
		let (sin_lat, cos_lat) = self.0[0].sin_cos();
		let (sin_lon, cos_lon) = self.0[1].sin_cos();

		// Oś X: 0°N, 0°E (Greenwich na równiku)
		// Oś Y: 0°N, 90°E (Ocean Indyjski na równiku)
		// Oś Z: 90°N (Biegun Północny)
		// Oś X: 0°N, 0°E | Oś Y: 0°N, 90°E | Oś Z: 90°N
		Cs([
			r * cos_lat * cos_lon, // X
			r * cos_lat * sin_lon, // Y
			r * sin_lat,           // Z
		])
	}

	/// 📚 【 POL】: Zwraca numer ćwiartki na płaszczyźnie XY (1-4).
	/// 📚 【 ENG】: Returns the quadrant number on the XY plane (1-4).
	#[rustfmt::skip] #[inline]
	pub fn q(&self) -> u8 {
		match (self.0[0] >= 0.0, self.0[1] >= 0.0) {
			(true, true) => 1, (false, true) => 2, (false, false) => 3, (true, false) => 4,
		}
	}

	/// 📚 【 POL】: Zwraca znaki kierunkowe osi X i Y w formie tablicy stringów (np. ["+", "-"]).
	/// 📚 【 ENG】: Returns the directional signs of the X and Y axes as an array of strings (e.g., ["+", "-"]).
	#[rustfmt::skip] #[inline]
	pub fn q_sign(&self) -> [&'static str; 2] { [self.0[0].sign_str(), self.0[1].sign_str()] }

	/// 📚 【 POL】: Kwadrat długości rzutu XY (szybsza alternatywa dla rxy).
	/// 📚 【 ENG】: Squared projection length XY (faster alternative to rxy).
	#[rustfmt::skip] #[inline]	pub fn rxy_sq(&self) -> f64 { self.0[0] * self.0[0] + self.0[1] * self.0[1] }

	/// 📚 【 POL】: Iloczyn wektorowy 2D (wyznacznik). Znak określa orientację (lewo/prawo).
	/// 📚 【 ENG】: 2D cross product (determinant). The sign determines orientation (left/right).
	#[rustfmt::skip] #[inline]
	pub fn cross(&self, other: &Cs<2>) -> f64 { self.0[0] * other.0[1] - self.0[1] * other.0[0] }

	/// 📚 【 POL】: Zwraca wektor prostopadły obrócony o 90° w lewo (CCW).
	/// 📚 【 ENG】: Returns a perpendicular vector rotated 90° to the left (CCW).
	#[rustfmt::skip] #[inline]
	pub fn perp(&self) -> Cs<2> { Cs([-self.0[1], self.0[0]]) }
}
