// examples/playground.rs

// Importujemy makro zdefiniowane na poziomie głównego crate'a
use cad_cs::cs;

// Importujemy struktury z Twojej biblioteki
// use cad_cs::libs::cs::{Cs2, Cs3};
// use cad_cs::libs::cs::model_coords::{Coords2dXyPolar, Coords3dXyzSpherical};
use cad_cs::libs::cs::debug_print::{Cs2ConsoleDebug, Cs3ConsoleDebug};

// use cad_cs::libs::frac::{as_frac,as_frac_pi};
use cad_cs::libs::angle::AngleFmt; //{Angle, deg, rad, pi_frac, AngleExt, AngleFmt};

fn main() {
    let d2_pt1 = cs![4.327, 7.194];
    let d2_pt2 = cs![4.327, -7.194];
    let d2_pt3 = cs![-4.327, -7.194];
    let d2_pt4 = cs![-4.327, 7.194];

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

    let xyz_pt1 = cs![16.321, 3.438, 8.213];
    let xyz_pt2 = cs![16.321, 3.438, -8.213];
    let xyz_pt3 = cs![16.321, -3.438, 8.213];
    let xyz_pt4 = cs![-16.321, 3.438, 8.213];
    let xyz_pt5 = cs![16.321, -3.438, -8.213];
    let xyz_pt6 = cs![-16.321, -3.438, 8.213];
    let xyz_pt7 = cs![-16.321, 3.438, -8.213];
    let xyz_pt8 = cs![-16.321, -3.438, -8.213];

    // biegunowy (zwraca Cs<2>)
    let bieg_pt1 = cs![15.0, 0.5 * std::f64::consts::PI].new_as_xy_from_rf();

    // sferyczny (zwraca Cs<3>)
    let sfer_pt1 = cs![
        14.0,
        0.25 * std::f64::consts::PI,
        1.25 * std::f64::consts::PI
    ]
    .new_as_xyz_from_rft();

    // cylindryczny względem osi X (zwraca Cs<3>)
    let x_cy_pt1 = cs![13.0, 0.5 * std::f64::consts::PI].new_as_xyz_from_rf_with_x(12.0);

    // cylindryczny względem osi Y (zwraca Cs<3>)
    let y_cy_pt1 = cs![12.0, 0.2 * std::f64::consts::PI].new_as_xyz_from_rf_with_y(11.0);

    // cylindryczny względem osi Z (zwraca Cs<3>)
    let z_cy_pt1 = cs![16.0, 0.6 * std::f64::consts::PI].new_as_xyz_from_rf_with_z(14.0);

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
