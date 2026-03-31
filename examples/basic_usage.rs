// examples/basic_usage.rs

// Importujemy makro zdefiniowane na poziomie głównego crate'a
use cad_cs::cs;

// Importujemy struktury z Twojej biblioteki
use cad_cs::libs::cs::{Cs2, Cs3};
use cad_cs::libs::cs::model_coords::{Coords2dXyPolar, Coords3dXyzSpherical};

fn main() {
    println!("🚀 Uruchamiam demonstrację biblioteki cad-cs!\n");

    // =================================================================
    // 1. SZYBKIE TWORZENIE PRZEZ MAKRO
    // =================================================================
    let pt_2d = cs![3.0, 4.0];
    let pt_3d = cs![3.0, 4.0, 12.0];

    println!("🔷 Punkt 2D kartezjański: {:?}", pt_2d);
    println!("🔶 Punkt 3D kartezjański: {:?}", pt_3d);
    
    // Obliczenia na rdzeniu (metody z `math.rs` i `d2/d3.rs`)
    println!("📏 Promień 2D (rxy): {}", pt_2d.rxy());
    println!("📏 Promień 3D pełny (rxyz): {}", pt_3d.rxyz());
    println!("📐 Kąt (azymut) dla 2D: {:.4} rad", pt_2d.arctan_y_x());

    println!("\n----------------------------------------------------\n");

    // =================================================================
    // 2. BEZPIECZNE TWORZENIE PRZEZ DTO (Domain Transfer Objects)
    // =================================================================
    
    // Z układu biegunowego (R, Φ) na kartezjański Cs2
    let punkt_z_biegunowego: Cs2 = Coords2dXyPolar {
        r_xy: 10.0,
        f_yx: std::f64::consts::PI / 2.0, // 90 stopni (prosto w górę osi Y)
    }.into();

    println!("🟪 Cs2 z biegunowego (R=10, Φ=90°): {:?}", punkt_z_biegunowego);

    // Z układu sferycznego (R, Φ, Θ) na kartezjański Cs3
    let punkt_ze_sferycznego: Cs3 = Coords3dXyzSpherical {
        r_xyz: 20.0,
        f_yx: 0.0,                         // Azymut w osi X
        t_zr: std::f64::consts::PI / 4.0,  // Odchylenie 45 stopni od osi Z
    }.into();

    println!("🟫 Cs3 ze sferycznego (R=20, Φ=0°, Θ=45°): {:?}", punkt_ze_sferycznego);
}