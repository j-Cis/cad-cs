// 📃 ./src/libs/cs/abstract_traits.rs

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::libs::{angle::AngleFmt, cs::model::Cs};

/// 📚 【 POL】: Kontrakt gwarantujący implementację podstawowych rzutowań 2D.
/// 📚 【 ENG】: Contract guaranteeing the implementation of basic 2D projections.
pub trait AbstractProjectionsCs2 {
	fn new_from_rf(r: f64, phi_rad: f64) -> Self;
	fn new_as_xy(&self) -> Cs<3>;
	fn new_as_xz(&self) -> Cs<3>;
	fn new_as_yz(&self) -> Cs<3>;
	fn new_as_xy_from_rf(&self) -> Self;
	fn new_as_xyz_from_rf_with_z(&self, z: f64) -> Cs<3>;
	fn new_as_xyz_from_rf_with_y(&self, y: f64) -> Cs<3>;
	fn new_as_xyz_from_rf_with_x(&self, x: f64) -> Cs<3>;
}

/// 📚 【 POL】: Kontrakt gwarantujący implementację podstawowych rzutowań 3D.
/// 📚 【 ENG】: Contract guaranteeing the implementation of basic 3D projections.
pub trait AbstractProjectionsCs3 {
	fn new_from_rft(r: f64, phi_rad: f64, theta_rad: f64) -> Self;
	fn new_from_rfz(r_d2: f64, phi_rad: f64, z: f64) -> Self;
	fn new_from_rfx(r_d2: f64, phi_rad: f64, x: f64) -> Self;
	fn new_from_rfy(r_d2: f64, phi_rad: f64, y: f64) -> Self;
	fn new_as_xyz_from_rft(&self) -> Cs<3>;
}

/// 📚 【 POL】: Kontrakt generyczny dla uniwersalnej matematyki wektorowej (np. dodawanie).
/// 📚 【 ENG】: Generic contract for universal vector mathematics.
pub trait AbstractMathCsGeneric {
	fn sub(&self, other: &Self) -> Self;
	fn add(&self, other: &Self) -> Self;
	fn dot(&self, other: &Self) -> f64;
	fn r_sq(&self) -> f64;
	fn r(&self) -> f64;
	fn normalize_r_projection(&self) -> Self;
	fn angle_between(&self, other: &Self) -> f64;
}

/// 📚 【 POL】: Kontrakt dla operacji matematycznych specyficznych dla 2D.
/// 📚 【 ENG】: Contract for 2D-specific mathematical operations.
pub trait AbstractMathCs2 {
	fn rxy(&self) -> f64;
	fn arctan_y_x(&self) -> f64;
	fn arctan_x_y(&self) -> f64;
	fn to_rf_from_xy(&self) -> Cs<2>;
	fn to_ecef_from_rad_sn_we(&self, r: f64) -> Cs<3>;
	fn q(&self) -> u8;
	fn q_sign(&self) -> [&'static str; 2];
	fn rxy_sq(&self) -> f64;
	fn cross(&self, other: &Cs<2>) -> f64;
	fn perp(&self) -> Cs<2>;
}

/// 📚 【 POL】: Kontrakt dla operacji matematycznych specyficznych dla 3D.
/// 📚 【 ENG】: Contract for 3D-specific mathematical operations.
pub trait AbstractMathCs3 {
	fn rxy(&self) -> f64;
	fn rxz(&self) -> f64;
	fn ryz(&self) -> f64;
	fn rxyz(&self) -> f64;
	fn arctan_y_x(&self) -> f64;
	fn arctan_z_x(&self) -> f64;
	fn arctan_z_y(&self) -> f64;
	fn arctan_x_y(&self) -> f64;
	fn arctan_x_z(&self) -> f64;
	fn arctan_y_z(&self) -> f64;
	fn arccos_x_rxyz(&self) -> f64;
	fn arccos_y_rxyz(&self) -> f64;
	fn arccos_z_rxyz(&self) -> f64;
	fn to_rf_from_xy(&self) -> Cs<2>;
	fn to_rf_from_xz(&self) -> Cs<2>;
	fn to_rf_from_yz(&self) -> Cs<2>;
	fn to_rfx_from_xyz(&self) -> Cs<3>;
	fn to_rfy_from_xyz(&self) -> Cs<3>;
	fn to_rfz_from_xyz(&self) -> Cs<3>;
	fn to_rft_from_xyz(&self) -> Cs<3>;
	fn to_ecef_from_dms_sn_we(
		sn_d: i16,
		sn_m: u8,
		sn_s: f32,
		we_d: i16,
		we_m: u8,
		we_s: f32,
		r: f64,
	) -> Self;
	fn to_dms_sn_we_from_xyz(&self) -> crate::libs::cs::model::CoordsSphericalEcefSnWeDms;
	fn q(&self) -> u8;
	fn q_sign(&self) -> [&'static str; 3];
	fn rxyz_sq(&self) -> f64;
	fn rxy_sq(&self) -> f64;
	fn rxz_sq(&self) -> f64;
	fn ryz_sq(&self) -> f64;
	fn normalize_rxy_projection(&self) -> Cs<3>;
	fn normalize_rxz_projection(&self) -> Cs<3>;
	fn normalize_ryz_projection(&self) -> Cs<3>;
	fn cross(&self, other: &Cs<3>) -> Cs<3>;
}

/// 📚 【 POL】: Kontrakt generyczny dla fundamentalnych operacji modelu Cs<N>.
/// 📚 【 ENG】: Generic contract for fundamental Cs<N> model operations.
pub trait AbstractModelCsGeneric<const N: usize> {
	fn new(data: [f64; N]) -> Self;
	fn origin() -> Self;
	fn as_slice(&self) -> &[f64];
}

/// 📚 【 POL】: Kontrakt dla pomocniczych funkcji diagnostycznych 2D.
/// 📚 【 ENG】: Contract for 2D diagnostic helper functions.
/// 📚 【 POL】: Trait rozszerzający dla Cs<2>, umożliwiający formatowanie danych wyjściowych do konsoli.
/// 📚 【 ENG】: Extension trait for Cs<2>, enabling output formatting to the console.
pub trait AbstractHelperCs2 {
	/// 📚 【 POL】: Wyświetla informację o ćwiartce i znakach składowych.
	/// 📚 【 ENG】: Displays quadrant information and component signs.
	fn print_q(&self, name: &str);

