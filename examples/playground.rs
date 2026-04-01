// 📃 ./examples/playground.rs

// ===================================================================================
// IMPORTY BIBLIOTEKI CAD-CS (Aktywne i zakomentowane do testów)
// ===================================================================================

// Makra fundamentalne
use cad_cs::cs;
// use cad_cs::{dms, dms_angle, frac, frac_pi};

// Główne typy jądra i traity
// use cad_cs::{Cs, Cs2, Cs3, Dim};

// Typy modeli i układów współrzędnych (DTO)
// use cad_cs::{CoordsXy, CoordsPolar};
// use cad_cs::{CoordsCylindricalZ, CoordsCylindricalY, CoordsCylindricalX};
// use cad_cs::{CoordsXyz, CoordsSpherical, CoordsSphericalEcefSnWeDms};

// Typy pomocnicze z modułu kątów
use cad_cs::libs::angle::AngleFmt;
// use cad_cs::libs::angle::{Angle, AngleExt, deg, rad, pi_frac};

// Typy z modułu ułamków
// use cad_cs::libs::frac::{as_frac, as_frac_pi};

// Traity diagnostyczne (Print Debug)
use cad_cs::libs::cs::helper::{d2::Cs2ConsoleDebug, d3::Cs3ConsoleDebug};

// Trait rozszerzający znaki
// use cad_cs::SignStrExt;

