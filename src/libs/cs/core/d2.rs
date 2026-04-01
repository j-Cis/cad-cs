// 📃 ./src/libs/cs/core/d2.rs

use crate::libs::cs::{
	abstract_traits::AbstractProjectionsCs2,
	model::{Cs, *},
	types::Cs2,
};

// --- IMPLEMENTACJE FROM (DTO -> Cs2) ---

impl From<CoordsXy> for Cs2 {
	/// 📚 【 POL】: Konwertuje kartezjańskie DTO XY na wektor Cs2.
	/// 📚 【 ENG】: Converts Cartesian XY DTO to a Cs2 vector.
	#[inline]
	fn from(c: CoordsXy) -> Self { Cs([c.x, c.y]) }
}

impl From<CoordsPolar> for Cs2 {
	/// 📚 【 POL】: Konwertuje współrzędne biegunowe XY na wektor kartezjański Cs2.
	/// 📚 【 ENG】: Converts XY polar coordinates to a Cs2 Cartesian vector.
	#[inline]
	fn from(c: CoordsPolar) -> Self {
		let (sin_f, cos_f) = c.f_y_x.sin_cos();
		Cs([c.r_d2 * cos_f, c.r_d2 * sin_f])
	}
}

impl AbstractProjectionsCs2 for Cs<2> {
	/// 📚 【 POL】: Tworzy nowy wektor Cs2 z układu biegunowego (R, Φ) na płaszczyźnie XY.
	/// 📚 【 ENG】: Creates a new Cs2 vector from a polar system (R, Φ) on the XY plane.
	#[rustfmt::skip] #[inline]
	fn new_from_rf(r: f64, phi_rad: f64) -> Self {
		let (sin_phi, cos_phi) = phi_rad.sin_cos();
		Cs([r * cos_phi, r * sin_phi])
	}

	// Projeksje 2D -> 3D
	#[rustfmt::skip] #[inline]	fn new_as_xy(&self) -> Cs<3> { Cs([self.0[0], self.0[1], 0.0]) }
	#[rustfmt::skip] #[inline]	fn new_as_xz(&self) -> Cs<3> { Cs([self.0[0], 0.0, self.0[1]]) }
	#[rustfmt::skip] #[inline]	fn new_as_yz(&self) -> Cs<3> { Cs([0.0, self.0[0], self.0[1]]) }

	// ===================================================================================
	// KREACJA Z UKŁADÓW BIEGUNOWYCH I CYLINDRYCZNYCH (Źródło: R, Phi)
	// ===================================================================================
	// ⚠️ UWAGA DOTYCZĄCA ZAKRESU PONIŻSZYCH METOD:
	// Poniższe metody asymilują ("as"), że wywołujący obiekt `self` NIE JEST
	// wektorem kartezjańskim [X, Y], lecz kontenerem na dane układu biegunowego:
	// self.0[0] = R (Promień)
	// self.0[1] = Φ (Kąt Azymutu w radianach)
	// Przykładowe wywołanie: let punkt = cs![promien, azymut_rad].new_as_xy_from_rf();

	/// 📚 【 POL】: Interpretuje Cs2 jako kontener [R, Φ] i zwraca kartezjański wektor 2D [X, Y].
	/// 📚 【 ENG】: Interprets Cs2 as a [R, Φ] container and returns a 2D Cartesian vector [X, Y].
	#[rustfmt::skip] #[inline]
	fn new_as_xy_from_rf(&self) -> Cs<2> {
		let (sin_phi, cos_phi) = self.0[1].sin_cos();
		Cs([self.0[0] * cos_phi, self.0[0] * sin_phi])
	}

	/// 📚 【 POL】: Interpretuje Cs2 jako [R_xy, Φ] i zwraca kartezjański Cs3 z zadaną wysokością Z.
	/// 📚 【 ENG】: Interprets Cs2 as [R_xy, Φ] and returns a Cartesian Cs3 with a given Z height.
	#[rustfmt::skip] #[inline]
	fn new_as_xyz_from_rf_with_z(&self, z: f64) -> Cs<3> {
		let (sin_phi, cos_phi) = self.0[1].sin_cos();
		Cs([self.0[0] * cos_phi, self.0[0] * sin_phi, z])
	}

	/// 📚 【 POL】: Interpretuje Cs2 jako [R_xz, Φ] i zwraca kartezjański Cs3 z zadaną szerokością Y.
	/// 📚 【 ENG】: Interprets Cs2 as [R_xz, Φ] and returns a Cartesian Cs3 with a given Y width.
	#[rustfmt::skip] #[inline]
	fn new_as_xyz_from_rf_with_y(&self, y: f64) -> Cs<3> {
		let (sin_phi, cos_phi) = self.0[1].sin_cos();
		Cs([self.0[0] * cos_phi, y, self.0[0] * sin_phi])
	}

	/// 📚 【 POL】: Interpretuje Cs2 jako [R_yz, Φ] i zwraca kartezjański Cs3 z zadaną współrzędną X.
	/// 📚 【 ENG】: Interprets Cs2 as [R_yz, Φ] and returns a Cartesian Cs3 with a given X coordinate.
	#[rustfmt::skip] #[inline]
	fn new_as_xyz_from_rf_with_x(&self, x: f64) -> Cs<3> {
		let (sin_phi, cos_phi) = self.0[1].sin_cos();
		Cs([x, self.0[0] * cos_phi, self.0[0] * sin_phi])
	}
}
