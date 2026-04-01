// src/libs/cs/core/d3.rs

use crate::libs::cs::model::{Cs, Cs3};
use crate::libs::cs::model_coords::*;

// --- IMPLEMENTACJE FROM (DTO -> Cs3) ---

impl From<Coords3dXyz> for Cs3 {
	/// 📚 【 POL】: Konwertuje kartezjańskie DTO XYZ na wektor Cs3.
	/// 📚 【 ENG】: Converts Cartesian XYZ DTO to a Cs3 vector.
	#[inline] fn from(c: Coords3dXyz) -> Self { Cs([c.x, c.y, c.z]) }
}

impl From<Coords3dXyCylindricalZ> for Cs3 {
	/// 📚 【 POL】: Konwertuje współrzędne cylindryczne względem osi Z na wektor kartezjański Cs3.
	/// 📚 【 ENG】: Converts cylindrical coordinates relative to the Z-axis to a Cs3 Cartesian vector.
	#[inline]
	fn from(c: Coords3dXyCylindricalZ) -> Self {
		let (sin_f, cos_f) = c.f_yx.sin_cos();
		Cs([c.r_xy * cos_f, c.r_xy * sin_f, c.z])
	}
}

impl From<Coords3dXzCylindricalY> for Cs3 {
	/// 📚 【 POL】: Konwertuje współrzędne cylindryczne względem osi Y na wektor kartezjański Cs3.
	/// 📚 【 ENG】: Converts cylindrical coordinates relative to the Y-axis to a Cs3 Cartesian vector.
	#[inline]
	fn from(c: Coords3dXzCylindricalY) -> Self {
		let (sin_f, cos_f) = c.f_zx.sin_cos();
		Cs([c.r_xz * cos_f, c.y, c.r_xz * sin_f])
	}
}

impl From<Coords3dYzCylindricalX> for Cs3 {
	/// 📚 【 POL】: Konwertuje współrzędne cylindryczne względem osi X na wektor kartezjański Cs3.
	/// 📚 【 ENG】: Converts cylindrical coordinates relative to the X-axis to a Cs3 Cartesian vector.
	#[inline]
	fn from(c: Coords3dYzCylindricalX) -> Self {
		let (sin_f, cos_f) = c.f_zy.sin_cos();
		Cs([c.x, c.r_yz * cos_f, c.r_yz * sin_f])
	}
}

impl From<Coords3dXyzSpherical> for Cs3 {
	/// 📚 【 POL】: Konwertuje współrzędne sferyczne na wektor kartezjański Cs3.
	/// 📚 【 ENG】: Converts spherical coordinates to a Cs3 Cartesian vector.
	#[inline]
	fn from(c: Coords3dXyzSpherical) -> Self {
		let (sin_f, cos_f) = c.f_yx.sin_cos(); // Azymut XY
		let (sin_t, cos_t) = c.t_zr.sin_cos(); // Inklinacja Z do R
		Cs([
			c.r_xyz * sin_t * cos_f,
			c.r_xyz * sin_t * sin_f,
			c.r_xyz * cos_t,
		])
	}
}

impl Cs<3> {
	/// 📚 【 POL】: Tworzy nowy wektor Cs3 z układu sferycznego (R, Φ, Θ).
	/// 📚 【 ENG】: Creates a new Cs3 vector from a spherical system (R, Φ, Θ).
	#[rustfmt::skip] #[inline]
	pub fn new_from_rft(r: f64, phi_rad: f64, theta_rad: f64) -> Self {
		let (sin_phi, cos_phi) = phi_rad.sin_cos();
		let (sin_theta, cos_theta) = theta_rad.sin_cos();
		Cs([r * sin_theta * cos_phi, r * sin_theta * sin_phi, r * cos_theta])
	}

	/// 📚 【 POL】: Interpretuje Cs3 jako kontener sferyczny [R, Φ, Θ] i zwraca wektor kartezjański [X, Y, Z].
	/// 📚 【 ENG】: Interprets Cs3 as a spherical container [R, Φ, Θ] and returns a Cartesian vector [X, Y, Z].
	#[rustfmt::skip] #[inline]
	pub fn new_as_xyz_from_rft(&self) -> Cs<3> {
		let (sin_phi, cos_phi) = self.0[1].sin_cos();
		let (sin_theta, cos_theta) = self.0[2].sin_cos();
		Cs([
			self.0[0] * sin_theta * cos_phi,
			self.0[0] * sin_theta * sin_phi,
			self.0[0] * cos_theta
		])
	}

	/// 📚 【 POL】: Tworzy nowy wektor 3D z układu cylindrycznego względem osi Z: (R_xy, Φ, Z).
	/// 📚 【 ENG】: Creates a new 3D vector from a cylindrical system relative to the Z-axis: (R_xy, Φ, Z).
	#[rustfmt::skip] #[inline]
	pub fn new_from_rfz(r_xy: f64, phi_rad: f64, z: f64) -> Self {
		let (sin_phi, cos_phi) = phi_rad.sin_cos();
		Cs([r_xy * cos_phi, r_xy * sin_phi, z])
	}

	/// 📚 【 POL】: Tworzy nowy wektor 3D z układu cylindrycznego względem osi X: (R_yz, Φ, X).
	/// 📚 【 ENG】: Creates a new 3D vector from a cylindrical system relative to the X-axis: (R_yz, Φ, X).
	/// ⚙️ 【 POL】: Kąt Φ mierzony jest od osi Y do Z na płaszczyźnie YZ.
	/// ⚙️ 【 ENG】: Angle Φ is measured from the Y-axis to the Z-axis on the YZ plane.
	#[rustfmt::skip] #[inline]
	pub fn new_from_rfx(r_yz: f64, phi_rad: f64, x: f64) -> Self {
		let (sin_phi, cos_phi) = phi_rad.sin_cos();
		Cs([x, r_yz * cos_phi, r_yz * sin_phi])
	}

	/// 📚 【 POL】: Tworzy nowy wektor 3D z układu cylindrycznego względem osi Y: (R_xz, Φ, Y).
	/// 📚 【 ENG】: Creates a new 3D vector from a cylindrical system relative to the Y-axis: (R_xz, Φ, Y).
	/// ⚙️ 【 POL】: Kąt Φ mierzony jest od osi X do Z na płaszczyźnie XZ.
	/// ⚙️ 【 ENG】: Angle Φ is measured from the X-axis to the Z-axis on the XZ plane.
	#[rustfmt::skip] #[inline]
	pub fn new_from_rfy(r_xz: f64, phi_rad: f64, y: f64) -> Self {
		let (sin_phi, cos_phi) = phi_rad.sin_cos();
		Cs([r_xz * cos_phi, y, r_xz * sin_phi])
	}
}