	/// 📚 【 POL】: Wyświetla współrzędne kartezjańskie (X, Y).
	/// 📚 【 ENG】: Displays Cartesian coordinates (X, Y).
	fn print_xy(&self, name: &str);

	/// 📚 【 POL】: Wyświetla współrzędne biegunowe (R, Φ).
	/// 📚 【 ENG】: Displays polar coordinates (R, Φ).
	fn print_rf(&self, name: &str, fmt: AngleFmt);

	/// 📚 【 POL】: Wyświetla zbiorczy raport debugowania dla wektora 2D.
	/// 📚 【 ENG】: Displays a summary debug report for the 2D vector.
	fn print(&self, name: &str, fmt: AngleFmt);
}

/// 📚 【 POL】: Kontrakt dla pomocniczych funkcji diagnostycznych 3D.
/// 📚 【 ENG】: Contract for 3D diagnostic helper functions.
/// 📚 【 POL】: Trait rozszerzający dla Cs<3>, umożliwiający zaawansowane formatowanie rzutów i geodezji.
/// 📚 【 ENG】: Extension trait for Cs<3>, enabling advanced formatting for projections and geodesy.
pub trait AbstractHelperCs3 {
	fn print_q(&self, name: &str);
	fn print_xyz(&self, name: &str);
	fn print_rft(&self, name: &str, fmt: AngleFmt);
	fn print_rfx(&self, name: &str, fmt: AngleFmt);
	fn print_rfy(&self, name: &str, fmt: AngleFmt);
	fn print_rfz(&self, name: &str, fmt: AngleFmt);
	fn print_dms_sn_we(&self, name: &str);
	fn print(&self, name: &str, fmt: AngleFmt);
}

pub trait AbstractArithmeticCsGeneric:
	Add<Output = Self>
	+ Sub<Output = Self>
	+ Neg<Output = Self>
	+ Mul<f64, Output = Self>
	+ Div<f64, Output = Self>
	+ Sized // Zapewnia, że rozmiar jest znany w czasie kompilacji
{
	// Deklarujemy metody, które potem implementujesz w math.rs
	fn add_cs(&self, rhs: &Self) -> Self;
	fn sub_cs(&self, rhs: &Self) -> Self;
	fn neg_cs(&self) -> Self;
	fn mul_scalar(&self, rhs: f64) -> Self;
	fn div_scalar(&self, rhs: f64) -> Self;
}

/// 📚 【 POL】: Trait rozszerzający dla typów liczbowych, ułatwiający prezentację znaków kierunkowych.
/// 📚 【 ENG】: Extension trait for numerical types, facilitating the presentation of directional signs.
pub trait AbstractSignStrExt {
	/// 📚 【 POL】: Zwraca "+" dla wartości nieujemnych oraz "-" dla ujemnych.
	/// 📚 【 ENG】: Returns "+" for non-negative values and "-" for negative ones.
	fn sign_str(self) -> &'static str;

	/// 📚 【 POL】: Zwraca "N" (Północ) dla wartości nieujemnych oraz "S" (Południe) dla ujemnych.
	/// 📚 【 ENG】: Returns "N" (North) for non-negative values and "S" (South) for negative ones.
	fn sign_sn(self) -> &'static str;

	/// 📚 【 POL】: Zwraca "E" (Wschód) dla wartości nieujemnych oraz "W" (Zachód) dla ujemnych.
	/// 📚 【 ENG】: Returns "E" (East) for non-negative values and "W" (West) for negative ones.
	fn sign_we(self) -> &'static str;
}