fn main() {
	// 📚 [POL]: Testy wyczerpujące wektorów 2D w poszczególnych ćwiartkach kartezjańskich.
	// 📚 [ENG]: Exhaustive testing of 2D vectors in individual Cartesian quadrants.
	let d2_pt1 = cs![4.327, 7.194];
	let d2_pt2 = cs![4.327, -7.194];
	let d2_pt3 = cs![-4.327, -7.194];
	let d2_pt4 = cs![-4.327, 7.194];

	// 📚 [POL]: Rzutowanie wektorów 2D na poszczególne płaszczyzny 3D.
	// 📚 [ENG]: Projecting 2D vectors onto individual 3D planes.
	let xy_pt1 = cs![4.327, 7.194].new_as_xy();
	let xy_pt2 = cs![4.327, -7.194].new_as_xy();
	let xy_pt3 = cs![-4.327, -7.194].new_as_xy();
	let xy_pt4 = cs![-4.327, 7.194].new_as_xy();

	let xz_pt1 = cs![4.327, 7.194].new_as_xz();
	let xz_pt2 = cs![4.327, -7.194].new_as_xz();
	let xz_pt3 = cs![-4.327, -7.194].new_as_xz();
	let xz_pt4 = cs![-4.327, 7.194].new_as_xz();

	let yz_pt1 = cs![4.327, 7.194].new_as_yz();
	let yz_pt2 = cs![4.327, -7.194].new_as_yz();
	let yz_pt3 = cs![-4.327, -7.194].new_as_yz();
	let yz_pt4 = cs![-4.327, 7.194].new_as_yz();

	// 📚 [POL]: Testy wyczerpujące wektorów 3D we wszystkich 8 oktantach.
	// 📚 [ENG]: Exhaustive testing of 3D vectors in all 8 octants.
	let xyz_pt1 = cs![16.321, 3.438, 8.213];
	let xyz_pt2 = cs![16.321, 3.438, -8.213];
	let xyz_pt3 = cs![16.321, -3.438, 8.213];
	let xyz_pt4 = cs![-16.321, 3.438, 8.213];
	let xyz_pt5 = cs![16.321, -3.438, -8.213];
	let xyz_pt6 = cs![-16.321, -3.438, 8.213];
	let xyz_pt7 = cs![-16.321, 3.438, -8.213];
	let xyz_pt8 = cs![-16.321, -3.438, -8.213];

	// ===================================================================================
	// KREACJA Z UKŁADÓW ALTERNATYWNYCH (SHORTHAND)
	// ===================================================================================

	// 📚 [POL]: System biegunowy (R=15.0, Φ=90°) -> Kartezjański Cs2.
	// 📚 [ENG]: Polar system (R=15.0, Φ=90°) -> Cartesian Cs2.
	let bieg_pt1 = cs![15.0, 0.5 * std::f64::consts::PI].new_as_xy_from_rf();

	// 📚 [POL]: System sferyczny (R=14.0, Φ=45°, Θ=225°) -> Kartezjański Cs3.
	// 📚 [ENG]: Spherical system (R=14.0, Φ=45°, Θ=225°) -> Cartesian Cs3.
	let sfer_pt1 =
		cs![14.0, 0.25 * std::f64::consts::PI, 1.25 * std::f64::consts::PI].new_as_xyz_from_rft();

	// 📚 [POL]: System cylindryczny względem osi X (X=12.0) -> Kartezjański Cs3.
	// 📚 [ENG]: Cylindrical system relative to X-axis (X=12.0) -> Cartesian Cs3.
	let x_cy_pt1 = cs![13.0, 0.5 * std::f64::consts::PI].new_as_xyz_from_rf_with_x(12.0);

	// 📚 [POL]: System cylindryczny względem osi Y (Y=11.0) -> Kartezjański Cs3.
	// 📚 [ENG]: Cylindrical system relative to Y-axis (Y=11.0) -> Cartesian Cs3.
	let y_cy_pt1 = cs![12.0, 0.2 * std::f64::consts::PI].new_as_xyz_from_rf_with_y(11.0);

	// 📚 [POL]: System cylindryczny względem osi Z (Z=14.0) -> Kartezjański Cs3.
	// 📚 [ENG]: Cylindrical system relative to Z-axis (Z=14.0) -> Cartesian Cs3.
	let z_cy_pt1 = cs![16.0, 0.6 * std::f64::consts::PI].new_as_xyz_from_rf_with_z(14.0);

	// ===================================================================================
	// WIZUALIZACJA I RAPORTOWANIE
	// ===================================================================================

	d2_pt1.print("d2_pt1", AngleFmt::Deg);
	d2_pt2.print("d2_pt2", AngleFmt::Deg);
	d2_pt3.print("d2_pt3", AngleFmt::Deg);
	d2_pt4.print("d2_pt4", AngleFmt::Deg);

	xy_pt1.print("xy_pt1", AngleFmt::Deg);
	xy_pt2.print("xy_pt2", AngleFmt::Deg);
	xy_pt3.print("xy_pt3", AngleFmt::Deg);
	xy_pt4.print("xy_pt4", AngleFmt::Deg);

	xz_pt1.print("xz_pt1", AngleFmt::Deg);
	xz_pt2.print("xz_pt2", AngleFmt::Deg);
	xz_pt3.print("xz_pt3", AngleFmt::Deg);
	xz_pt4.print("xz_pt4", AngleFmt::Deg);

	yz_pt1.print("yz_pt1", AngleFmt::Deg);
	yz_pt2.print("yz_pt2", AngleFmt::Deg);
	yz_pt3.print("yz_pt3", AngleFmt::Deg);
	yz_pt4.print("yz_pt4", AngleFmt::Deg);

	xyz_pt1.print("xyz_pt1", AngleFmt::Deg);
	xyz_pt2.print("xyz_pt2", AngleFmt::Deg);
	xyz_pt3.print("xyz_pt3", AngleFmt::Deg);
	xyz_pt4.print("xyz_pt4", AngleFmt::Deg);
	xyz_pt5.print("xyz_pt5", AngleFmt::Deg);
	xyz_pt6.print("xyz_pt6", AngleFmt::Deg);
	xyz_pt7.print("xyz_pt7", AngleFmt::Deg);
	xyz_pt8.print("xyz_pt8", AngleFmt::Deg);

	bieg_pt1.print("bieg_pt1", AngleFmt::Deg);
	sfer_pt1.print("sfer_pt1", AngleFmt::Deg);
	x_cy_pt1.print("x_cy_pt1", AngleFmt::Deg);
	y_cy_pt1.print("y_cy_pt1", AngleFmt::Deg);
	z_cy_pt1.print("z_cy_pt1", AngleFmt::Deg);
}
